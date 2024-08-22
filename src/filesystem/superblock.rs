use crate::println;

// use super::BlockDevice;

#[allow(dead_code)]
#[repr(packed(2))]
#[derive(Debug, Clone, Copy)]
pub struct Superblock {
    /// The total number of inodes in the file system
    pub total_inodes: u32,
    /// The total number of blocks in the file system
    pub total_blocks: u32,
    /// The total number of blocks reserved for the superuser
    pub superuser_reserved: u32,
    /// Total unallocated blocks
    pub total_blocks_unallocated: u32,
    /// Total unallocated inodes
    pub total_inodes_unallocated: u32,
    /// Block number of the block containing the
    /// superblock (also the starting block number; NOT always zero.)
    pub superblock_number: u32,
    /// log2 (block size); x << 10 is block size (0 << 10 = 1024)
    pub block_size_log2: u32,
    /// log2 (fragment size); x << 10 is fragment size (0 << 10 = 1024)
    pub frag_size_log2: u32,
    /// number of blocks in each block group
    pub blocks_per_block_group: u32,
    /// number of fragments in each block group
    pub frags_per_block_group: u32,
    /// number of inodes in each block group
    pub inodes_per_block_group: u32,
    /// last mount time (in POSIX time)
    pub last_mount_posix: u32,
    /// last write time (in POSIX time)
    pub last_write_posix: u32,
    /// number of times since the volume has been mounted since its last consistency check
    pub mounts_since_last_check: u16,
    /// number of mounts allowed before a coonsistency check must be done
    pub mounts_for_check: u16,
    /// ext2 signature used to help confirm the presence of ext2 on a volume (0xef53)
    pub magic_number: u16,
    /// file system state. can be one of two values:
    /// 1: file system is clean
    /// 2: file system has errors
    pub system_state: u16,
    /// what to do when an error is detected. can be one of three values:
    /// 1: ignore the error (continue on)
    /// 2: remount file system as read-only
    /// 3: kernel panic
    pub error_decision: u16,
    /// minor portion of version.
    /// combine with major portion to construct full version field
    pub version_minor: u16,
    /// POSIX time of last consistency check
    pub last_check_posix: u32,
    /// interval (in posix time) between forced consistency checks
    pub forced_check_interval: u32,
    /// operating system id from which the filesystem on this volume was created
    pub os_id: u32,
    /// major portion of version.
    /// combine with minor portion to construct full version field
    pub version_major: u32,
    /// the user id that can use reserved blocks
    pub reserved_block_id_user: u16,
    /// the group id that can use reserved blocks
    pub reserved_block_id_group: u16,
    _padding: [u8; 940], // 1024 - 84 = 940 bytes of padding
}

fn buf_to_superblock(buffer: &[u8; 1024]) -> Superblock {
    assert_eq!(
        core::mem::size_of::<Superblock>(),
        1024,
        "struct_size_mismatch"
    );

    unsafe {
        let ptr = buffer.as_ptr() as *const Superblock;
        ptr.read()
    }
}

fn superblock_to_buf(superblock: &Superblock) -> &[u8; 1024] {
    unsafe { &*(superblock as *const Superblock as *const [u8; 1024]) }
}

impl Superblock {
    fn new() -> Self {
        Self {
            total_inodes: 0,
            total_blocks: 0,
            superuser_reserved: 0,
            total_blocks_unallocated: 0,
            total_inodes_unallocated: 0,
            superblock_number: 0xef53, // magic number
            block_size_log2: 4,        // block size of 4096 bytes; 0x4 << 10
            frag_size_log2: 4,         // block size of 4096 bytes; 0x4 << 10
            blocks_per_block_group: 0,
            frags_per_block_group: 0,
            inodes_per_block_group: 0,
            last_mount_posix: 0,
            last_write_posix: 0,
            mounts_since_last_check: 0,
            mounts_for_check: 0,
            magic_number: 0,
            system_state: 0,
            error_decision: 0,
            version_minor: 0,
            last_check_posix: 0,
            forced_check_interval: 0,
            os_id: 0,
            version_major: 0,
            reserved_block_id_user: 0,
            reserved_block_id_group: 0,
            _padding: [0u8; 940],
        }
    }

    //    pub fn read_from_disk(disk: &dyn BlockDevice) -> Self {
    //        let mut buffer = [0u8; 1024];
    //        disk.read_block(1, &mut buffer);
    //        buf_to_superblock(&buffer)
    //    }
    //
    //    pub fn init() {
    //        let superblock = &Superblock::new();
    //        let blocksize = superblock.block_size_log2;
    //        println!("{}", blocksize);
    //        DISK.lock().write_block(1, superblock_to_buf(superblock));
    //    }
    //
    //    pub fn is_valid(&self) -> bool {
    //        self.magic_number == 0xef53
    //    }
}
