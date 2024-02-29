use core::{mem::size_of, ptr::null_mut};

extern "C" {
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
}

static mut ALLOC_START: usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;


#[repr(u8)]
pub enum PageBits {
	Empty = 0,
	Taken = 1 << 0,
	Last = 1 << 1,
}

impl PageBits {
	// We convert PageBits to a u8 a lot, so this is
	// for convenience.
	pub fn val(self) -> u8 {
		self as u8
	}
}

pub struct Page {
	flags: u8,
}

impl Page {
	// If this page has been marked as the final allocation,
	// this function returns true. Otherwise, it returns false.
	pub fn is_last(&self) -> bool {
		self.flags & PageBits::Last.val() != 0
	}

	// If the page is marked as being taken (allocated), then
	// this function returns true. Otherwise, it returns false.
	pub fn is_taken(&self) -> bool {
		self.flags & PageBits::Taken.val() != 0
	}

	// This is the opposite of is_taken().
	pub fn is_free(&self) -> bool {
		!self.is_taken()
	}

	// Clear the Page structure and all associated allocations.
	pub fn clear(&mut self) {
		self.flags = PageBits::Empty.val();
	}

	// Set a certain flag. We ran into trouble here since PageBits
	// is an enumeration and we haven't implemented the BitOr Trait
	// on it.
	pub fn set_flag(&mut self, flag: PageBits) {
		self.flags |= flag.val();
	}

	pub fn clear_flag(&mut self, flag: PageBits) {
		self.flags &= !(flag.val());
	}
}

pub fn alloc(pages: usize) -> *mut u8 {
	// We have to find a contiguous allocation of pages
	assert!(pages > 0);
	unsafe {
		// We create a Page structure for each page on the heap. We
		// actually might have more since HEAP_SIZE moves and so does
		// the size of our structure, but we'll only waste a few bytes.
		let num_pages = HEAP_SIZE / PAGE_SIZE;
		let ptr = HEAP_START as *mut Page;
		for i in 0..num_pages - pages {
			let mut found = false;
			// Check to see if this Page is free
			if (*ptr.add(i)).is_free() {
				found = true;
				for j in i..i + pages {
					if (*ptr.add(j)).is_taken() {
						found = false;
						break;
					}
				}
			}
		if found {
			for k in i..i + pages - 1 {
				(*ptr.add(k)).set_flag(PageBits::Taken);
			}

			(*ptr.add(i+pages-1)).set_flag(PageBits::Taken);
			(*ptr.add(i+pages-1)).set_flag(PageBits::Last);

			return (ALLOC_START + PAGE_SIZE * i)
					as *mut u8;
			}
		}
	}

	// If we get here, that means that no contiguous allocation was
	// found.
	null_mut()
}


pub fn dealloc(ptr: *mut u8) {
	assert!(!ptr.is_null());
	unsafe {
		let addr =
			HEAP_START + (ptr as usize - ALLOC_START) / PAGE_SIZE;

		assert!(addr >= HEAP_START && addr < HEAP_START + HEAP_SIZE);
		let mut p = addr as *mut Page;

		while (*p).is_taken() && !(*p).is_last() {
			(*p).clear();
			p = p.add(1);
		}
		assert!(
				(*p).is_last() == true,
				"Possible double-free detected! (Not taken found \
					before last)"
		);
		// Handle last page
		(*p).clear();
	}
}

/// Allocate and zero a page or multiple pages
/// pages: the number of pages to allocate
/// Each page is PAGE_SIZE which is calculated as 1 << PAGE_ORDER
/// On RISC-V, this typically will be 4,096 bytes.
pub fn zalloc(pages: usize) -> *mut u8 {
	let ret = alloc(pages);
	if !ret.is_null() {
		let size = (PAGE_SIZE * pages) / 8;
		let big_ptr = ret as *mut u64;
		for i in 0..size {
			unsafe {
				(*big_ptr.add(i)) = 0;
			}
		}
	}
	ret	
}

use crate::println;
use crate::print;

/// Print all page allocations
/// This is mainly used for debugging.
pub fn print_page_allocations() {
	unsafe {
		let num_pages = HEAP_SIZE / PAGE_SIZE;
		let mut beg = HEAP_START as *const Page;
		let end = beg.add(num_pages);
		let alloc_beg = ALLOC_START;
		let alloc_end = ALLOC_START + num_pages * PAGE_SIZE;
		println!();
		println!(
					"PAGE ALLOCATION TABLE\nMETA: {:p} -> {:p}\nPHYS: \
					0x{:x} -> 0x{:x}",
					beg, end, alloc_beg, alloc_end
		);
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
		let mut num = 0;
		while beg < end {
			if (*beg).is_taken() {
				let start = beg as usize;
				let memaddr = ALLOC_START
								+ (start - HEAP_START)
								* PAGE_SIZE;
				print!("0x{:x} => ", memaddr);
				loop {
					num += 1;
					if (*beg).is_last() {
						let end = beg as usize;
						let memaddr = ALLOC_START
										+ (end
											- HEAP_START)
										* PAGE_SIZE
										+ PAGE_SIZE - 1;
						print!(
								"0x{:x}: {:>3} page(s)",
								memaddr,
								(end - start + 1)
						);
						println!(".");
						break;
					}
					beg = beg.add(1);
				}
			}
			beg = beg.add(1);
		}
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
		println!(
					"Allocated: {:>6} pages ({:>10} bytes).",
					num,
					num * PAGE_SIZE
		);
		println!(
					"Free     : {:>6} pages ({:>10} bytes).",
					num_pages - num,
					(num_pages - num) * PAGE_SIZE
		);
		println!();
	}
}	
