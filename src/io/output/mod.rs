use core::fmt::{self, Write};
use crate::drivers::serial::Serial;

#[derive(Copy, Clone)]
pub struct Console;

impl Console {
    #[inline]
    pub fn write_char(&self, c: char) {
        let mut buf = [0u8; 4];
        for &byte in c.encode_utf8(&mut buf).as_bytes() {
            Serial::write_byte(byte);
        }
    }

    pub fn write_str(&self, str: &str) {
        for byte in str.bytes() {
            Serial::write_byte(byte);
        }
    }

    pub fn write_fmt(&self, args: fmt::Arguments) {
        let mut writer = *self;
        let _ = Write::write_fmt(&mut writer, args);
    }
}

impl Write for Console {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        Console::write_str(self, str);
        Ok(())
    }
}

pub static CONSOLE: Console = Console;