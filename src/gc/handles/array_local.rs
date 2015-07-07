use gc::{Array, AsArray, ArrayRoot, GcAllocator};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub struct ArrayLocal<'s, T> {
    handle: *const Array<T>,
    _type: PhantomData<&'s ()>
}

impl<'s, T> ArrayLocal<'s, T> {
    pub unsafe fn new(handle: *const Array<T>) -> ArrayLocal<'s, T> {
        ArrayLocal {
            handle: handle
        }
    }
    
    pub fn as_root<U: GcAllocator>(&self, allocator: &U) -> ArrayRoot<T> {
        allocator.alloc_array_root_from_local(*self)
    }
}

impl<'s, T> Copy for ArrayLocal<'s, T> { }

impl<'s, T> Clone for ArrayLocal<'s, T> {
    fn clone(&'s self) -> ArrayLocal<'s, T> {
        ArrayLocal {
            handle: self.handle
        }
    }
}

impl<'s, T> Deref for ArrayLocal<'s, T> {
    type Target = [T];
    
    fn deref(&self) -> &[T] {
        unsafe { &**self.handle }
    }
}

impl<'s, T> DerefMut for ArrayLocal<'s, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { &mut **(self.handle as *mut Array<T>) }
    }
}

impl<'s, T> AsArray<T> for ArrayLocal<'s, T> {
    fn as_ptr(&self) -> Array<T> {
        unsafe { *self.handle }
    }
}
