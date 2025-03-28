#![no_std]
#![no_main]

use avr_device::attiny416::Peripherals;
use panic_halt as _; // Import panic handler

// Define clock frequency 
const F_CPU: u32 = 10_000_000;

// Simple delay function (not precise, for demo purposes)
fn delay_ms(ms: u32) {
    // Rough approximation: adjust based on your clock speed
    let cycles = (F_CPU / 1_000) * ms / 50; // ~50 cycles per loop
    let mut bool: bool = true;
    for _ in 0..cycles {
        // Use a volatile write to prevent optimization
        unsafe { core::ptr::write_volatile(&mut bool, true); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap(); // Take ownership of peripherals
    let portb = &dp.PORTB; // Reference to PORTB

    portb.dirset.write(|w| w.pb5().set_bit()); // Set PB5 as output

    loop {
        portb.outtgl.write(|w| w.pb5().set_bit()); // Toggle PB5
        delay_ms(500); // Delay 500ms
    }
}
