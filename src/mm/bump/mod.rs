//! # bump
//!     Contain bum struct definition

pub mod global_impl;
pub mod helpers;
pub mod implementation;

use core::cell::UnsafeCell;

/// Bump Allocator Struct Definition
/// A sequential linear allocator (Bump Allocator) designed for early kernel initialization.
///
/// # Thread Safety
///
/// Uses [`UnsafeCell`] to provide interior mutability, allowing updates via shared
/// references (`&self`). Operations must be synchronized externally if accessed from
/// multiple CPU cores or execution threads.
pub struct BumpAllocator {
	/// The current memory address boundary for the next allocation request.
	///
	/// Moves forward (bumped) as allocations occur toward `heap_end`.
	pub next: UnsafeCell<usize>,

	/// The base starting physical/virtual address of the heap memory range.
	pub heap_start: UnsafeCell<usize>,

	/// The maximum allowable physical/virtual address limit of the heap memory range.
	pub heap_end: UnsafeCell<usize>,
}
