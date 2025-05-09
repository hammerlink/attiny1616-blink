# This is a justfile comment
default:
    @echo "Hello! Run 'just --list' to see available commands"

build:
    cargo build --release
    # Build succeeded

backuphex:
    avrdude -c serialupdi -p t1616 -P /dev/ttyUSB0 -U flash:r:target/backup.hex:i

backuphex416:
    avrdude -c serialupdi -p t416 -P /dev/ttyACM0 -U flash:r:target/backup416.hex:i

ping_pymu:
    # source ~/venvs/mcuprog/bin/activate 
    pymcuprog ping -u /dev/ttyACM0 -d attiny416

flashusb:
    cargo build --bin at1616-rgb --release
    # avr-objcopy -O ihex target/avr-none/release/at1616-rgb.elf target/1616.hex
    # pymcuprog write --erase -t uart -u /dev/ttyUSB0 -d attiny1616 --clk 115200 --filename ./target/1616.hex
    avrdude -c serialupdi -p t1616 -P /dev/ttyUSB0 -U flash:w:target/avr-none/release/at1616-rgb.elf:e

avrassembly:
    cargo build --bin at1616-rgb --release
    avr-objdump -d target/avr-none/release/at1616-rgb.elf > dissambly-rgb.asm

flashusb416:
    # EXECUTE MANUALLY source ~/venvs/mcuprog/bin/activate 
    cargo build --release
    avr-objcopy -O ihex target/avr-none/release/attiny1616-blink.elf target/416.hex
    pymcuprog write --erase -u /dev/ttyACM0 -d attiny416 --clk 115200 --filename ./target/416.hex

test:
    @echo "Running tests..."
    # Your test commands here
