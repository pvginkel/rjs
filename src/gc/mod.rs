// TODO #56: The handles field of Root currently is a Rc. This is not preferable
// because of performance. However there is a problem. If the field is changed to
// a *const and the Rc is changed to a Box, a segmentation fault will occur.

const INITIAL_LOCAL_SCOPE_CAPACITY : usize = 8;

extern crate libc;
extern crate time;

use std::ops::Index;
use std::ptr;
use std::mem::{size_of, transmute, swap};
use std::cell::RefCell;
use self::strategy::Strategy;
use self::strategy::copying::Copying;
use std::rc::Rc;
pub use self::handles::{ArrayLocal, ArrayRoot, Array, Local, Ptr, Root};
pub use self::handles::{AsPtr, AsArray};

pub mod os;
mod strategy;
pub mod handles;

#[allow(non_camel_case_types)] 
pub type ptr_t = *const u8;

pub struct LocalScope {
    heap: *const GcHeap,
    index: usize
}

impl LocalScope {
    pub fn alloc_local<'s, T>(&'s self, ty: u32) -> Local<'s, T> {
        unimplemented!();
    }
    
    pub fn alloc_array_local<'s, T>(&'s self, ty: u32, size: usize) -> ArrayLocal<'s, T> {
        unimplemented!();
    }
}

impl Drop for LocalScope {
    fn drop(&mut self) {
        tracegc!("dropping scope {}", self.index);
        unsafe { &*self.heap }.drop_current_scope(self.index);
    }
}

struct LocalScopeData {
    current: Vec<ptr_t>,
    handles: Vec<Vec<ptr_t>>
}

impl LocalScopeData {
    fn new() -> LocalScopeData {
        LocalScopeData {
            current: Vec::with_capacity(INITIAL_LOCAL_SCOPE_CAPACITY),
            handles: Vec::new()
        }
    }
    
    fn add(&mut self, ptr: ptr_t) -> *const ptr_t {
        unsafe { assert!(*transmute::<_, *const usize>(ptr) != 0x30252d0); }
        if self.current.len() == self.current.capacity() {
            self.grow();
        }
        
        let index = self.current.len();
        self.current.push(ptr);
        
        unsafe { (*self.current).as_ptr().offset(index as isize) }
    }
    
    fn grow(&mut self) {
        let mut new = Vec::with_capacity(self.current.capacity() * 2);
        swap(&mut new, &mut self.current);
        self.handles.push(new);
    }
}

pub struct GcOpts {
    pub initial_heap: usize,
    pub init_gc: f64,
    pub slow_growth_factor: f64,
    pub fast_growth_factor: f64
}

impl GcOpts {
    pub fn default() -> GcOpts {
        GcOpts {
            initial_heap: 16 * 1024 * 1024, // 16M
            init_gc: 0.95,
            slow_growth_factor: 1.5,
            fast_growth_factor: 3.0
        }
    }
}

struct RootHandles {
    data: RefCell<RootHandlesData>
}

struct RootHandlesData {
    ptrs: Vec<ptr_t>,
    free: Vec<u32>
}

impl RootHandles {
    fn new() -> RootHandles {
        RootHandles {
            data: RefCell::new(RootHandlesData {
                ptrs: Vec::new(),
                free: Vec::new()
            })
        }
    }
    
    fn add(&self, ptr: ptr_t) -> u32 {
        let mut data = self.data.borrow_mut();
        
        let index = if let Some(index) = data.free.pop() {
            assert_eq!(data.ptrs[index as usize], ptr::null());
            
            data.ptrs[index as usize] = ptr;
            index
        } else {
            let index = data.ptrs.len() as u32;
            data.ptrs.push(ptr);
            index
        };
        
        index
    }
    
    fn remove(&self, handle: u32) -> ptr_t {
        let mut data = self.data.borrow_mut();
        
        data.free.push(handle);
        let ptr = data.ptrs[handle as usize];
        data.ptrs[handle as usize] = ptr::null();
        
        ptr
    }
    
