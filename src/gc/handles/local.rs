use gc::{Ptr, Root, AsPtr, GcAllocator};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub struct Local<'s, T> {
    handle: *const Ptr<T>,
    _type: PhantomData<&'s ()>
}

impl<'s, T> Local<'s, T> {
    pub unsafe fn new(handle: *const Ptr<T>) -> Local<'s, T> {
        Local {
            handle: handle
        }
    }
    
    pub fn as_root<U: GcAllocator>(&self, allocator: &U) -> Root<T> {
        allocator.alloc_root_from_local(*self)
    }
}

impl<'s, T> Copy for Local<'s, T> {}

impl<'s, T> Clone for Local<'s, T> {
    fn clone(&'s self) -> Local<'s, T> {
        Local {
            handle: self.handle
        }
    }
}

impl<'s, T> Deref for Local<'s, T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        unsafe { &**self.handle }
    }
}

impl<'s, T> DerefMut for Local<'s, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut **(self.handle as *mut Ptr<T>) }
    }
}

impl<'s, T> AsPtr<T> for Local<'s, T> {
    fn as_ptr(&self) -> Ptr<T> {
        unsafe { *self.handle }
    }
}
