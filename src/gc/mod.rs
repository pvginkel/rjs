//! The `gc` crates implements the `rjs` garbage collector.
//!
//! The garbage collector implemented in this crate is implemented as a generic
//! garbage collector. However, its functionality is very specifically targeted
//! towards the `rjs` requirements.
//!
//! References to objects managed by the garbage collector need to be rooted.
//! The reason they need to be rooted is because the garbage collector needs to
//! be able to find out what objects are in use at any time.
//!
//! There are two ways in which an object can be rooted. The `Root<T>` struct
//! allows for a persistent reference to an object on the GC heap. Instances of
//! the `Root<T>` struct are valid for the full lifetime of the GC heap.
//! The `Local<T>` struct allows for a transient reference to an object on the
//! GC heap. Instances of the `Local<T>` struct are valid for the lifetime of
//! a `LocalScope`. Once the `LocalScope` is dropped, all `Local<T>` references
//! become invalid.
//!
//! Both `Root<T>` and `Local<T>` have an array variant, respectively
//! `ArrayRoot<T>` and `ArrayLocal<T>`.

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

/// Types references to memory managed by the garbage collector.
#[allow(non_camel_case_types)] 
pub type ptr_t = *const u8;

/// Tracks `Local<T>` references for a specific scope.
///
/// Rooted references to GC objects tracked by `Local<T>` are tracked by an
/// instance of the `LocalScope` struct. Instances of the `Local<T>` struct
/// are valid for the lifetime of a `LocalScope` instance.
pub struct LocalScope {
    heap: *const GcHeap,
    index: usize
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

/// Allows for configuration of a `GcHeap`.
///
/// The default values of the `GcOpts` struct can be retrieved through the
/// `GcOpts::default()` method.
pub struct GcOpts {
    /// Specifies the initial size of the heap in bytes.
    pub initial_heap: usize,
    
    /// Specifies when the garbage collector should initiate a collection.
    ///
    /// The `init_gc` field specifies the maximum fill rate of the GC heap.
    /// If the current fill rate goes beyond this percentage, a collection will
    /// be initiated.
    ///
    /// This percentage is expressed as a value between `0.0` and `1.0`.
    pub init_gc: f64,
    
    /// Specifies the slow growth factor of the GC heap.
    ///
    /// The GC heaps tracks the percentage of the GC heap that was freed the
    /// last time a collection ran. If this percentage is between 50% and 85%,
    /// the heap will be grown by this factor.
    ///
    /// The slow growth factor must be greater than `1.0`.
    pub slow_growth_factor: f64,
    
    /// Specifies the fast growth factor of the GC heap.
    ///
    /// The GC heaps tracks the percentage of the GC heap that was freed the
    /// last time a collection ran. If this percentage is over 85%,
    /// the heap will be grown by this factor.
    ///
    /// The fast growth factor must be greater than `1.0`.
    pub fast_growth_factor: f64
}

impl GcOpts {
    /// Creates an instance of the `GcOpts` struct with a default configuration.
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

// TODO: #90: GcWalker should not be a box but a generic parameter.
// However I'm not sure how to make this work. The problem is that an implementation
// of this trait must be handed to the strategy, and I'm not sure how.

/// Provides a garbage colleced heap.
pub struct GcHeap {
    handles: Rc<RootHandles>,
    heap: RefCell<Copying>,
    scopes: RefCell<Vec<LocalScopeData>>,
    walker: Box<GcWalker>
}

impl GcHeap {
    /// Creates a new instance of the `GcHeap` struct.
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
    
    /// Allocate a raw memory block on the GC heap.
    ///
    /// Memory allocated using the `alloc<T>()` method is not tracked in any way.
    /// To allocated tracked memory, call either `alloc_root<T>()` or
    /// `alloc_local<T>()`.
    pub unsafe fn alloc<T>(&self, ty: u32) -> Ptr<T> {
        let size = (size_of::<T>() + size_of::<usize>() - 1) / size_of::<usize>() * size_of::<usize>();
        
        let ptr = self.alloc_raw(
            size +
            size_of::<GcMemHeader>()
        );
        
        *GcMemHeader::from_ptr(ptr) = GcMemHeader::new(ty, size, false);
        
        Ptr::from_ptr(ptr)
    }
    
    /// Allocate a block of memory on the GC heap tracked by a `Root<T>`.
    pub fn alloc_root<T>(&self, ty: u32) -> Root<T> {
        unsafe { Root::new(self, self.alloc::<T>(ty)) }
    }
    
    /// Allocate a block of memory on the GC heap tracked by a `Local<T>`.
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
    
