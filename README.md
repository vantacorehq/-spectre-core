<div align="center">
  <img src="./banner.svg" alt="SPECTRE-CORE Banner" width="100%">
  
  <h1>SPECTRE-CORE</h1>
  <p><b>RUNTIME PROCESS MEMORY INJECTOR.</b></p>
  
  <img src="https://img.shields.io/badge/STATUS-OPERATIONAL-FF0000?style=for-the-badge&logo=target&logoColor=white" alt="Status">
  <img src="https://img.shields.io/badge/SYSCALL-process__vm__writev-000000?style=for-the-badge" alt="Syscall">
  <img src="https://img.shields.io/badge/OS-LINUX-8B0000?style=for-the-badge&logo=linux&logoColor=white" alt="OS">
  <img src="https://img.shields.io/badge/DEPENDENCIES-ZERO-000000?style=for-the-badge" alt="Dependencies">
</div>

---

## ☠️ THE MANIFESTO

Most security tools scan files. They hook network traffic. They operate on the surface. 

**SPECTRE-CORE goes deeper. It operates directly in the RAM.**

While corporate security teams are writing reports and recording apology videos, we are rewriting the variables of live processes. This is a zero-dependency, pure Rust implementation of a kernel-level memory injector. It bypasses standard debugging interfaces and directly modifies the state of any running application on the Linux system.

If it is loaded in memory, we own it.

## ⚡ CORE CAPABILITIES

*   **Direct Memory Access:** Uses `process_vm_readv` and `process_vm_writev` system calls to read and overwrite memory pages without triggering traditional `ptrace` anti-debugging checks.
*   **Map Validation:** Parses `/proc/[pid]/maps` in real-time to ensure target addresses are actually allocated, preventing fatal segmentation faults during injection.
*   **Zero Dependencies:** No external crates. No bloated frameworks. Pure `std`, raw `libc` FFI, and `unsafe` Rust.
*   **Silent Execution:** Modifies live variables on the fly. No need to pause the target process. The victim application will not even realize its memory has been compromised.

## 🏗️ ARCHITECTURE

```mermaid
graph TD
    A[SPECTRE-CORE] -->|Reads Maps| B(/proc/[PID]/maps)
    A -->|Validates Address| C{Is Mapped?}
    C -->|Yes| D[process_vm_writev Syscall]
    C -->|No| E[Abort Injection]
    D -->|Overwrites Bytes| F[Target Process RAM]
    F -->|Executes Modified State| G[Total Control]
    
    style A fill:#000,stroke:#FF0000,stroke-width:2px,color:#fff
    style D fill:#8B0000,stroke:#fff,stroke-width:1px,color:#fff
    style G fill:#000,stroke:#FF0000,stroke-width:2px,color:#fff



