use crate::student_func_ptr::StudentFuncPtr;

#[repr(C)]
#[derive(Debug)]
pub struct EntryInfo {
    pub file_watch_set_func_ptr: *const libc::c_void,
    pub args: *const *const u8,
    pub args_length: i32,
    pub student_func_ptr: StudentFuncPtr,
}
