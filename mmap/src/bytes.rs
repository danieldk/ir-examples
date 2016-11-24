use std::mem;
use std::slice;

pub unsafe fn typed_to_bytes<T>(slice: &[T]) -> &[u8] {
    slice::from_raw_parts(slice.as_ptr() as *const u8,
                          slice.len() * mem::size_of::<T>())
}

pub unsafe fn bytes_to_typed<T>(slice: &[u8]) -> &[T] {
    slice::from_raw_parts(slice.as_ptr() as *const T,
                          slice.len() / mem::size_of::<T>())
}
