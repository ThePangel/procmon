pub trait Bus {
    fn read(&self, addr: u64) -> Option<u8>;
    fn write(&mut self, addr: u64, byte: u8);
    fn free(&mut self);
    fn run(&mut self, addr: u64);
}

mod proc_bus;
pub use proc_bus::{ProcBus, Region};

#[cfg(target_os = "linux")]
pub mod linux_bus;

#[cfg(target_os = "windows")]
pub mod win_bus;
