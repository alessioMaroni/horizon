//! Memory Management (MM) Subsystem Module
//!
//! Provides memory allocation primitives for the kernel, including a physical frame/buddy
//! allocator for general kernel dynamics and a lightweight bump allocator for early boot stages.

// TODO: Uncoment after fixing buddy allocator
// pub mod buddy;

/// Example Usage of the Early Bump Allocator:
/// ```rust
/// // Direct manual allocation via BumpAllocator raw interface
/// let layout = core::alloc::Layout::from_size_align(1024, 8).unwrap();
/// let ptr = unsafe { BUMP_ALLOCATOR.alloc(layout) };
///
/// if !ptr.is_null() {
///     // Access and write to raw allocated memory safely...
/// }
/// ```
pub mod bump;

// use crate::mm::buddy::LockedBuddyAllocator;
use crate::mm::bump::BumpAllocator;

/// Primary global kernel heap allocator.
///
/// Handles all dynamic allocations required by standard kernel data structures
/// (`alloc::boxed::Box`, `alloc::vec::Vec`, `alloc::string::String`, etc.).
/// Synchronized via interior locking mechanism to ensure thread safety across CPU cores.
#[global_allocator]
// pub static ALLOCATOR: LockedBuddyAllocator = LockedBuddyAllocator::new();
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();
