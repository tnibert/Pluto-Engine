build:
	cargo build --release
	agb-gbafix target/thumbv4t-none-eabi/release/pluto_engine -o test.gba

run: build
	mgba-qt test.gba

# Load the rom onto the Analogue Pocket
MNTPATH=/run/media/tim/pocket
pocket:
	mkdir -p $(MNTPATH)/Assets/gba/common/homebrew
	cp test.gba $(MNTPATH)/Assets/gba/common/homebrew/test.gba