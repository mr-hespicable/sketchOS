struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
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

    ///Initialize allocator with given heap bounds
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.add_free_region(heap_start, heap_end);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        todo!()
    }
}
