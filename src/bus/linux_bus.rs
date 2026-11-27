use std::{
    ffi::c_void,
    fs::{File, OpenOptions, read_to_string},
    io::{self},
    os::unix::fs::FileExt,
    ptr,
};

use super::{Bus, ProcBus, Region};
use libc::{
    MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, munmap,
};
pub struct LinuxBus {
    pub mem_size: usize,
    pub memory_ptr: *mut u8,
}

impl LinuxBus {
    pub fn alloc(mem_size: usize) -> Self {
        let memory_ptr = unsafe {
            let ptr = mmap(
                ptr::null_mut(),
                mem_size,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            if ptr == MAP_FAILED {
                panic!("mmap failed")
            }
            ptr as *mut u8
        };
        Self {
            mem_size,
            memory_ptr,
        }
    }
}

impl Bus for LinuxBus {
    fn read(&self, addr: u64) -> Option<u8> {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return None;
        }

        Some(unsafe { std::ptr::read_volatile(self.memory_ptr.add(addr as usize)) })
    }
    fn write(&mut self, addr: u64, byte: u8) {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return;
        }

        unsafe { std::ptr::write_volatile(self.memory_ptr.add(addr as usize), byte) }
    }

    fn free(&mut self) {
        unsafe {
            munmap(self.memory_ptr as *mut c_void, self.mem_size);
        }
    }
    fn run(&mut self, addr: u64) {
        if addr > self.mem_size as u64 {
            println!("INVALID ADDRESS: Address is out of bounds");
            return;
        }

        let code: extern "C" fn() -> i32 =
            unsafe { std::mem::transmute(self.memory_ptr.wrapping_add(addr as usize)) };
        let result = code();

        println!("Code executed with result: {}", result);
    }
}

pub struct LinuxProcBus {
    mem: File,
    pid: u32,
}

impl LinuxProcBus {
    pub fn attach(pid: u32) -> Self {
        let mem = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/proc/{pid}/mem"))
            .unwrap();

        LinuxProcBus { mem, pid }
    }
}

impl ProcBus for LinuxProcBus {
    fn regions(&self) -> io::Result<Vec<Region>> {
        let pid = self.pid;
        let mut regions = Vec::new();
        let lines = read_to_string(format!("/proc/{pid}/maps"))?;

        for line in lines.lines() {
            let mut parts = line.split_whitespace();
            let Some((start_str, end_str)) = parts.next().unwrap_or("").split_once("-") else {
                continue;
            };
            let Ok(start) = u64::from_str_radix(start_str, 16) else {
                continue;
            };
            let Ok(end) = u64::from_str_radix(end_str, 16) else {
                continue;
            };

            let permissions = parts.next().unwrap_or("");
            let name = parts.nth(3).unwrap_or("").to_string();

            regions.push(Region {
                start,
                end,
                read: permissions.contains("r"),
                write: permissions.contains("w"),
                exec: permissions.contains("x"),
                name,
            });
        }

        Ok(regions)
    }
    fn read(&self, addr: u64) -> Option<u8> {
        let mut byte_buf = [0u8; 1];
        self.mem.read_at(&mut byte_buf, addr).ok()?;
        Some(byte_buf[0])
    }
    fn write(&mut self, addr: u64, byte: u8) {
        let byte_buf = [byte];
        if !self.mem.write_at(&byte_buf, addr).is_ok() {
            println!("Failed to write {:02X} at {:016X}", byte, addr);
        }
    }
    fn drop(&mut self) {}
}
