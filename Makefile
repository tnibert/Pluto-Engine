all:
	cargo build --release
	agb-gbafix target/thumbv4t-none-eabi/release/agb_play -o test.gba
	mgba-qt test.gba