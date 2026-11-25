use crate::{
    bus::Bus,
    proc_bus::{ProcBus, Region},
};
use std::ffi::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Diagnostics::Debug::{FlushInstructionCache, ReadProcessMemory, WriteProcessMemory},
    System::Memory::{
        MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualAlloc,
        VirtualProtectEx,
    },
    System::Threading::{
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ,
        PROCESS_VM_WRITE,
    },
};

pub struct WinBus {
    pub mem_size: usize,
}

impl Bus for WinBus {
    fn read(&self, memory_ptr: *mut u8, addr: u64) -> Option<u8> {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return None;
        }

        Some(unsafe { std::ptr::read_volatile(memory_ptr.add(addr as usize)) })
    }

    fn write(&mut self, memory_ptr: *mut u8, addr: u64, byte: u8) {
        if addr > self.mem_size as u64 {
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

pub struct WinProcBus {
    h: HANDLE,
    pub pid: u32,
}

impl WinProcBus {
    pub fn attach(pid: u32) -> Self {
        let h = unsafe {
            OpenProcess(
                PROCESS_VM_READ
                    | PROCESS_VM_WRITE
                    | PROCESS_VM_OPERATION
                    | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            )
            .unwrap()
        };
        WinProcBus { h, pid }
    }
}

impl ProcBus for WinProcBus {
    fn read(&self, addr: u64) -> Option<u8> {
        let mut byte: u8 = 0;
        let mut n_bytes: usize = 0;

        let r = unsafe {
            ReadProcessMemory(
                self.h,
                addr as *const c_void,
                &mut byte as *mut u8 as *mut c_void,
                1,
                Some(&mut n_bytes),
            )
        };
        if r.is_ok() && n_bytes == 1 {
            Some(byte)
        } else {
            None
        }
    }

    fn write(&mut self, addr: u64, byte: u8) {
        let pointer = addr as *const c_void;

        let mut old = PAGE_PROTECTION_FLAGS(0);

        unsafe {
            if VirtualProtectEx(self.h, pointer, 1, PAGE_EXECUTE_READWRITE, &mut old).is_err() {
                return;
            }

            let mut n_bytes: usize = 0;

            WriteProcessMemory(
                self.h,
                pointer,
                &byte as *const u8 as *const c_void,
                1,
                Some(&mut n_bytes),
            )
            .unwrap();

            let mut tmp = PAGE_PROTECTION_FLAGS(0);
            let _ = VirtualProtectEx(self.h, pointer, 1, old, &mut tmp);

            let _ = FlushInstructionCache(self.h, Some(pointer), 1);
        }
    }
    fn run(&mut self, addr: u64) {}
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.h);
        }
    }
    fn regions(&self) -> Vec<Region> {
        todo!();
        Vec::new()
    }
}
