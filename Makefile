run: build
	mgba-qt test.gba

build:
	cargo build --release
	agb-gbafix target/thumbv4t-none-eabi/release/agb_play -o test.gba

pocket:
	cp test.gba /run/media/tim/pocket/Assets/gba/common/homebrew/test.gba