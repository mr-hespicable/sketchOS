use x86_64::{structures::paging::PageTable, VirtAddr};

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
