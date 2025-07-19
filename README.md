# Pluto Engine

## Project

This is a game engine demo written in Rust for the Gameboy Advance.

## Building

### Prerequisites

You will need the following installed in order to build and run this project:

* A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing
rust with the following: `cargo install agb-gbafix`.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
make build
```

The resulting file will be named `test.gba` in current working directory.  The file will be prepared to run on real hardware.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
make run
```

## Starting development

You can find the documentation for agb [here](https://docs.rs/agb/latest/agb/).

## Asset Credit

- sfx/bgm.xm - Bionic Girl by Drozerix - public domain asset retrieved from https://modarchive.org/module.php?174416.
- fonts/NESCyrillic.ttf - NES Cyrillic by xbost - public domain asset retrieved from https://www.pentacom.jp/pentacom/bitfontmaker2/gallery/?id=234.
- all other assets are from AGB official tutorials