    fn clone_root(&self, handle: u32) -> u32 {
        let ptr = self.data.borrow().ptrs[handle as usize];
        self.add(ptr)
    }
    
    unsafe fn get_target(&self, handle: u32) -> ptr_t {
        let data = &*self.data.borrow();
        
        if data.ptrs.len() <= handle as usize {
            panic!("root is not valid anymore");
        }
        
        data.ptrs[handle as usize]
    }
}

struct GcMemHeader {
    header: usize
}

impl GcMemHeader {
    fn new(ty: u32, size: usize, is_array: bool) -> GcMemHeader {
        let mut header =
            (ty & 0x7f) << 1 |
            (size as u32 & 0xffffff) << 8;
        
        if is_array {
            header |= 1;
        }
        
        GcMemHeader {
            header: header as usize
        }
    }
    
    #[inline(always)]
    fn get_type_id(&self) -> u32 {
        (self.header >> 1) as u32 & 0x7f
    }
    
    fn get_size(&self) -> usize {
        self.header >> 8 & 0xffffff
    }
    
    fn is_array(&self) -> bool {
        self.header & 1 != 0
    }
    
    unsafe fn from_ptr<'a>(ptr: ptr_t) -> &'a mut GcMemHeader {
        transmute(ptr.offset(-(size_of::<GcMemHeader>() as isize)))
    }
}

pub struct GcHeap {
    handles: Rc<RootHandles>,
    heap: RefCell<Copying>,
    scopes: RefCell<Vec<LocalScopeData>>,
    walker: Box<GcWalker>
}

impl GcHeap {
    pub fn new(walker: Box<GcWalker>, opts: GcOpts) -> GcHeap {
        if opts.fast_growth_factor <= 1.0 {
            panic!("fast_growth_factor must be more than 1");
        }
        if opts.slow_growth_factor <= 1.0 {
            panic!("slow_growth_factor must be more than 1");
        }
        if opts.init_gc > 1.0 {
            panic!("init_gc must be less than or equal to 1");
        }
        
        GcHeap {
            handles: Rc::new(RootHandles::new()),
            heap: RefCell::new(Copying::new(opts)),
            scopes: RefCell::new(Vec::new()),
            walker: walker
        }
    }
    
    unsafe fn alloc_raw(&self, size: usize) -> ptr_t {
        let mut ptr = self.heap.borrow_mut().alloc_raw(size);
        if ptr.is_null() {
            self.gc();
            
            ptr = self.heap.borrow_mut().alloc_raw(size);
            if ptr.is_null() {
                panic!("could not allocate memory after GC");
            }
        }
        
        if ptr.is_null() {
            ptr
        } else {
            ptr.offset(size_of::<GcMemHeader>() as isize)
        }
    }
    
    pub unsafe fn alloc<T>(&self, ty: u32) -> Ptr<T> {
        let size = (size_of::<T>() + size_of::<usize>() - 1) / size_of::<usize>() * size_of::<usize>();
        
        let ptr = self.alloc_raw(
            size +
            size_of::<GcMemHeader>()
        );
        
        *GcMemHeader::from_ptr(ptr) = GcMemHeader::new(ty, size, false);
        
        Ptr::from_ptr(ptr)
    }
    
    pub fn alloc_root<T>(&self, ty: u32) -> Root<T> {
        unsafe { Root::new(self, self.alloc::<T>(ty)) }
    }
    
    pub fn alloc_local<T>(&self, ty: u32) -> Local<T> {
        self.alloc_local_from_ptr(unsafe { self.alloc::<T>(ty) })
    }
    
