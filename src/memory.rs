use x86_64::{
    structures::paging::{frame, OffsetPageTable, Page, PageTable, PhysFrame, Mapper, Size4KiB, FrameAllocator},
    PhysAddr, 
    VirtAddr,
};

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}


unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table, _) = Cr3::read(); //read lvl 4 reg 'CR3'

    let phys: PhysAddr = level_4_table.start_address(); //grab physical start address
    let virt: VirtAddr = physical_memory_offset + phys.as_u64(); //add offset to ^ to get virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr(); //convert ^ to pagetable raw pointer

    &mut *page_table_ptr //unsafe creation of page_table_ptr reference
}

/// This fn is unsafe bc caller must ensure all of the physical memory is  
/// mapped to virtual memory at the passed `physical_memory_offset` '.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> { //can only be called once
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)

}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { // because caller must ensure the frame is not already in use
        //FIXME: unsafe operation (remove!)
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}