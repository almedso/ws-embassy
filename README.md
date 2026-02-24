# Workspace Rust Embedded Embassy

## Packages

**firmware-core**: Home of the business logic
* Follow the ports and adapter (hexagonal architecture) style
* No dependency to anything
* Interfaces modeled a neutral traits or as defined in embedded-hal, embedded graphics
* No-std crate

**firmware-sim**: Simulate the application on host
* Useful for fast turnaround development of business logic
* Useful for simulation e.g. bevy etc

**firmware-stm32f401re
* Board specific implementation of biz interfaces
* Board Nucleo STM32F401RE (512k Flash, 96k RAM)


## How to develop

### Prerequisites


* Install `probe-rs`
* Install rust and target toolchains

### 

