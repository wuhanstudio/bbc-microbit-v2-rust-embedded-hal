# Rust Embedded HAL: From Zero to Aync

> BBC Microbit v2 (nRF52833 Cortex-M4)  
> Flash: 512KB, SRAM:128KB, CPU: 64MHz

<img width="653" height="284" alt="image" src="https://github.com/user-attachments/assets/3eb503e4-cd1d-46d4-8f3a-5ecf7b95b928" />

## Prerequisites

Install [Probe-rs](https://probe.rs/docs/getting-started/installation/):

```
# Linux
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

```
# Windows
$ Set-ExecutionPolicy RemoteSigned -scope CurrentUser
$ irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

Install Rust toolchain:

```
# Linux
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
# Windows
$ winget install rustup

# If you don't have Visual Studio on Windows
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup default stable-x86_64-pc-windows-gnu
```

Add Cortex-M thmbv7em (with hardware floating point) support

```
$ rustup target add thumbv7em-none-eabihf
```

## References

- Async/await on Embedded Rust: https://ferrous-systems.com/blog/async-on-embedded/#from-blocking-to-non-blocking
- TheRustyBits: https://www.youtube.com/@therustybits
- From Zero to Async: https://github.com/therustybits/zero-to-async
