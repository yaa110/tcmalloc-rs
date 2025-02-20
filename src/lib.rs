use std::alloc::{GlobalAlloc, Layout};

mod bindings;

pub struct TcMalloc;

unsafe impl GlobalAlloc for TcMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { bindings::tc_malloc(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { bindings::tc_free(ptr as *mut _) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { bindings::tc_calloc(layout.size(), 1) as *mut u8 }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { bindings::tc_realloc(ptr as *mut _, new_size) as *mut u8 }
    }
}
