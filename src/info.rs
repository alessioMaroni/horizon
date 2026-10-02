// TODO: Document

use crate::arch::mm::{_heap_start, _heap_end};

pub struct Info {
	pub heap_start: usize,
	pub heap_end: usize,
}

impl Info {
	pub fn init() -> Self {
		let hs = unsafe { &_heap_start as *const u8 as usize };
		let he = unsafe { &_heap_end as *const u8 as usize };

		Self {
			heap_start: hs,
			heap_end: he,
		}
	}
}
