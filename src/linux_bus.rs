use std::{ffi::c_void, ptr};

use crate::{
    Bus,
    proc_bus::{ProcBus, Region},
};
use libc::{
    MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, munmap,
};
pub struct LinuxBus {
    pub mem_size: usize,
}

impl Bus for LinuxBus {
    fn read(&self, memory_ptr: *mut u8, addr: u64) -> Option<u8> {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return None;
        }

        Some(unsafe { std::ptr::read_volatile(memory_ptr.add(addr as usize)) })
    }
    fn write(&mut self, memory_ptr: *mut u8, addr: u64, byte: u8) {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address isregions of bounds");
            return;
        }

        unsafe { std::ptr::write_volatile(memory_ptr.add(addr as usize), byte) }
    }

    fn alloc(&mut self) -> *mut u8 {
        unsafe {
            let ptr = mmap(
                ptr::null_mut(),
                self.mem_size,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            if ptr == MAP_FAILED {
                panic!("mmap failed")
            }
            ptr as *mut u8
        }
    }
    fn free(&mut self, memory_ptr: *mut u8) {
        unsafe {
            munmap(memory_ptr as *mut c_void, self.mem_size);
        }
    }
    fn run(&mut self, memory_ptr: *mut u8, addr: u64) {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return;
        }

        let code: extern "C" fn() -> i32 =
            unsafe { std::mem::transmute(memory_ptr.wrapping_add(addr as usize)) };
        let result = code();

        println!("Code executed with result: {}", result);
    }
}

pub struct LinuxProcBus {}

impl ProcBus for LinuxProcBus {
    fn regions(&self) -> Vec<Region> {
        todo!()
    }
    fn read(&self, addr: u64) -> Option<u8> {
        todo!()
    }
    fn write(&mut self, addr: u64, byte: u8) {
        todo!()
    }
    fn drop(&mut self) {
        todo!()
    }
}
