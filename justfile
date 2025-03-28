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
    avrdude -c serialupdi -p t1616 -P /dev/ttyUSB0 -U flash:w:target/avr-none/release/attiny1616-blink.elf:e

flashusb416:
    # EXECUTE MANUALLY source ~/venvs/mcuprog/bin/activate 
    cargo build --release
    pymcuprog write --erase -u /dev/ttyACM0 -d attiny416 --clk 115200 --filename ./target/backup.hex

test:
    @echo "Running tests..."
    # Your test commands here
