use core::fmt::{self, Write};
use crate::arch::serial::UART_ADDR;

#[derive(Copy, Clone)]
pub struct Console;

impl Console {
    #[inline(always)]
    pub fn write_byte(&self, byte: u8) {
        let uart = UART_ADDR as *mut u8;
        unsafe {
            uart.write_volatile(byte);
        }
    }

    #[inline]
    pub fn write_char(&self, c: char) {
        let mut buf = [0u8; 4];
        for &byte in c.encode_utf8(&mut buf).as_bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_str(&self, str: &str) {
        for byte in str.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_fmt(&self, args: fmt::Arguments) {
        let mut console = *self;
        let _ = Write::write_fmt(&mut console, args);
    }
}

impl Write for Console {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        Console::write_str(self, str);
        Ok(())
    }
}

pub static CONSOLE: Console = Console;