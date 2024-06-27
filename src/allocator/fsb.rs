use core::{
    alloc::{GlobalAlloc, Layout},
    mem,
    ptr::{self, NonNull},
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

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_alloc.init(heap_start, heap_size);
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

        match list_index(&layout) {
            None => allocator.fallback_allocator(layout),
            Some(index) => match allocator.list_heads[index].take() {
                None => {
                    let block_size = BLOCK_SIZES[index];
                    let block_align = block_size;

                    let new_layout = Layout::from_size_align(block_size, block_align).unwrap();
                    allocator.fallback_allocator(new_layout)
                }
                Some(list_node) => {
                    allocator.list_heads[index] = list_node.next.take();
                    list_node as *mut ListNode as *mut u8
                }
            },
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        match list_index(&layout) {
            None => allocator
                .fallback_alloc
                .deallocate(NonNull::new(ptr).unwrap() as NonNull<u8>, layout),
            Some(index) => {
                let new_list_node: ListNode = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]); // confirm that the block size has the required
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]); // size + alignment to store the list node

                let new_list_node_ptr = ptr as *mut ListNode;
                new_list_node_ptr.write(new_list_node); // write new list node to the location of
                                                        // the new list node's ptr
                allocator.list_heads[index] = Some(&mut *new_list_node_ptr);
            }
        }
    }
}
