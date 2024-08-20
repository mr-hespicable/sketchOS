use core::ops::Range;

use superblock::Superblock;

pub mod inode;
pub mod superblock;

const BOOTLOADER_SIZE: usize = 512; // 1 sector
const KERNAL_OFFSET: usize = 2 * 1024 * 1024; // 2 MB
const PADDING: usize = 2 * 1024 * 1024 - BOOTLOADER_SIZE; // 2 MB - the bootloader size
pub const SUPERBLOCK_OFFSET: usize = BOOTLOADER_SIZE + KERNAL_OFFSET + PADDING; // offset for superblock (4MB)

pub trait BlockDevice {
    fn read_block(&self, block_num: u64, data: &mut [u8]);
    fn write_block(&mut self, block_num: u64, data: &[u8]);
}

pub struct DiskImage {
    data: &'static mut [u8],
    block_size: u64, // 8 bytes
}

impl DiskImage {
    pub fn new(data: &'static mut [u8], block_size: u64) -> Self {
        DiskImage { data, block_size }
    }

    fn get_range(&self, block_num: u64, data: &[u8]) -> Range<usize> {
        let offset = SUPERBLOCK_OFFSET + (block_num * self.block_size) as usize; // offset from beginning of DiskImage
        offset..offset + data.len()
    }
}

impl BlockDevice for DiskImage {
    fn read_block(&self, block_num: u64, data: &mut [u8]) {
        let range = self.get_range(block_num, data);
        data.copy_from_slice(&self.data[range]); // cp info from given range in disk into data buf.
    }

    fn write_block(&mut self, block_num: u64, data: &[u8]) {
        let range = self.get_range(block_num, data);
        self.data[range].copy_from_slice(data); // cp data from data buf into disk.
    }
}

/// initializes the superblock, TODO: more
pub fn init() {
    Superblock::init();
}
