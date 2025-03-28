#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use avr_device::attiny416::Peripherals; // Import peripherals for ATmega328P
use panic_halt as _; // Import panic handler

// Define clock frequency 
const F_CPU: u32 = 20_000_000;

// Simple delay function (not precise, for demo purposes)
fn delay_ms(ms: u16) {
    // Rough approximation: adjust based on your clock speed
    let cycles = (F_CPU / 1_000) * (ms as u32) / 3; // ~3 cycles per loop
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop") }; // No-op instruction
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap(); // Take ownership of peripherals
    let portb = &dp.PORTB; // Reference to PORTB

    portb.dirset.write(|w| w.pb5().set_bit()); // Set PB5 as output

    loop {
        portb.outtgl.write(|w| w.pb5().set_bit()); // Toggle PB5
        delay_ms(50); // Delay 500ms
    }
}
