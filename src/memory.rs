use x86_64::{
    structures::paging::{frame, Page, PageTable},
    PhysAddr, VirtAddr,
};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table, _) = Cr3::read(); //read lvl 4 reg 'CR3'

    let phys: PhysAddr = level_4_table.start_address(); //grab physical start address
    let virt: VirtAddr = physical_memory_offset + phys.as_u64(); //add offset to ^ to get virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr(); //convert ^ to pagetable raw pointer

    &mut *page_table_ptr //unsafe creation of page_table_ptr reference
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_table_frame, _) = Cr3::read(); //read lvl 4 reg 'CR3'

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];

    let mut frame = level_4_table_frame; //remember last physical frame

    //traverse the page tables
    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        //read entry and update frame var
        let entry = &table[index];

        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        }
    }
    //calculate & return physical address
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
