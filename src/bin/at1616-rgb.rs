#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

use avr_device::{
    attiny1616::{Peripherals, PORTA, RTC}, generic::{Reg, RegisterSpec, Resettable, Writable}, interrupt::{self, Mutex}
};
use core::{arch::asm, cell::RefCell};
use panic_halt as _; // Import panic handler

// Global variables for use in interrupt handlers
static TIMER: Mutex<RefCell<Option<RTC>>> = Mutex::new(RefCell::new(None));
static PORT: Mutex<RefCell<Option<PORTA>>> = Mutex::new(RefCell::new(None));

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
    // protected_write(&dp, &dp.SLPCTRL.ctrla, 0);
    // protected_write(&dp, &dp.CLKCTRL.osc32kctrla, 0); // RUNSTDBY disabled
    // protected_write(&dp, &dp.CLKCTRL.osc20mctrla, 0); // RUNSTDBY disabled
    // protected_write(&dp, &dp.CLKCTRL.mclkctrlb, 0); // Set Prescaler disabled
    // protected_write(&dp, &dp.CLKCTRL.mclkctrla, 1); // Select 32kHz clock
    // protected_write(&dp, &dp.CLKCTRL.mclklock, 0b1); // LOCK CLOCK

    //let portb = &dp.PORTB; // Reference to PORTB
    dp.PORTA.dirset.write(|w| {
        w.pa0().set_bit();
        w.pa1().set_bit();
        w.pa2().set_bit();
        w.pa3().set_bit();
        w.pa4().set_bit();
        w.pa5().set_bit();
        w.pa6().set_bit();
        w.pa7().set_bit()
    }); // set all ports as output
    dp.PORTB.dirset.write(|w| {
        w.pb0().set_bit();
        w.pb1().set_bit();
        w.pb2().set_bit();
        w.pb3().set_bit();
        w.pb4().set_bit();
        w.pb5().set_bit();
        w.pb6().set_bit();
        w.pb7().set_bit()
    }); // set all ports as output
    dp.PORTC.dirset.write(|w| {
        w.pc0().set_bit();
        w.pc1().set_bit();
        w.pc2().set_bit();
        w.pc3().set_bit();
        w.pc4().set_bit();
        w.pc5().set_bit()
    }); // set all ports as output

    // Configure RTC timer interrupt
    dp.RTC.clksel.write(|w| w.clksel().int1k());
    dp.RTC.per.write(|w| w.bits(512)); // 512 overflow counter to get 500ms
    dp.RTC.cmp.reset(); // Reset compare register
    dp.RTC.intctrl.write(|w| {
        w.cmp().clear_bit();
        w.ovf().set_bit()
    });
    dp.RTC.ctrla.write(|w| {
        w.runstdby().clear_bit();
        w.prescaler().div1();
        w.rtcen().set_bit()
    });

    dp.PORTA.outclr.write(|w| {
        w.pa0().set_bit();
        w.pa1().set_bit();
        w.pa2().set_bit();
        w.pa3().set_bit();
        w.pa4().set_bit();
        w.pa5().set_bit();
        w.pa6().set_bit();
        w.pa7().set_bit()
    }); // set all ports as output
    dp.PORTB.outclr.write(|w| {
        w.pb0().set_bit();
        w.pb1().set_bit();
        w.pb2().set_bit();
        w.pb3().set_bit();
        w.pb4().set_bit();
        w.pb5().set_bit();
        w.pb6().set_bit();
        w.pb7().set_bit()
    }); // set all ports as output
    dp.PORTC.outclr.write(|w| {
        w.pc0().set_bit();
        w.pc1().set_bit();
        w.pc2().set_bit();
        w.pc3().set_bit();
        w.pc4().set_bit();
        w.pc5().set_bit()
    }); // set all ports as output

 // Example: Send green (R=0, G=255, B=0)
    let color = [0x00, 0xFF, 0x00]; // GRB order for NEO_GRB

    loop {
        send_neopixel(&dp.PORTA, &color);
        delay_ms(100); // Simple delay to see the color
    }
}

// Simple delay (tune based on clock speed)
fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..800 { // Adjust for 3.33 MHz clock
            unsafe { asm!("nop") };
        }
    }
}

// Bit-bang the NeoPixel signal on PA7
fn send_neopixel(porta: &avr_device::attiny1616::PORTA, color: &[u8; 3]) {
    for byte in color.iter() {
        for bit in (0..8).rev() {
            let is_one = (byte & (1 << bit)) != 0;
            unsafe {
                if is_one {
                    // 1: ~700ns HIGH, ~600ns LOW
                    porta.outset.write(|w| w.pa7().set_bit()); // HIGH
                    asm!("nop; nop; nop; nop; nop; nop; nop; nop; nop; nop"); // ~500-700ns delay
                    porta.outclr.write(|w| w.pa7().set_bit()); // LOW
                    asm!("nop; nop; nop; nop; nop; nop"); // ~300-600ns delay
                } else {
                    // 0: ~350ns HIGH, ~800ns LOW
                    porta.outset.write(|w| w.pa7().set_bit()); // HIGH
                    asm!("nop; nop; nop"); // ~150-350ns delay
                    porta.outclr.write(|w| w.pa7().set_bit()); // LOW
                    asm!("nop; nop; nop; nop; nop; nop; nop; nop"); // ~400-800ns delay
                }
            }
        }
    }
    // Reset pulse: >50µs LOW (already ensured by loop delay)
}
