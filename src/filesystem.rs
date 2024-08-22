use core::{arch::asm, ops::Range};

use superblock::Superblock;

use crate::println;

pub mod inode;
pub mod superblock;

const BOOTLOADER_SIZE: usize = 512; // 1 sector; 512 KiB
const KERNAL_OFFSET: usize = 2 * 1024 * 1024; // 2 MiB
const PADDING: usize = 14 * 1024 * 1024 - BOOTLOADER_SIZE; // 14 MiB - the bootloader size
pub const SUPERBLOCK_OFFSET: usize = BOOTLOADER_SIZE + KERNAL_OFFSET + PADDING; // offset for superblock (16MiB)

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

impl DiskImage {
    fn read_block(&self, block_num: u64, buffer: &mut [u8]) {
        let range = self.get_range(block_num, buffer);
        buffer.copy_from_slice(&self.data[range]); // cp info from given range in disk into data buf.
    }

    pub fn write_block(&mut self, sector_number: u8, buffer: &[u8]) -> Result<(), ()> {
        //    assert!(buffer.len() <= self.block_size as usize);
        //     let range = self.get_range(block_num, buffer);
        //      self.data[range].copy_from_slice(buffer); // cp data from data buf into disk.
        unsafe {
            asm!(
                "mov al, 1",              // Number of sectors to write
                "mov ch, 0",              // Cylinder number
                "mov cl, {0:l}",          // Sector number (1-18)
                "mov dh, 0",              // Head number
                "mov dl, 0x80",           // Drive number (0x80 for the first hard drive)
                "mov es, {1:x}",          // Data segment
                "mov bx, {2:x}",          // Data offset
                "int 0x13",               // BIOS interrupt
                in(reg) sector_number as i16,
                in(reg) &buffer as *const _ as u16,
                in(reg) &buffer [0] as *const _ as u16,
                options(nostack)
            );

            // Check for errors (carry flag)
            let carry_flag: i16;
            asm!(
                "lahf",              // Load AH register into FLAGS
                "test ah, 0x01",    // Check carry flag
                "mov {0:l}, ah",      // Store carry flag
                out(reg) carry_flag,
                options(nostack)
            );

            if carry_flag != 0 {
                Err(()) // Error occurred
            } else {
                Ok(()) // Success
            }
        }
    }
}

/// initializes the superblock, TODO: more
pub fn init() {
    // Superblock::init();
}
