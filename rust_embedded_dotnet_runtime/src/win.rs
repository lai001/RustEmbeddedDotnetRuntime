#[link(name = "kernel32")]
#[link(name = "User32")]
#[cfg(windows)]
extern "stdcall" {
    pub fn LoadLibraryA(lpFileName: *const u8) -> *const libc::c_void;
    pub fn LoadLibraryW(lpLibFileName: *const u16) -> *mut libc::c_void;
    pub fn GetProcAddress(hModule: *mut libc::c_void, lpProcName: *const u8) -> *mut libc::c_void;
    pub fn GetLastError() -> i64;
}
