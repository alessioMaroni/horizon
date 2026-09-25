// Copyright (c) 2026 Horizon Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(target_arch = "x86_64")]
	crate::println!("[Kernel Panics] Panic: {:?}", info);
	loop {}
}
