# Rust Embedded HAL: From Zero to Aync

> [!NOTE]
> This tutorial uses BBC Microbit v2, here's another tutorial that uses [STM32 Bluepill](https://github.com/wuhanstudio/stm32-bluepill-rust-embedded-hal).

> BBC Microbit v2 (nRF52833 Cortex-M4)  
> Flash: 512KB, SRAM:128KB, CPU: 64MHz

<img width="653" height="284" alt="image" src="https://github.com/user-attachments/assets/3eb503e4-cd1d-46d4-8f3a-5ecf7b95b928" />

## Overview

| Module                |                      BiliBili                       |                                                                                             YouTube | Code                                                                                                |
| :-------------------- | :-------------------------------------------------: | --------------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------- |
| Overview              | [Link](https://www.bilibili.com/video/BV1fu4WzmECd) | [Link](https://www.youtube.com/watch?v=WTXYgtFgWV8&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=1) | -                                                                                                   |
| RTT Print             | [Link](https://www.bilibili.com/video/BV1xGsMzCEUh) | [Link](https://www.youtube.com/watch?v=n-cmjDTjK5Y&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=2) | [00-hello-rtt-print](00-hello-rtt-print/)                                                           |
| Defmt Print           | [Link](https://www.bilibili.com/video/BV1xGsMzCEUh) | [Link](https://www.youtube.com/watch?v=n-cmjDTjK5Y&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=2) | [01-hello-defmt-rtt](01-hello-defmt-rtt/) |
| HAL (Delay)           |                        [Link](https://www.bilibili.com/video/BV1dM2aBFEKh)                         |                                                                                            [Link](https://www.youtube.com/watch?v=LIeYggkQD_E&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=3) | [02-hello-delay](02-hello-delay/)                                                                   |
| HAL (GPIO)            |                        [Link](https://www.bilibili.com/video/BV1W92dB2Eku)                         |                                                                                            [Link](https://www.youtube.com/watch?v=NTi6t9zFtBY&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=4) | [03-hello-gpio](03-hello-gpio/)                                                                     |
| HAL (I2C)             |                        [Link](https://www.bilibili.com/video/BV1cmibB6Edw)                         |                                                                                           [Link](https://www.youtube.com/watch?v=yCe5UF39y0k&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=5) | [04-hello-i2c](04-hello-i2c/)                                                                       |
| HAL (SPI)             |                        [Link](https://www.bilibili.com/video/BV191qPBZE6e)                         |                                                                                            [Link](https://www.youtube.com/watch?v=pMBSY-vYTx0&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=6) | [05-hello-spi](05-hello-spi/)                                                                       |
| HAL (UART)            |                        [Link](https://www.bilibili.com/video/BV1rqi1BHE14)                         |                                                                                            [Link](https://www.youtube.com/watch?v=xAD05rUIZVs&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=7) | [06-hello-uart](06-hello-uart/)                                                                     |
| Async (Ticker)          |                        [Link](https://www.bilibili.com/video/BV12ZPxz5ELq)                         |                                                                                            [Link](https://www.youtube.com/watch?v=-wxTweZdns4&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=9) | [07-hello-systick](07-hello-systick/)                                                               |
| Async (Timer)           |                        [Link](https://www.bilibili.com/video/BV1RmP8ziEBA)                         |                                                                                            [Link](https://www.youtube.com/watch?v=RY4PLtTdgHk&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=10) | [08-hello-systick-static](08-hello-systick-static/)                                                 |
| Async (State Machine) |                        [Link](https://www.bilibili.com/video/BV1JEdsBkEff/)                         |                                                                                            [Link](https://www.youtube.com/watch?v=KNH0XILfSQs&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=13) | [09-hello-state-machine](09-hello-state-machine/)                                                   |
| Async (Future)        |                        [Link](https://www.bilibili.com/video/BV1fAdxBrEsJ/)                         |                                                                                            [Link](https://www.youtube.com/watch?v=gNSgA1Sq1yk&list=PLlRCv8NaDaU8XxW7s7M4qfnb-dzwu5Qs9&index=14) | [10-hello-future-timer](10-hello-future-timer/)                                                     |
| Async (Executor)      |                        Coming Soon                         |                                                                                            Coming Soon | [11-hello-future-executor](11-hello-future-executor/)                                               |
| Async (Waker)         |                        Coming Soon                         |                                                                                            Coming Soon | [12-hello-waker](12-hello-waker/)                                              |
| Async (Await)         |                        Coming Soon                         |                                                                                            Coming Soon | [13-hello-async-await](13-hello-async-await/)                                                       |
| Conclusion         |                        Coming Soon                         |                                                                                            Coming Soon | -                                                       |

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
