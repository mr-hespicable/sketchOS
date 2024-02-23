use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table, _) = Cr3::read(); //read lvl 4 reg 'CR3'

    let phys: PhysAddr = level_4_table.start_address(); //grab physical start address
    let virt: VirtAddr = physical_memory_offset + phys.as_u64(); //add offset to ^ to get virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr(); //convert ^ to pagetable raw pointer

    &mut *page_table_ptr //unsafe creation of page_table_ptr reference
}
