# This is a justfile comment
default:
    @echo "Hello! Run 'just --list' to see available commands"

build:
    cargo build --release
    # Build succeeded

backuphex:
    avrdude -c serialupdi -p t1616 -P /dev/ttyUSB0 -U flash:r:target/backup.hex:i

flashusb:
    avrdude -c serialupdi -p t1616 -P /dev/ttyUSB0 -U flash:w:target/avr-none/release/attiny1616-blink.elf:e

test:
    @echo "Running tests..."
    # Your test commands here
