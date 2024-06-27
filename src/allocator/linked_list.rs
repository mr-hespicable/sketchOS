use super::{align_up, Locked};
use core::{
    alloc::{GlobalAlloc, Layout},
    mem, ptr,
};

struct ListNode {
    size: usize,                         // size of listnode
    next: Option<&'static mut ListNode>, // pointer to the next list node
}

impl ListNode {
    /// Create new ListNode
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    /// Returns the start address of the ListNode
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    /// Returns the end address of the ListNode
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    /// Create empty LinkedListAllocator
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0), // points to first heap region
        }
    }

    /// Initialize allocator with given heap bounds
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.add_free_region(heap_start, heap_end);
    }

    /// Adds the given memory region to the front of the list (push operation)
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // make sure nothing is fucked (memory is where it should be and node isnt bigger than
        // remaining space available)
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr); //is the address a list_node
                                                                       //and the same size
        assert!(size >= mem::size_of::<ListNode>());

        // creates new node and appends it at start of list
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    /// Looks for free region of given size & alignment, and removes it from the list.
    ///
    /// Returns an Option-wrapped tuple of the list node and the start address
    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                // the region is suitible for allocation -> remove node from list
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                // the region is not suitable -> continue with next region
                current = current.next.as_mut().unwrap()
            }
        }
        // no suitible region found
        None
    }

    /// Try to use the given region for an allocation with a given `size` and `alignment`.
    ///
    /// Returns this allocation's start address on success.
    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?; // checked addition so that
                                                                  // overflow does not occur; if it
                                                                  // does, return Err(()) from ok_or()
        if alloc_end > region.end_addr() {
            // region too small
            return Err(());
        }

        let excess = region.end_addr() - alloc_end;

        if excess > 0 && excess < mem::size_of::<ListNode>() {
            // rest of region (the excess) too small to hold a full ListNode
            return Err(());
        }

        Ok(alloc_start)
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>()) // attempt to increase alignment to size of listnode
            .expect("adjusting alignment failed")
            .pad_to_align(); // rounds size of the current block up to a multiple of the alignment, so the starting
                             // address of the next block will have the correct alignment for storing a listnode as
                             // well.
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout); // TODO: implement size_align
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow has occured");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }
            alloc_start as *mut u8 // return ptr to start of allocation region
        } else {
            ptr::null_mut() // return a null_mut to signify there is no mem region available
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout); // TODO: implement size_align

        self.lock().add_free_region(ptr as usize, size);
    }
}