    fn alloc_local_from_any_ptr<T, U: AsPtr<T>>(&self, ptr: U) -> Local<T> {
        let mut scopes = self.scopes.borrow_mut();
        let len = scopes.len();
        if len == 0 {
            panic!("no local scope present");
        }
        
        tracegc!("registering local for {:?} with scope {}", ptr.as_ptr().ptr(), len - 1);
        
        unsafe { Local::new(transmute(scopes[len - 1].add(ptr.as_ptr().ptr()))) }
    }
    
    pub fn alloc_array_root<T>(&self, ty: u32, size: usize) -> ArrayRoot<T> {
        unsafe { ArrayRoot::new(self, self.alloc_array::<T>(ty, size)) }
    }
    
    pub fn alloc_array_local<T>(&self, ty: u32, size: usize) -> ArrayLocal<T> {
        self.alloc_array_local_from_ptr(unsafe { self.alloc_array::<T>(ty, size) })
    }
    
    fn alloc_array_local_from_ptr<T, U: AsArray<T>>(&self, ptr: U) -> ArrayLocal<T> {
        let mut scopes = self.scopes.borrow_mut();
        let len = scopes.len();
        if len == 0 {
            panic!("no local scope present");
        }
        
        unsafe { ArrayLocal::new(transmute(scopes[len - 1].add(ptr.as_ptr().ptr()))) }
    }
    
    pub unsafe fn alloc_array<T>(&self, ty: u32, size: usize) -> Array<T> {
        let item_size = (size_of::<T>() + size_of::<usize>() - 1) / size_of::<usize>() * size_of::<usize>();
        
        let ptr = self.alloc_raw(
            size_of::<usize>() +
            (item_size * size) +
            size_of::<GcMemHeader>()
        );
        
        *GcMemHeader::from_ptr(ptr) = GcMemHeader::new(ty, item_size, true);
        *transmute::<_, *mut usize>(ptr) = size;
        
        Array::from_ptr(ptr)
    }
    
    pub fn gc(&self) {
        let mut walkers = self.walker.create_root_walkers();
        
        // Add the root handles walker if there are root handles.
        
        let mut handles = self.handles.data.borrow_mut();
        if handles.ptrs.len() != handles.free.len() {
            let ptr = (*handles.ptrs).as_mut_ptr();
            let end = unsafe { ptr.offset(handles.ptrs.len() as isize) };
            
            walkers.push(Box::new(RootHandlesWalker {
                ptr: ptr,
                end: end
            }));
        }
        
        // Add the local scopes walker if there are any.
        
        let scopes = self.scopes.borrow();
        if scopes.len() > 0 {
            walkers.push(Box::new(LocalScopesWalker {
                scopes: unsafe { transmute::<&[LocalScopeData], *const [LocalScopeData]>(&**scopes) },
                scope: 0,
                vec: 0,
                index: 0
            }));
        }
        
        self.heap.borrow_mut().gc(walkers, &*self.walker);
    }
    
    pub fn mem_allocated(&self) -> usize {
        self.heap.borrow().mem_allocated()
    }
    
    pub fn mem_used(&self) -> usize {
        self.heap.borrow().mem_used()
    }
    
    pub fn new_local_scope(&self) -> LocalScope {
        let mut scopes = self.scopes.borrow_mut();
        
        let index = scopes.len();
        scopes.push(LocalScopeData::new());
        
        tracegc!("creating scope {}", index);
        
        LocalScope {
            heap: self as *const GcHeap,
            index: index
        }
    }
    
    fn drop_current_scope(&self, index: usize) {
        let mut scopes = self.scopes.borrow_mut();
        
        if scopes.len() != index + 1 {
            panic!("local scopes must be destoryed in the order they are created");
        }
        
        scopes.pop();
    }
}

pub trait GcRootWalker {
    unsafe fn next(&mut self) -> *mut ptr_t;
}

struct RootHandlesWalker {
    ptr: *mut ptr_t,
    end: *mut ptr_t
}

