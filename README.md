<div align="center">
  <img src="./banner.svg" alt="SPECTRE-CORE Banner" width="100%">
  
  <h1>SPECTRE-CORE</h1>
  <p><b>RUNTIME PROCESS MEMORY INJECTOR.</b></p>
  
  <img src="https://img.shields.io/badge/STATUS-OPERATIONAL-FF0000?style=for-the-badge&logo=target" alt="Status">
  <img src="https://img.shields.io/badge/SYSCALL-process__vm__writev-000000?style=for-the-badge" alt="Syscall">
  <img src="https://img.shields.io/badge/OS-LINUX-550000?style=for-the-badge" alt="OS">
</div>

---

## ☠️ WHAT IS SPECTRE-CORE?

Most security tools try to intercept network traffic or scan files. They operate on the surface. 

**SPECTRE-CORE goes deeper.** It operates directly in the RAM.

This is a zero-dependency, pure Rust implementation of a process memory injector. It uses the `process_vm_readv` and `process_vm_writev` Linux syscalls to read and overwrite the memory of a live process without triggering standard debugging stops. 

If you know the memory address (or if you scan for it), you can change the variables of any running application on the system.

## ⚡ CORE CAPABILITIES

*   **Direct Memory Access:** Reads and writes to `/proc/[pid]/mem` using kernel syscalls.
*   **Memory Map Validation:** Parses `/proc/[pid]/maps` to ensure the target address is actually allocated, preventing segfaults.
*   **Bypass Standard Protections:** Does not rely on `ptrace`, meaning basic anti-debugging checks might not flag it immediately.
*   **Zero Dependencies:** No external crates. Pure `std` and `libc` FFI.

## 🛠️ USAGE (For Authorized Red Teaming Only)

> ⚠️ **WARNING:** This tool modifies the memory of live processes. Doing this to system-critical processes will crash the OS. Use only on your own applications or in controlled environments.

```bash
# 1. Find the PID of the target process
pgrep -f target_app

# 2. Find the memory address of the variable you want to change
# (Use /proc/[pid]/maps or a debugger to find the address)

# 3. Compile and run SPECTRE
rustc -O main.rs -o spectre
sudo ./spectre <PID> <HEX_ADDRESS> <HEX_VALUE>

# Example: Change a 32-bit integer at address 0x7fff... to 999
sudo ./spectre 1234 0x7fff5fbff8c0 0x000003E7
