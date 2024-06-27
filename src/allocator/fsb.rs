use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

use super::Locked;

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048]; // TODO: add additional
                                                                           // block sizes
struct ListNode {
    next: Option<&'static mut ListNode>,
}

pub struct FSBAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_alloc: linked_list_allocator::Heap,
}

impl FSBAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        FSBAllocator {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_alloc: linked_list_allocator::Heap::empty(),
        }
    }

    fn fallback_allocator(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_alloc.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Returns lowest possible block size for the given layout
fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

unsafe impl GlobalAlloc for Locked<FSBAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        if list_index(&layout) == None {
            allocator.fallback_allocator(layout)
        } else {
            todo!()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
