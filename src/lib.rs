#![no_std]
#![feature(c_size_t)]

mod bindings;
pub use bindings::{calloc, free, malloc, realloc};
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

pub struct HardenedMalloc;

unsafe impl GlobalAlloc for HardenedMalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size()) as *mut u8 }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { calloc(layout.size(), 1) as *mut u8 }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            free(ptr as *mut c_void);
        }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, size: usize) -> *mut u8 {
        unsafe { realloc(ptr as *mut c_void, size) as *mut u8 }
    }
}
