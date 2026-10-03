// TODO: Document

pub use crate::CONSOLE;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
	let _ = write!(CONSOLE, "[PANIC] Kernel panic! {}\n", info);
	loop {}
}
