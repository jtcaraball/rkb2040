# RKB2040

An extensible keyboard firmware for rp2040 based boards written in rust.
Although I use it on my own keyboard, this project's main goal was to learn the
rust programming language.

## Features

- Customizable pin layout.
- Support for direct and matrix wiring layouts.
- Layers.
- One shot modifiers.
- Modifier + key combos.

## Building and flashing firmware

To build the firmware you will need to install
[rust](https://www.rust-lang.org/), the toolchain for the ARM Cortex-M0+
[target](https://doc.rust-lang.org/rustc/platform-support/thumbv6m-none-eabi.html)
and tooling to convert the elf binary to uf2
([elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) seems to be the de facto
tool for doing it). With this requirements met, running the following
commands at the root of the project will build our firmware.

```bash
    cargo build --profile lto --no-default-features --features={keyboard} &&
    elf2uf2-rs target/thumbv6m-none-eabi/lto/rkb2040
```

To flash the firmware: set your board into bootloader mode, mount the board's
mass storage device to your computers file system and copy the file
`target/thumbv6m-none-eabi/lto/rkb2040.uf2` to it.

> [!NOTE] Assuming there is no automatic drive mounting and that no other SATA
> drives are connected, the `build.sh` script will do the previous steps
> automatically (possibly leaving you with a new `/mnt/usb1` directory). The
> script is there only for my own convenience but it might prove useful to you.

## Writing firmware for your own keyboard

To write firmware for a new keyboard you will need to specify 4 items:

1. A board support package (bsp) for your rp2040 board.
2. A hardware layout.
3. A keybind map.
4. A main execution loop.

All of this should require minimal work thanks to the modules in `rkb2040-lib`.
To do so add a new sub-module to the `rkb2040::keyboards` module. For a direct
wiring layout example look at `rkb2040/keyboards/sweep2` and for a matrix
layout example look at `rkb2040/keyboards/skeletyl`.

> [!NOTE] Custom bsp's for the
> [sea-picro](https://github.com/joshajohnson/sea-picro) and the
> [splinky](https://github.com/Bastardkb/Splinky) are included in the `bsp/`
> directory. You can find many more
> [here](https://github.com/rp-rs/rp-hal-boards), courtesy of the fine folks at
> the rp-hal project.

> [!IMPORTANT] Keybinds are assigned left to right, top to bottom, following
> the order of keys specified in the hardware layout definition.

## TODOs

- Add support for 'COL2ROW' matrix scanning.
- Add logic for entering bootloader mode nicely.

## Acknowledgements

Many of the problems I faced were previously solved by the following projects
and crates:

- [rp-hal](https://github.com/rp-rs/rp-hal).
- [MarcusGrass/rp2040-kbd](https://github.com/MarcusGrass/rp2040-kbd).
- [ArchUsr64/egboard](https://github.com/ArchUsr64/egboard).
- [usbd-human-interface-device](https://github.com/dlkj/usbd-human-interface-device).
