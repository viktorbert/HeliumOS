# Helium-DSE
A hobbyist operating system, attempting to create a modern bare-metal lisp environment.
Inspired by Henry Gressmans blog <https://blog.henrygressmann.de/rust-os/1-hello-riscv/>
and Stephen Marz' blog <https://osblog.stephenmarz.com/>.

## How to build
Dependencies:
- rustup
- qemu-system-riscv

```
rustup toolchain install stable

rustup component add rust-src --toolchain stable

rustup target add riscv64gc-unknown-none-elf --toolchain stable

cd ./kernel

cargo run
```

## Long Term goals
- GUI
- Netstack
- A custom Lisp runtime, based *roughly* on Clojure in terms of syntax and with a CLOS-style OOP system
