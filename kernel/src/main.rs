#![no_std]
#![no_main]
#![allow(unused)]

// RISC-V runtime
extern crate riscv_rt;

// Kernel control flow
use riscv_rt::entry;
mod panic_handler;
mod utils;

// Kernel memory management
extern crate alloc;
use alloc::vec::Vec;
mod linear_allocator;
mod heap;

// Kernel main

#[entry]
fn main(a0: usize) -> ! {
    println!("Hello world from hart {}\n", a0);


    // Initialize kernel heap allocator
    unsafe {
        heap::init_kernel_heap(); // new
    }

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    utils::shutdown();
}
