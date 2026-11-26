#[derive(Debug)]
pub struct Region {
    pub start: u64,
    pub end: u64,
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub name: String,
}

pub trait ProcBus {
    fn regions(&self) -> Vec<Region>;
    fn read(&self, addr: u64) -> Option<u8>;
    fn write(&mut self, addr: u64, byte: u8);
    fn drop(&mut self);
}
