# Breakaway Trestle

Get it? No?

A trestle is sort of like a bridge, but in my head I picture it as metal and rusting...

This project attempts to reimplement the functionality from [Breakaway Bridge](https://github.com/thomashinds/breakaway-bridge),
but written in Rust for the Raspberry Pi Pico W (rather than in C++ for the ESP32).

Currently this is very far from feature complete, but the basic Rust-C (or C++) interoperation is in place,
including calling Raspberry Pi Pico SDK library functions from the Rust application, and building the whole project
using a single command that uses both CMake and Cargo under the hood.

## High Level Design

 - `app/` Main application logic, written in Rust
 - `init/` Application entrypoint (C++)
 - `pico_w_sys/` Rust bindings for the Raspberry Pi Pico C SDK, including BLE on the Pico W