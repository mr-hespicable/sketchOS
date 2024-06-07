use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

/// Returns page table pointer reference to active level 4 table.
///
/// unsafe function bc if physical mem is not completely mapped
/// to virt mem at physical_mem_offset bad things happen.
///
/// WARNING: DO NOT CALL THIS FUNCTION MORE THAN ONCE.
/// DOING SO WILL CAUSE ALIASING &mut REFERENCES. (undefined) (thus bad)
pub unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_mem_offset)
}

///do docs
fn translate_addr_inner(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    // read level 4 frame from cr3 reg
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // traverse lvl 4 page table
    for &index in &table_indexes {
        // frame to page table ref
        let virt = physical_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported atm :3"),
        };
    }

    // calc physical address by adding page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