    /// Allocate an array on the GC heap tracked by a `ArrayRoot<T>`.
    pub fn alloc_array_root<T>(&self, ty: u32, size: usize) -> ArrayRoot<T> {
        unsafe { ArrayRoot::new(self, self.alloc_array::<T>(ty, size)) }
    }
    
    /// Allocate an array on the GC heap tracked by a `ArrayLocal<T>`.
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
    
    /// Allocate a raw array on the GC heap.
    ///
    /// Memory allocated using the `alloc_array<T>()` method is not tracked in any way.
    /// To allocated tracked memory, call either `alloc_array_root<T>()` or
    /// `alloc_array_local<T>()`.
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
    
    /// Initiates a collection.
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
    
    /// Gets the size of the GC heap.
    pub fn mem_allocated(&self) -> usize {
        self.heap.borrow().mem_allocated()
    }
    
    /// Gets how much memory is in use.
    pub fn mem_used(&self) -> usize {
        self.heap.borrow().mem_used()
    }
    
    /// Creates a new `LocalScope` to track `Local<T>` instances.
    ///
    /// To root references to memory managed by the GC heap using a `Local<T>`
    /// struct, a `LocalScope` needs to be present. A local scope is created using
    /// the `new_local_scope()` method.
    ///
    /// `Local<T>` references are valid for the lifetime of the `LocalScope` in
    /// which it is created. Once the `LocalScope` is dropped, all `Local<T>`
    /// references become invalidated.
    ///
    /// Local scopes can be nested. New `Local<T>` instances are created in the
    /// last created local scope.
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

/// The `GcRootWalker` trait allows the garbage collector to track roots.
///
/// GC roots are tracked through `Local<T>` and `Root<T>` references. However,
/// it is also possible to track GC roots outside of the GC heap. An example
/// of this is the stack.
///
/// If GC roots are being tracked outside of the GC heap, a `GcRootWalker`
/// must be provided to the GC heap so that the GC heap can find these roots
/// and update them when the GC heap is copied or compacted.
pub trait GcRootWalker {
    /// Gets the next root.
    ///
    /// The GC heap will repeatedly call the `next()` method until it returns
    /// `ptr::null()`. The returned pointer is a pointer to a pointer to the GC
    /// heap. The GC heap will rewrite this pointer if the referenced object is
    /// relocated.
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

/// The `GcWalker` trait provides integration with the GC heap.
pub trait GcWalker {
    /// Used to mark pointers while marking the GC heap.
    ///
    /// The `walk()` method is called for every word of every memory block
    /// managed by the GC heap. `ty` is the type as provided to the allocation
    /// methods. `ptr` is a pointer to the GC managed object. `index` is the
    /// index of the word that needs to be checked.
    ///
    /// The return `GcWalk` value specifies what type of value the word
    /// represents.
    fn walk(&self, ty: u32, ptr: ptr_t, index: u32) -> GcWalk;
    
    /// Called when a memory block is freed.
    ///
    /// The `finalize()` method allows for finalization to be implemented. Every
    /// time a memory block is freed, the `finalize()` method is called.
    /// `ty` is the type as provided to the allocation methods. `ptr` is a
    /// pointer to the GC managed object.
    fn finalize(&self, ty: u32, ptr: ptr_t) -> GcFinalize;
    
    /// Allows custom root walkers to be provided to the GC heap.
    ///
    /// On every collection, the `create_root_walkers()` method is called to
    /// allow extra `GcRootWalker`'s to be provided to the GC heap.
    ///
    /// These `GcRootWalker`'s can be used to provide extra array roots to the GC
    /// heap.
    fn create_root_walkers(&self) -> Vec<Box<GcRootWalker>>;
}

/// Specifies the type of a word of a GC managed memory block.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GcWalk {
    /// Specifies that the type is a pointer and should be walked when tracing
    /// the GC heap.
    Pointer,
    
    /// Specifies that the type is not a pointer.
    Skip,
    
    /// Specifies that there are no more pointers in the memory object.
    ///
    /// It is not required to return `End` after the last pointer.
    /// However, returning `End` will allow collection to continue with the
    /// next memory object immediately.
    End,
    
    /// Specifies that there are no more elements of an array of the current
    /// memory type will contain a pointer.
    ///
    /// `EndArray` will usually be returned as the only result of memory types
    /// that do not have pointers at all. This allows the GC heap to fully
    /// ignore an array. This applies e.g. to strings.
    EndArray
}

/// Specifies how a memory object was finalized.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GcFinalize {
    /// Specifies that the memory object was finalized.
    ///
    /// Returning `Finalized` does not require any action to have been performed.
    /// Returning `Finalized` specifies that when an array is being freed, the
    /// next array element should be provided to the finalizer.
    Finalized,
    
    /// Specifies that the memory type is not finalizable.
    ///
    /// Returning `NotFinalizable` will stop processing of an array.
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
