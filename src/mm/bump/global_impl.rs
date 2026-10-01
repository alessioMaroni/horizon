//! Global Implementation of the Bump Allocator

use crate::mm::bump::BumpAllocator;
use core::alloc::{GlobalAlloc, Layout};

unsafe impl Sync for BumpAllocator {}
unsafe impl GlobalAlloc for BumpAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { self.alloc(layout) }
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe { self.dealloc(ptr, layout) }
	}
}
