#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

use core::{arch::asm, ptr};

use avr_device::{
    attiny1616::{PORTA, Peripherals},
    generic::{Reg, RegisterSpec, Resettable, Writable},
    interrupt,
};
use panic_halt as _; // Import panic handler
//
// Define TCB0 base address and offsets
const TCB0_BASE: usize = 0x0A40;
const TCB0_CTRLA: *mut u8 = (TCB0_BASE + 0x00) as *mut u8; // Control A
const TCB0_CTRLB: *mut u8 = (TCB0_BASE + 0x01) as *mut u8; // Control B
const TCB0_INTCTRL: *mut u8 = (TCB0_BASE + 0x06) as *mut u8; // Interrupt Control
const TCB0_CCMP: *mut u16 = (TCB0_BASE + 0x0C) as *mut u16; // CCMP (16-bit access)
//// Optional: Interrupt flags for clearing
const TCB0_INTFLAGS: *mut u8 = (TCB0_BASE + 0x04) as *mut u8;

// BUTTON = PA0

// Unsafe global for PORTA
static mut PORTA_PTR: *mut PORTA = ptr::null_mut();

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

static COLORS: [[u8; 3]; 6] = [
    [0xff, 0xff, 0xff], // GRB WHITE
    [0xff, 0x00, 0x00], // GRB GREEN
    [0x00, 0xff, 0x00], // GRB RED
    [0x00, 0x00, 0xff], // GRB BLUE
    [0x00, 0x55, 0x99], // GRB PURPLE
    [0x00, 0x00, 0x00], // GRB BLACK
];

static mut COLOR_INDEX: usize = 0;

#[avr_device_macros::interrupt(attiny1616)]
fn PORTA_PORT() {
    unsafe {
        send_color_to_rgb_led(&COLORS[COLOR_INDEX], 500);
        COLOR_INDEX = (COLOR_INDEX + 1) % COLORS.len();
        (*PORTA_PTR).intflags.write(|w| w.pa0().set_bit()); // Clear interrupt flag
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap(); // Take ownership of peripherals

    // Initialize the clock
    protected_write(&dp, &dp.SLPCTRL.ctrla, 0);
    protected_write(&dp, &dp.CLKCTRL.osc32kctrla, 0); // RUNSTDBY disabled
    protected_write(&dp, &dp.CLKCTRL.osc20mctrla, 0); // RUNSTDBY disabled
    protected_write(&dp, &dp.CLKCTRL.mclkctrla, 0); // Select 20 MHz clock
    protected_write(&dp, &dp.CLKCTRL.mclkctrlb, 0); // Set Prescaler disabled
    protected_write(&dp, &dp.CLKCTRL.mclklock, 0b1); // LOCK CLOCK

    // Configure PA0 as Button input
    dp.PORTA.dirset.write(|w| w.pa0().clear_bit()); // set PA0 as input
    dp.PORTA.out.write(|w| w.pa0().set_bit()); // set PA0 as pull-up
    dp.PORTA
        .pin0ctrl
        .write(|w| w.isc().falling().pullupen().set_bit()); // Enable falling edge interrupt & configure pull-up as enabled

    // experimental code
    // unsafe {
    //     // Set CCMP to 5 (period = (5+1) * 50 ns = 300 ns)
    //     core::ptr::write_volatile(TCB0_CCMP, 5);
    // }

    // PA7 is used for RGB LED
    dp.PORTA.dirset.write(|w| w.pa7().set_bit());

    unsafe {
        PORTA_PTR = &raw const dp.PORTA as *const _ as *mut _;
    }
    send_color_to_rgb_led(&COLORS[5], 200);
    unsafe {
        interrupt::enable();
    }
    loop {}
}

// bitbang, COLOR iS Green Red Blue
fn send_color_to_rgb_led(color: &[u8; 3], wait_count: u32) {
    let mut is_one_list: [bool; 24] = [false; 24];
    let mut index = 0;

    for byte in color.iter() {
        for bit in (0..8).rev() {
            is_one_list[index] = (byte & (1 << bit)) != 0;

            index = (index + 1) % 24;
        }
    }
    let mut is_one: bool;
    is_one = is_one_list.get(0).unwrap().clone();

    while index < 23 {
        unsafe {
            if is_one {
                // HIGH, 800nS
                (*PORTA_PTR).outset.write(|w| w.pa7().set_bit());
                index = index + 1;
                is_one = is_one_list[index];
                asm!("nop", "nop", "nop", "nop");
                // LOW, 450nS
                (*PORTA_PTR).outclr.write(|w| w.pa7().set_bit());
            } else {
                // HIGH, 400nS
                (*PORTA_PTR).outset.write(|w| w.pa7().set_bit());
                index = index + 1;
                asm!("nop", "nop");
                // LOW, 850nS
                (*PORTA_PTR).outclr.write(|w| w.pa7().set_bit());
                is_one = is_one_list[index];
                asm!("nop", "nop", "nop");
            }
        }
    }
    let mut wait_index: u32 = 0;
    while wait_index < wait_count {
        wait_index += 1;
        unsafe {
            asm!("nop");
        }
    }
}
