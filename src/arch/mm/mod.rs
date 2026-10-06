// TODO: Document

pub mod stack;

unsafe extern "C" {
	pub static _heap_start: u8;
	pub static _heap_end: u8;
}