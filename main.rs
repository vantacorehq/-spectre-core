// SPECTRE-CORE - Runtime Process Memory Injector
// Compile: rustc -O main.rs -o spectre
// Run: sudo ./spectre <PID> <HEX_ADDRESS> <HEX_VALUE>

use std::env;
use std::io;
use std::os::raw::{c_int, c_void, c_ulong};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Linux Syscall numbers (x86_64)
const SYS_PROCESS_VM_READV: c_ulong = 310;
const SYS_PROCESS_VM_WRITEV: c_ulong = 311;

#[repr(C)]
struct Iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

extern "C" {
    fn syscall(num: c_ulong, ...) -> i64;
}

/// Reads memory from a target process using process_vm_readv syscall.
fn read_process_memory(pid: c_int, address: usize, buffer: &mut [u8]) -> io::Result<()> {
    let local_iov = Iovec {
        iov_base: buffer.as_mut_ptr() as *mut c_void,
        iov_len: buffer.len(),
    };
    let remote_iov = Iovec {
        iov_base: address as *mut c_void,
        iov_len: buffer.len(),
    };

    let ret = unsafe {
        syscall(
            SYS_PROCESS_VM_READV,
            pid,
            &local_iov as *const _,
            1 as c_ulong,
            &remote_iov as *const _,
            1 as c_ulong,
            0 as c_ulong,
        )
    };

    if ret < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Overwrites memory in a target process using process_vm_writev syscall.
fn write_process_memory(pid: c_int, address: usize, data: &[u8]) -> io::Result<()> {
    let local_iov = Iovec {
        iov_base: data.as_ptr() as *mut c_void,
        iov_len: data.len(),
    };
    let remote_iov = Iovec {
        iov_base: address as *mut c_void,
        iov_len: data.len(),
    };

    let ret = unsafe {
        syscall(
            SYS_PROCESS_VM_WRITEV,
            pid,
            &local_iov as *const _,
            1 as c_ulong,
            &remote_iov as *const _,
            1 as c_ulong,
            0 as c_ulong,
        )
    };

    if ret < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Parses /proc/[pid]/maps to ensure the target address is mapped and writable.
/// Prevents segfaults and invalid memory access.
fn is_address_mapped(pid: c_int, address: usize) -> bool {
    let maps_path = format!("/proc/{}/maps", pid);
    if let Ok(file) = File::open(&maps_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                let range: Vec<&str> = parts[0].split('-').collect();
                if range.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        usize::from_str_radix(range[0], 16),
                        usize::from_str_radix(range[1], 16),
                    ) {
                        if address >= start && address < end {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("[!] Usage: sudo {} <PID> <HEX_ADDRESS> <HEX_VALUE>", args[0]);
        eprintln!("[!] Example: sudo {} 1337 0x7fff5fbff8c0 0xDEADBEEF", args[0]);
        std::process::exit(1);
    }

    let pid: c_int = args[1].parse().expect("[-] Invalid PID");
    let address = usize::from_str_radix(args[2].trim_start_matches("0x"), 16)
        .expect("[-] Invalid Memory Address");
    let value = u32::from_str_radix(args[3].trim_start_matches("0x"), 16)
        .expect("[-] Invalid Value");

    println!("[SPECTRE] Initializing memory injector...");
    println!("[SPECTRE] Target PID: {}", pid);
    println!("[SPECTRE] Target Address: 0x{:x}", address);

    if !is_address_mapped(pid, address) {
        eprintln!("[!] Address 0x{:x} is not mapped in process {}", address, pid);
        eprintln!("[!] Aborting injection to prevent segmentation fault.");
        std::process::exit(1);
    }

    // 1. Read current value
    let mut buffer = [0u8; 4];
    if let Err(e) = read_process_memory(pid, address, &mut buffer) {
        eprintln!("[!] Failed to read target memory: {}", e);
        std::process::exit(1);
    }
    
    let current_val = u32::from_ne_bytes(buffer);
    println!("[SPECTRE] Current value at address: 0x{:x}", current_val);

    // 2. Overwrite memory
    let new_data = value.to_ne_bytes();
    if let Err(e) = write_process_memory(pid, address, &new_data) {
        eprintln!("[!] Failed to write to target memory: {}", e);
        std::process::exit(1);
    }
    
    println!("[SPECTRE] Injecting new value: 0x{:x}", value);

    // 3. Verify injection
    let mut check_buf = [0u8; 4];
    read_process_memory(pid, address, &mut check_buf)?;
    let check_val = u32::from_ne_bytes(check_buf);
    
    if check_val == value {
        println!("[SUCCESS] Memory injection complete. Process is now compromised.");
    } else {
        eprintln!("[!] Memory overwrite failed. Target may have restored the value.");
    }

    Ok(())
}
   
      
   


        
    
    
      
