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

- Install the required build targets for the RP2040.

```bash
$ rustup target add thumbv6m-none-eabi
```

- Install two required crates:
  - the `cortex-m-rt` crate, which is a minimal runtime for Cortex-M
    microcontrollers.
  - `rp2040-boot2` which is a second-stage bootloader.

```bash
$ cargo add cortex-m-rt rp2040-boot2
```

- Setup the [memory map](memory.x) for the used hardware, as specified in the
  datasheet. This should include at least the start of the RAM and FLASH
  memories.

- To avoid continuously entering linking arguments when building, prepare a
  [`.cargo/config.toml`](.cargo/config.toml) file.

```bash
$ touch mkdir .cargo && touch .cargo/config.toml
```

- Ensure that `rust-analyzer` compiles created code for the target architecture,
  not the host.

```bash
$ mkdir .vscode && touch .vscode/settings.json
```

- Improve the ergonomics of LLVM tooling. This allows us to evaluate build
  artefacts with LLVM tooling as `cargo` subcommands.

```bash
$ rustup component add llvm-tools
$ cargo install cargo-binutils
$ cargo size -- -Ax # test command
```

- To write to the board, install another `cargo-embed`, which is part of
  `probe-rs`. This can be installed using:

```bash
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

- Verify that the wanted architecture is available.

```bash
$ probe-rs chip list | grep Cortex-M0+
```

- Switch to the [`elf2uf2-rs`](https://crates.io/crates/elf2uf2-rs) runner using
  `cargo build` compiler flags with `rust-analyzer`. Using this more low-level
  runner, specific for RP Pico devices, deployment can be done without
  [`probe-rs`](https://probe.rs/) hardware. (If there is an error that `libudev`
  cannot be found, install it with `$pacman -Syu systemd-libs`. This might be
  quite esoteric but I am writing this on a
  [Steam Deck](https://store.steampowered.com/steamdeck) of all things...)

```bash
$ cargo install elf2uf2-rs --locked
```

- Test the workflow.

  - Prepare dummy `main.rs`.

  ```rust
  #![no_std]
  #![no_main]

  use core::panic::PanicInfo;

  use cortex_m_rt::entry;

  /// The linker will place this boot block at the start of our program image. We
  /// need this to help the ROM bootloader get our code up and running.
  /// Note: This boot block is not necessary when using a rp-hal based BSP
  /// as the BSPs already perform this step.
  #[link_section = ".boot2"]
  #[used]
  pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

  #[entry]
  fn main() -> ! { loop {} }

  #[panic_handler]
  fn panic(\_i: &PanicInfo) -> ! { loop {} }
  ```

  - Test the build process.

  ```bash
  $ cargo build
  ```

  - Reset the device with the button combination:
    `RESET -> RESET + BOOT -> BOOT`.

  - Embed the code onto the device.

  ```bash
  $ cargo run --release
  ```

  Alternatively, if you have a `probe.rs` debugger, use

  ```bash
  $ cargo embed --chip Cortex-M0+
  ```

## Sources

- [WaveShare RP2040-Zero](https://www.waveshare.com/wiki/RP2040-Zero)
- [RP2040 datasheet](https://pip-assets.raspberrypi.com/categories/814-rp2040/documents/RP-008371-DS-1-rp2040-datasheet.pdf?disposition=inline)
- [ARMv6-M reference](https://users.ece.utexas.edu/~valvano/mspm0/Arm_Architecture_v6m_Reference_Manual.pdf)
- [RustC platform support for `thumbv6n-none-eabi`](https://doc.rust-lang.org/beta/rustc/platform-support/thumbv6m-none-eabi.html)
- The Rusty Bits YouTube videos
  - [Embedded Rust setup explained](https://www.youtube.com/watch?v=TOAynddiu5M)
  - [Blinking an LED: Embdeded Rust ecosystem explored](https://www.youtube.com/watch?v=A9wvA_S6m7Y)
  - [From Zero to Async in Embedded Rust](https://www.youtube.com/watch?v=wni5h5vIPhU)
