use super::align_up;
use core::mem;

struct ListNode {
    size: usize, // size of listnode
    next: Option<&'static mut ListNode>, // pointer to the next list node TODO: check if this is
                 // actually true
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
}