impl GcRootWalker for RootHandlesWalker {
    unsafe fn next(&mut self) -> *mut ptr_t {
        while self.ptr < self.end {
            let ptr = self.ptr;
            self.ptr = self.ptr.offset(1);
            
            if !(*ptr).is_null() {
                tracegc!("root walker returning {:?}", *ptr);
                
                return ptr;
            }
        }
        
        ptr::null_mut()
    }
}

struct LocalScopesWalker {
    // TODO #57: This does not have to be a pointer. The only reason it is
    // is to remove the lifetime parameter because I cannot figure
    // out how to get it working with a lifetime.
    scopes: *const [LocalScopeData],
    scope: usize,
    vec: usize,
    index: usize
}

impl GcRootWalker for LocalScopesWalker {
    unsafe fn next(&mut self) -> *mut ptr_t {
        let scopes = transmute::<_, &[LocalScopeData]>(self.scopes);
        
        loop {
            // If we're at the last scope, quit.
            
            if self.scope == scopes.len() {
                return ptr::null_mut();
            }
            
            assert!(self.scope < scopes.len());
            
            let scope = &scopes[self.scope];
            
            // If this vec is the last vec of this scope, reset vec and try again.
            
            if self.vec > scope.handles.len() {
                self.vec = 0;
                self.scope += 1;
                continue;
            }
            
            let vec = if self.vec == 0 {
                &scope.current
            } else {
                &scope.handles[self.vec - 1]
            };
            
            // If we're at the end of the current vec, go to the next vec and try again.
            
            if self.index >= vec.len() {
                self.index = 0;
                self.vec += 1;
                continue;
            }
            
            let ptr = (*vec).as_ptr().offset(self.index as isize) as *mut ptr_t;
            
            self.index += 1;
            
            return ptr;
        }
    }
}

pub trait GcWalker {
    fn walk(&self, ty: u32, ptr: ptr_t, index: u32) -> GcWalk;
    
    fn finalize(&self, ty: u32, ptr: ptr_t) -> GcFinalize;
    
    fn create_root_walkers(&self) -> Vec<Box<GcRootWalker>>;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GcWalk {
    Pointer,
    Skip,
    End,
    EndArray
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GcFinalize {
    Finalized,
    NotFinalizable
}

pub trait GcAllocator {
    fn alloc_array_local_from_root<T>(&self, root: &ArrayRoot<T>) -> ArrayLocal<T>;
    
    fn alloc_array_root_from_local<T>(&self, local: ArrayLocal<T>) -> ArrayRoot<T>;
    
    fn alloc_array_local_from_array<T>(&self, array: Array<T>) -> ArrayLocal<T>;
    
    fn alloc_root_from_local<T>(&self, local: Local<T>) -> Root<T>;
    
    fn alloc_local_from_ptr<T>(&self, ptr: Ptr<T>) -> Local<T>;
    
    fn alloc_local_from_root<T>(&self, root: &Root<T>) -> Local<T>;
}

impl GcAllocator for GcHeap {
    fn alloc_array_local_from_root<T>(&self, root: &ArrayRoot<T>) -> ArrayLocal<T> {
        self.alloc_array_local_from_ptr(root.as_ptr())
    }
    
    fn alloc_array_root_from_local<T>(&self, local: ArrayLocal<T>) -> ArrayRoot<T> {
        unsafe { ArrayRoot::new(self, local) }
    }
    
    fn alloc_array_local_from_array<T>(&self, array: Array<T>) -> ArrayLocal<T> {
        self.alloc_array_local_from_ptr(array)
    }
    
    fn alloc_root_from_local<T>(&self, local: Local<T>) -> Root<T> {
        unsafe { Root::new(self, local) }
    }
    
    fn alloc_local_from_ptr<T>(&self, ptr: Ptr<T>) -> Local<T> {
        self.alloc_local_from_any_ptr(ptr)
    }
    
    fn alloc_local_from_root<T>(&self, root: &Root<T>) -> Local<T> {
        self.alloc_local_from_any_ptr(root.as_ptr())
    }
}
