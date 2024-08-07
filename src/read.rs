use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.
// requires a proper handle with reading rights, can use the windows_memory_access::handle::get_read_only_handle(pid); to
// simplify getting handles and windows_memory_access::handle::close_handle(handle); to close.

pub fn utf16_string(handle: HANDLE, offset: u32) -> Result<String, Error> {
    let mut buffer = [0u16; 100];

    unsafe {
        ReadProcessMemory(
            handle,
            offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
        .map(|_| {
            let utf16_array = buffer
                .into_iter()
                .take_while(|&c| c != 0)
                .collect::<Vec<u16>>();

            String::from_utf16_lossy(&utf16_array)
        })
    }
}

pub fn u64_bytes(handle: HANDLE, offset: u32) -> Result<[u8; 8], Error> {
    let mut buffer = [0u8; 8];

    unsafe {
        ReadProcessMemory(
            handle,
            offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(buffer)
}

pub fn u32_bytes(handle: HANDLE, offset: u32) -> Result<[u8; 4], Error> {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(buffer)
}

pub fn u16_bytes(handle: HANDLE, offset: u32) -> Result<[u8; 2], Error> {
    let mut buffer = [0u8; 2];

    unsafe {
        ReadProcessMemory(
            handle,
            offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(buffer)
}

pub fn u8_bytes(handle: HANDLE, offset: u32) -> Result<[u8; 1], Error> {
    let mut buffer = [0u8; 1];

    unsafe {
        ReadProcessMemory(
            handle,
            offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(buffer)
}
