
pub fn init() {
    println!("[MEM] Memory management initialized");
    setup_page_table();
    allocate_heap();
}
