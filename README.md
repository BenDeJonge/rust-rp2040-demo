# rp2040-demo

This repository serves as a basic introduction to embedded Rust development. The
used hardware is a
[WaveShare RP2040-Zero](https://www.waveshare.com/wiki/RP2040-Zero), a dual-core
[ARM Cortex M0+ device](https://en.wikipedia.org/wiki/ARM_Cortex-M#Cortex-M0+).

## Getting started

- Create a new Rust project.

```bash
$ cargo new rp2040-demo
$ cd rp2040-demo
$ code - r.
```

### Essential dependencies

- To write to the board, install the
  [`cargo-embed`](https://crates.io/crates/cargo-embed) crate, which is part of
  [ `probe-rs`](https://probe.rs/).

```bash
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

- Verify that the wanted architecture is available.

```bash
$ probe-rs chip list | grep Cortex-M0+
```

- Ensure that `rust-analyzer` compiles code for the target architecture, instead
  of the host, in the [`.vscode/settings.json`](.vscode/settings.json).

### Hardware-specific dependencies for RP2040

- Install the required build target.

```bash
$ rustup target add thumbv6m-none-eabi
```

- Install two required crates:
  - `cortex-m-rt`, which is a minimal runtime for Cortex-M microcontrollers.
    This contains all the required parts to build a `no_std` binary.
  - `rp2040-boot2` which is a
    [second-stage bootloader](https://en.wikipedia.org/wiki/Bootloader#Second-stage_boot_loader).
    This loads the "operating system" (in this case, `main.rs`) onto the chip
    after the first-stage has loaded the BIOS.

```bash
$ cargo add cortex-m-rt rp2040-boot2
```

- Setup the [memory map](memory.x) for the used hardware, as specified in the
  datasheet. This should include at least the start of the RAM and FLASH
  memories. Since a second-stage bootloader is used, this is also added.

- If you _do not_ want to use [probe.rs](https://probe.rs/) hardware, switch to
  the [`elf2uf2-rs`](https://crates.io/crates/elf2uf2-rs) runner using
  `cargo build` compiler flags with `rust-analyzer`. (If there is an error that
  `libudev` cannot be found, install it with `$pacman -Syu systemd-libs`. This
  might be quite esoteric but I am writing this on a
  [Steam Deck](https://store.steampowered.com/steamdeck) of all things...)

```bash
$ cargo install elf2uf2-rs --locked
```

### Convenience settings

- To avoid continuously entering linking arguments when building, prepare a
  [`.cargo/config.toml`](.cargo/config.toml) file.

- Improve the ergonomics of LLVM tooling. This allows evaluation of build
  artefacts with LLVM tooling as `cargo` subcommands.

```bash
$ rustup component add llvm-tools
$ cargo install cargo-binutils
$ cargo size -- -Ax # test command
```

<!-- automatically mount specific drives.
https://forum.manjaro.org/t/root-tip-how-to-use-systemd-to-mount-any-device/1185
 -->

### Testing the workflow

- Prepare [hello world `main.rs`](examples/hello_world/main.rs).
- Test the build process.

```bash
$ cargo build --example hello_world
```

- Reset the device with the button combination: `RESET -> RESET + BOOT -> BOOT`.
  If needed, mount the device.
- Embed the code onto the device.

```bash
$ cargo run --release examples/hello_world/main.rs
```

Alternatively, if you have a `probe.rs` debugger, use

```bash
$ cargo embed --chip Cortex-M0+
```

### Develop and flash the actual binary

- Embed a newly developed version of [`main.rs`](src/main.rs) onto hardware.

```bash
$ cargo build
$ cargo run --release
```

## Sources

- [WaveShare RP2040-Zero](https://www.waveshare.com/wiki/RP2040-Zero)
- [RP2040 datasheet](https://pip-assets.raspberrypi.com/categories/814-rp2040/documents/RP-008371-DS-1-rp2040-datasheet.pdf?disposition=inline)
- [ARMv6-M reference](https://users.ece.utexas.edu/~valvano/mspm0/Arm_Architecture_v6m_Reference_Manual.pdf)
- [RustC platform support for `thumbv6n-none-eabi`](https://doc.rust-lang.org/beta/rustc/platform-support/thumbv6m-none-eabi.html)
- The Rusty Bits YouTube videos
  - [Embedded Rust setup explained](https://www.youtube.com/watch?v=TOAynddiu5M)
  - [Blinking an LED: Embedded Rust ecosystem explored](https://www.youtube.com/watch?v=A9wvA_S6m7Y)
  - [From Zero to Async in Embedded Rust](https://www.youtube.com/watch?v=wni5h5vIPhU)
