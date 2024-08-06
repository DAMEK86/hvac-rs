# hvac

simple program to control some values and a hvac heat pump using ds18b20s and relays

Compile against riscv targets
```sh
cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std=core,std,panic_abort --release
```