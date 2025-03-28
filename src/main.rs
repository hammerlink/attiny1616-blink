#![no_std]
#![no_main]

use avr_device::{
    attiny416::Peripherals,
    generic::{Reg, RegisterSpec, Resettable, Writable},
};
use panic_halt as _; // Import panic handler

// Define clock frequency
const F_CPU: u32 = 20_000_000;
// Simple delay function (not precise, for demo purposes)
fn delay_ms(ms: u32) {
    // Rough approximation: adjust based on your clock speed
    let cycles = (F_CPU / 1_000) * ms / 10; // ~ cycles per loop
    let mut bool: bool = true;
    for _ in 0..cycles {
        // Use a volatile write to prevent optimization
        unsafe {
            core::ptr::write_volatile(&mut bool, true);
        }
    }
}

fn protected_write<T>(dp: &Peripherals, reg: &Reg<T>, value: u8)
where
    T: Writable + Resettable,
    T: RegisterSpec<Ux = u8>,
{
    // Write the CCP signature (0xD8) to enable protected register writes
    dp.CPU.ccp.write(|w| unsafe { w.bits(0xD8) });

    // Write the value to the register
    reg.write(|w| unsafe { w.bits(value) });
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap(); // Take ownership of peripherals

    // Initialize the clock
    protected_write(&dp, &dp.SLPCTRL.ctrla, 0);
    protected_write(&dp, &dp.CLKCTRL.osc32kctrla, 0); // RUNSTDBY disabled
    protected_write(&dp, &dp.CLKCTRL.osc20mctrla, 0); // RUNSTDBY disabled
    protected_write(&dp, &dp.CLKCTRL.mclkctrlb, 0); // Set Prescaler disabled
    protected_write(&dp, &dp.CLKCTRL.mclkctrla, 1); // Select 32kHz clock
    protected_write(&dp, &dp.CLKCTRL.mclklock, 0b1); // LOCK CLOCK

    let portb = &dp.PORTB; // Reference to PORTB
    portb.dirset.write(|w| w.pb5().set_bit()); // Set PB5 as output

    loop {
        portb.outtgl.write(|w| w.pb5().set_bit()); // Toggle PB5
        delay_ms(5); // Delay 500ms
    }
}
