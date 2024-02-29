#![no_std]
#![no_main]
#![allow(unused)]

// RISC-V runtime
extern crate riscv_rt;

// Kernel control flow
use riscv_rt::entry;
mod panic_handler;
mod system_control;
mod io;
//mod paging;
//use crate::paging::print_page_allocations;

// Kernel memory management
extern crate alloc;
use alloc::vec::Vec;
mod linear_allocator;
mod heap;

// Kernel main

#[entry]
fn main(a0: usize) -> ! {

    crate::io::uart::uart_get(0x1000_0000);
    
    println!("\x1b[0;32m\nHello world from hart {}\n\x1b[0m", a0);


    // Initialize kernel heap allocator
    unsafe {
        heap::init_kernel_heap(); // new
    }

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    //print_page_allocations();

    loop {
        if let Some(c) = crate::io::uart::uart_get(0x1000_0000) {
            match c {
                8 => {
                    // Backspace
                    print!("{}{}{}", 8 as char, 'X', 8 as char);
                },
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                },
                _ => {
                    print!("{}", c as char);
                }
            }
        }
    }	

    system_control::shutdown();
}
