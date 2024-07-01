#![allow(clippy::all)]

use core::array::from_fn;

const SLAB_SIZE: usize = 32; // objs per slab

/// A slab. the value `objects` corresponds to an array of length SLAB_SIZE, filled with
/// the type T surrounded by an Option. This enables it to be set to None when implemented.
struct Slab<T> {
    objects: [Option<T>; SLAB_SIZE], // Array of objects.
    free_list: [usize; SLAB_SIZE],
    free_count: usize,
}

impl<T: core::marker::Copy> Slab<T> {
    pub fn new() -> Self {
        let empty: Option<T> = None;
        Slab {
            objects: [empty; SLAB_SIZE],
            free_list: from_fn(|i| i),
            free_count: 0,
        }
    }
}
