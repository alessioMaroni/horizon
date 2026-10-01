//! Bump Allocator helper functions

use crate::mm::bump::BumpAllocator;

impl BumpAllocator {
	/// # Helper function
	/// Resets the RAM pointer back to the start of the heap.
	///
	/// # SAFETY
	/// Calling this means every subsequent allocation will overwrite the previously allocated RAM.
	pub fn reset_ram(&self) {
		unsafe {
			*self.next.get() = *self.heap_start.get();
		}
	}
}

/// Helper function to align an address upwards to the nearest multiple of `align`.
///
/// `align` must be a power of two (e.g., 4, 8, 16, 4096).
#[inline]
pub fn align_up(addr: usize, align: usize) -> usize {
	(addr + align - 1) & !(align - 1)
}
