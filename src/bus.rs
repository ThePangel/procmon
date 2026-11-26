pub trait Bus {
    fn read(&self, addr: u64) -> Option<u8>;
    fn write(&mut self, addr: u64, byte: u8);
    fn free(&mut self);
    fn run(&mut self, addr: u64);
}
