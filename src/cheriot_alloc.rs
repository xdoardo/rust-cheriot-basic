use crate::rtos_utils::{cheriot_alloc, cheriot_free};

/// An allocator based on the CHERIoT RTOS allocator.
struct CHERIoTRTOSAllocator;

unsafe impl alloc::alloc::GlobalAlloc for CHERIoTRTOSAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { cheriot_alloc(layout.size() as _) as _ }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { cheriot_free(ptr as _) }
    }
}

#[global_allocator]
static CHERIOT_RTOS_ALLOCATOR: CHERIoTRTOSAllocator = CHERIoTRTOSAllocator;
