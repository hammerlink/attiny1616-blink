#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use avr_device::{
    attiny416::{PORTB, Peripherals, RTC},
    generic::{Reg, RegisterSpec, Resettable, Writable},
    interrupt::{self, Mutex},
};
use core::cell::RefCell;
use panic_halt as _; // Import panic handler

// Global variables for use in interrupt handlers
static TIMER: Mutex<RefCell<Option<RTC>>> = Mutex::new(RefCell::new(None));
static PORT: Mutex<RefCell<Option<PORTB>>> = Mutex::new(RefCell::new(None));

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

#[avr_device_macros::interrupt(attiny416)]
fn RTC_CNT() {
    avr_device::interrupt::free(|cs| {
        // Access the global variables in a thread-safe manner
        let timer = TIMER.borrow(cs).borrow_mut();
        let port = PORT.borrow(cs).borrow_mut();

        if let (Some(timer), Some(port)) = (timer.as_ref(), port.as_ref()) {
            // Toggle PB5 on RTC overflow
            port.outtgl.write(|w| w.pb5().set_bit());
            timer.intflags.write(|w| w.ovf().set_bit());
        }
    });
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

    //let portb = &dp.PORTB; // Reference to PORTB
    dp.PORTB.dirset.write(|w| w.pb5().set_bit()); // Set PB5 as output

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

    // Store peripherals in global variables for use in interrupt handlers
    interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(dp.RTC)); // Store RTC in global variable
        PORT.borrow(cs).replace(Some(dp.PORTB)); // Store PORTB in global variable
    });

    // Enable global interrupts
    unsafe { interrupt::enable() }

    loop {}
}
