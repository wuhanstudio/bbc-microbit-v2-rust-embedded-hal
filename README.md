# Rust Embedded HAL: From Zero to Aync

> BBC Microbit v2 (nRF52833 Cortex-M4)  
> Flash: 512KB, SRAM:128KB, CPU: 64MHz

<img width="653" height="284" alt="image" src="https://github.com/user-attachments/assets/3eb503e4-cd1d-46d4-8f3a-5ecf7b95b928" />

```
# If you don't have Visual Studio on Windows
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup default stable-x86_64-pc-windows-gnu

$ rustup target add thumbv7em-none-eabihf

```

## References

- Async/await on Embedded Rust: https://ferrous-systems.com/blog/async-on-embedded/#from-blocking-to-non-blocking
- TheRustyBits: https://www.youtube.com/@therustybits
- From Zero to Async: https://github.com/therustybits/zero-to-async
