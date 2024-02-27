use core::sync::atomic::{AtomicUsize};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use core::sync::atomic::{Ordering};

pub struct LinearAllocator {
    head: AtomicUsize, // the current index of the buffer
    // AtomicUsize is a special type that allows us to safely share data
    // between threads without using locks

    start: *mut u8, // raw pointer to the start of the heap
    end: *mut u8,   // raw pointer to the end of the heap
}
// allow our allocator to be shared between threads
unsafe impl Sync for LinearAllocator {}

impl LinearAllocator {
    pub const fn empty() -> Self {
        Self {
            head: AtomicUsize::new(0),
            start: core::ptr::null_mut(),
            end: core::ptr::null_mut(),
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.start = start as *mut u8;
        self.end = unsafe { self.start.add(size) };
    }
}

unsafe impl GlobalAlloc for LinearAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        /* The byte multiple that our allocated memory must start at
          most hardware architectures perform better when reading/writing
          data at aligned addresses (e.g. 4 bytes, 8 bytes, etc.) so we
          need to make sure that our memory is aligned properly
        */
        let align = layout.align();

        // The size is the number of bytes we need to allocate
        let size = layout.size();

        let mut head = self.head.load(Ordering::Relaxed);

        // Align the head to the required alignment
        // e.g. if head is 1 and align is 4, we need to add 3 to head to get 4
        if head % align != 0 {
            head += align - (head % align);
        }

        // Move the head forward by the size of the allocation
        let new_head = head + size;

        // are we out of memory?
        if self.start.add(new_head) > self.end {
            return core::ptr::null_mut();
        }

        self.head.store(new_head, Ordering::Relaxed);
        NonNull::new_unchecked(self.start.add(head) as *mut u8).as_ptr()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // no-op
    }
}
