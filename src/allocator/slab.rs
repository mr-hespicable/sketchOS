const SLAB_SIZE: usize = 32; // objs per slab

/// A slab. the value `objects` corresponds to an array of length SLAB_SIZE, filled with
/// the type T surrounded by an Option. This enables it to be set to None when implemented.
struct Slab<T> {
    objects: [Option<T>; SLAB_SIZE], // Array of objects.
    free_list: [usize; SLAB_SIZE],
    free_object_count: usize,
}

impl<T> Slab<T> {
    pub const fn new() -> Self {
        let mut free_list = [0; SLAB_SIZE];

        // TODO: implement free pointer
        // https://blogs.oracle.com/linux/post/linux-slub-allocator-internals-and-debugging-1
    }
}
