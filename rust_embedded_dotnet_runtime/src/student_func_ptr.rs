use crate::student::{
    nativeStudentDelete, nativeStudentGetAge, nativeStudentGetName, nativeStudentNew,
    nativeStudentSetAge, nativeStudentSetName,
};

#[derive(Debug)]
pub struct StudentFuncPtr {
    native_student_new: *mut libc::c_void,
    native_student_delete: *mut libc::c_void,
    native_student_set_id: *mut libc::c_void,
    native_student_get_id: *mut libc::c_void,
    native_student_set_tag: *mut libc::c_void,
    native_student_get_tag: *mut libc::c_void,
}

impl StudentFuncPtr {
    pub fn new() -> StudentFuncPtr {
        StudentFuncPtr {
            native_student_new: nativeStudentNew as *mut libc::c_void,
            native_student_delete: nativeStudentDelete as *mut libc::c_void,
            native_student_set_id: nativeStudentSetAge as *mut libc::c_void,
            native_student_get_id: nativeStudentGetAge as *mut libc::c_void,
            native_student_set_tag: nativeStudentSetName as *mut libc::c_void,
            native_student_get_tag: nativeStudentGetName as *mut libc::c_void,
        }
    }
}

pub type EntryPointFn = unsafe extern "stdcall" fn(
    args: *const *const u8,
    length: i32,
    student_func_ptr: StudentFuncPtr,
);
