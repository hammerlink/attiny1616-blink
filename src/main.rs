#![no_std]
#![no_main]

use panic_halt as _;

#[attiny_hal::entry]
fn main() -> ! {
    loop {
        // Blink logic here (requires GPIO setup)
        let mut x = 1;
        x += 1;
        ()
    }
}
