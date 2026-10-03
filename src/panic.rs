// TODO: Document

pub use crate::io::output::CONSOLE;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
	let _ = write!(CONSOLE, "[PANIC] Kernel panic! {}\n", info);
	loop {}
}
