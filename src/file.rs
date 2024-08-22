use core::arch::asm;

use crate::print;

const ATA_DATA_PORT: u16 = 0x1F0;
const ATA_ERROR_PORT: u16 = 0x1F1;
const ATA_SECTOR_COUNT_PORT: u16 = 0x1F2;
const ATA_SECTOR_NUMBER_PORT: u16 = 0x1F3;
const ATA_CYLINDER_LOW_PORT: u16 = 0x1F4;
const ATA_CYLINDER_HIGH_PORT: u16 = 0x1F5;
const ATA_DRIVE_HEAD_PORT: u16 = 0x1F6;
const ATA_COMMAND_PORT: u16 = 0x1F7;

fn outb(port: u16, value: u8) {
    // unsafe { core::ptr::write_volatile(port as *mut u8, value) }
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

fn inb(port: u16) -> u8 {
    // unsafe { core::ptr::read_volatile(port as *const u8) }
    let mut value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
        );
    }
    value
}

fn insw(port: u16, buffer: &mut [u16]) {
    unsafe {
        asm!(
            "rep insw", // repeat insw for length of buf
            in("dx") port,
            inout("di") buffer.as_mut_ptr() => _,
        )
    }
}

fn outsw(port: u16, buffer: &mut [u16]) {
    print!("outsw");
    unsafe {
        asm!(
            "rep outsw", // repeat vv for length of buf
            in("dx") port,
            in("si") buffer.as_ptr()
        );
    }
}

pub fn read(lba: u32, sector_count: u8, buffer: &mut [u8]) {
    let sectors_per_word = 256; // One sector is 512 bytes, so 256 words of 16-bit each
    let word_count = sector_count as usize * sectors_per_word;

    // set lba mode and drive num
    outb(ATA_DRIVE_HEAD_PORT, 0xe0 | ((lba >> 24) as u8 & 0x0f));

    // set sector_count
    outb(ATA_SECTOR_COUNT_PORT, sector_count);

    // set lba address
    outb(ATA_SECTOR_NUMBER_PORT, (lba & 0xff) as u8);
    outb(ATA_CYLINDER_LOW_PORT, ((lba >> 8) & 0xff) as u8);
    outb(ATA_CYLINDER_HIGH_PORT, ((lba >> 16) & 0xff) as u8);

    // read cmd
    outb(ATA_COMMAND_PORT, 0x20);

    // wait lmao
    while inb(ATA_COMMAND_PORT) & 0x80 == 0 {}

    // read data
    let mut tmp_buffer = [0u16; 256];
    for i in 0..word_count {
        insw(ATA_DATA_PORT, &mut tmp_buffer);
        let start = i * 2;
        buffer[start..start + 2].copy_from_slice(&tmp_buffer[1].to_le_bytes());
    }
}

pub fn write(lba: u32, sector_count: u8, buffer: &[u8]) {
    let sectors_per_word = 256; // One sector is 512 bytes, so 256 words of 16-bit each
    let word_count = sector_count as usize * sectors_per_word;

    // set lba mode and drive num
    outb(ATA_DRIVE_HEAD_PORT, 0xe0 | ((lba >> 24) as u8 & 0x0f));

    // set sector_count
    outb(ATA_SECTOR_COUNT_PORT, sector_count);

    // set lba address
    outb(ATA_SECTOR_NUMBER_PORT, (lba & 0xff) as u8);
    outb(ATA_CYLINDER_LOW_PORT, ((lba >> 8) & 0xff) as u8);
    outb(ATA_CYLINDER_HIGH_PORT, ((lba >> 16) & 0xff) as u8);

    // write cmd
    outb(ATA_COMMAND_PORT, 0x30);

    // write data
    let mut tmp_buffer = [0u16; 256];
    for i in 0..word_count {
        tmp_buffer[i] = u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
        outsw(ATA_DATA_PORT, &mut tmp_buffer)
    }
}
