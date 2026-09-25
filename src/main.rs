#![no_std]
#![no_main]

mod panic;

// Include i blocchi di avvio (L'ordine è gestito dal linker script memory.ld)
core::arch::global_asm!(include_str!("../image_def.s"));
core::arch::global_asm!(include_str!("../set_stack.s"));

// --- REGISTRI DI RESET ---
const RESETS_BASE: usize = 0x40020000;
const RESETS_RESET: usize = RESETS_BASE + 0x0;
const RESETS_DONE: usize = RESETS_BASE + 0x8;

// --- REGISTRI SIO (Banco 0: Pin 0-31 dell'RP2350) ---
const SIO_BASE: usize = 0xd0000000;
const SIO_GPIO_OUT0_SET: usize = SIO_BASE + 0x010;
const SIO_GPIO_OUT0_CLR: usize = SIO_BASE + 0x014;
const SIO_GPIO_OE0_SET: usize = SIO_BASE + 0x030;

// --- REGISTRI PAD E MUX ---
const PADS_BANK0_BASE: usize = 0x40038000;
const IO_BANK0_BASE: usize = 0x40028000;

#[unsafe(no_mangle)]
pub extern "C" fn _main() -> ! {
    let pin = 6; 
    let bit = 1 << pin;

    unsafe {
        // 1. SVEGLIA LE PERIFERICHE (Rimuovi il reset per IO_BANK0 e PADS_BANK0)
        // Bit 5 = IO_BANK0, Bit 8 = PADS_BANK0
        let reset_mask = (1 << 5) | (1 << 8);
        
        let resets = RESETS_RESET as *mut u32;
        let resets_done = RESETS_DONE as *mut u32;

        // Togli il reset azzerando i bit corrispondenti
        let current_reset = core::ptr::read_volatile(resets);
        core::ptr::write_volatile(resets, current_reset & !reset_mask);

        // Attendi che l'hardware sia pronto
        while core::ptr::read_volatile(resets_done) & reset_mask != reset_mask {
            core::hint::spin_loop();
        }

        // 2. SBLOCCA IL PIN (Rimuovi l'isolamento hardware)
        let pad_reg = (PADS_BANK0_BASE + 4 + (pin * 4)) as *mut u32;
        // Scrivendo 1 << 6 (IE = Input Enable), implicitamente settiamo a 0 
        // il bit 8 (ISO = Isolation), connettendo elettricamente il pin.
        core::ptr::write_volatile(pad_reg, 1 << 6);

        // 3. ASSEGNA IL PIN AL SIO (Muxing)
        let gpio_ctrl = (IO_BANK0_BASE + 4 + (pin * 8)) as *mut u32;
        // Funzione 5 collega il pin al blocco SIO per controllarlo via software
        core::ptr::write_volatile(gpio_ctrl, 5); 

        // 4. ABILITA L'USCITA (Output Enable Set per il Banco 0)
        let oe_set = SIO_GPIO_OE0_SET as *mut u32;
        core::ptr::write_volatile(oe_set, bit);

        // 5. LOOP DI LAMPEGGIO INFINITO
        loop {
            // Accendi il LED scrivendo nel registro SET
            core::ptr::write_volatile(SIO_GPIO_OUT0_SET as *mut u32, bit);
            
            // Ritardo (usiamo asm!("nop") così LLVM in --release non ottimizza via il loop)
            for _ in 0..800_000 {
                core::arch::asm!("nop");
            }

            // Spegni il LED scrivendo nel registro CLEAR
            core::ptr::write_volatile(SIO_GPIO_OUT0_CLR as *mut u32, bit);
            
            for _ in 0..800_000 {
                core::arch::asm!("nop");
            }
        }
    }
}
