pub struct Serial;

use crate::arch::serial::UART_ADDR;

impl Serial {
    #[inline(always)]
    pub fn write_byte(b: u8) {
        let uart: *mut u8 = UART_ADDR as *mut u8;
        unsafe {
            uart.write_volatile(b);
        }
    }
}