extern crate winapi;
use std::io::{self, Error, ErrorKind};
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID};
use winapi::um::fileapi::{CreateFileW, ReadFile, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::{GENERIC_READ, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_ATTRIBUTE_NORMAL};

fn main() -> io::Result<()> {
    // Open the raw disk (e.g., \\.\PhysicalDrive0 for the first physical drive)
    let disk_path = r"\\.\PhysicalDrive0";
    let disk_handle = unsafe {
        CreateFileW(
            disk_path.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };

    if disk_handle == INVALID_HANDLE_VALUE {
        return Err(Error::new(ErrorKind::Other, "Failed to open raw disk"));
    }

    let mut buffer = [0u8; 512];  // Adjust buffer size as needed
    let mut bytes_read: DWORD = 0;

    // Read from the raw disk
    let read_result = unsafe {
        ReadFile(
            disk_handle,
            buffer.as_mut_ptr() as LPVOID,
            buffer.len() as DWORD,
            &mut bytes_read,
            null_mut(),
        )
    };

    if read_result == FALSE {
        return Err(Error::last_os_error());
    }

    println!("Read {} bytes from the disk", bytes_read);
    println!("Data: {:?}", &buffer[..bytes_read as usize]);

    // Close the disk handle
    unsafe { winapi::um::handleapi::CloseHandle(disk_handle) };

    Ok(())
}
