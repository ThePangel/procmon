use crate::bus::Bus;
use std::ffi::c_void;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc,
};

pub struct WinBus {
    pub mem_size: usize,
}

impl Bus for WinBus {
    fn read(&self, memory_ptr: *mut u8, addr: u64) -> Option<u8> {
        if addr >= self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return None;
        }

        Some(unsafe { std::ptr::read_volatile(memory_ptr.add(addr as usize)) })
    }

    fn write(&mut self, memory_ptr: *mut u8, addr: u64, byte: u8) {
        if addr >= self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return;
        }

        unsafe { std::ptr::write_volatile(memory_ptr.add(addr as usize), byte) }
    }

    fn alloc(&mut self) -> *mut u8 {
        unsafe {
            let ptr = VirtualAlloc(
                None,
                self.mem_size,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_EXECUTE_READWRITE,
            );
            if ptr.is_null() {
                panic!("Failed to allocate memory");
            }
            ptr as *mut u8
        }
    }

    fn free(&mut self, memory_ptr: *mut u8) {
        unsafe {
            windows::Win32::System::Memory::VirtualFree(
                memory_ptr as *mut c_void,
                0,
                windows::Win32::System::Memory::MEM_RELEASE,
            )
            .expect("VirtualFree failed");
        }
    }
    fn run(&mut self, memory_ptr: *mut u8, addr: u64) {
        if addr >= self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return;
        }

        let code: extern "C" fn() -> i32 =
            unsafe { std::mem::transmute(memory_ptr.wrapping_add(addr as usize)) };
        let result = code();

        println!("Code executed with result: {}", result);
    }
}
