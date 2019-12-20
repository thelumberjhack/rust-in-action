extern crate kernel32;
extern crate winapi;

use winapi::shared::minwindef::{
    DWORD,
    HANDLE,
    LPVOID,
    PVOID,
    SIZE_T,
    LPSYSTEM_INFO,
    SYSTEM_INFO,
    MEMORY_BASIC_INFORMATION,
};

fn main() {
    // Following variables will be used in unsafe blocks so we need to declare
    // them here to access them outside of their scope.
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_app_addr: LPVOID;
    let max_app_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMORY_BASIC_INFORMATION;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    // this block guarantees that all memory is initialized
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // This block of code is where system calls are made
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };

    min_app_addr = proc_info.lpMinimumApplicationAddress;
    max_app_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_app_addr, max_app_addr);

    loop {
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
