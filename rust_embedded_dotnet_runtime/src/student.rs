#[repr(C)]
#[derive(Debug)]
pub struct Student {
    pub age: libc::c_int,
    pub name: *const u16,
}

#[no_mangle]
pub extern "C" fn nativeStudentNew(age: libc::c_int, name: *const u16) -> *mut Student {
    let student: Student = Student { age, name };
    Box::into_raw(Box::new(student))
}

#[no_mangle]
pub extern "C" fn nativeStudentDelete(student: *mut Student) {
    if !student.is_null() {
        unsafe { Box::from_raw(student) };
    }
}

#[no_mangle]
pub extern "C" fn nativeStudentSetAge(student: *mut Student, age: libc::c_int) {
    unsafe {
        if student.is_null() {
            panic!();
        }
        (*student).age = age;
    }
}

#[no_mangle]
pub extern "C" fn nativeStudentGetAge(student: *mut Student) -> libc::c_int {
    unsafe {
        if student.is_null() {
            panic!();
        }
        (*student).age
    }
}

#[no_mangle]
pub extern "C" fn nativeStudentSetName(student: *mut Student, name: *const u16) {
    unsafe {
        if student.is_null() {
            panic!();
        }
        (*student).name = name;
    }
}

#[no_mangle]
pub extern "C" fn nativeStudentGetName(student: *mut Student) -> *const u16 {
    unsafe {
        if student.is_null() {
            panic!();
        }
        (*student).name
    }
}
