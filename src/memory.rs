use bootloader::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/// Returns usable frames from bootloader's memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// create frame allocator from passed memory_map
    ///
    /// unsafe fn bc caller must guarantee that the
    /// memory map is 100% valid - basically, they have
    /// to be marked as USABLE (flag)
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// returns an iterator over usable frames from memory map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable); // get usable regions
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr()); // map each region to its address range
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096)); // transform to an iterator of frame start addresses.
                                                                         // we use 4096 (4KiB) to get the start address of each frame

        // create PhysFrame types from start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// return page table offset from physical memory offset
pub unsafe fn init(physical_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_mem_offset);
    OffsetPageTable::new(level_4_table, physical_mem_offset)
}

/// Returns page table pointer reference to active level 4 table.
///
/// This is an unsafe function bc if physical mem is not
/// completely mapped to virt mem at phys_mem_offset bad things happen.
/// WARNING: DO NOT CALL THIS FUNCTION MORE THAN ONCE.
/// DOING SO WILL CAUSE ALIASING &mut REFERENCES. (undefined) (thus bad)
unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
