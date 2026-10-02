// TODO: Document

use crate::arch::serial::_uart_start;

pub struct Serial;

impl Serial {
	#[inline(always)]
	pub fn write_byte(b: u8) {
		let uart: *mut u8 = core::ptr::addr_of!(_uart_start) as *mut u8;
		unsafe {
			uart.write_volatile(b);
		}
	}
}
