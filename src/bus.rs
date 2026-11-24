pub trait Bus {
    fn read(&self, memory_ptr: *mut u8, addr: u64) -> Option<u8>;
    fn write(&mut self, memory_ptr: *mut u8, addr: u64, byte: u8);
    fn alloc(&mut self) -> *mut u8;
    fn free(&mut self, memory_ptr: *mut u8);
    fn run(&mut self, memory_ptr: *mut u8, addr: u64);
}
