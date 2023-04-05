use std::sync::Mutex;

pub type FileChangedFunc = unsafe extern "stdcall" fn();

pub struct FileWatch {
    pub file_changed_func: *const FileChangedFunc,
}

unsafe impl Send for FileWatch {}
unsafe impl Sync for FileWatch {}

lazy_static! {
    pub static ref GLOBAL_FILE_WATCH: Mutex<FileWatch> = Mutex::new(FileWatch {
        file_changed_func: std::ptr::null_mut()
    });
}

#[no_mangle]
pub extern "C" fn nativeFileWatchSet(func: *const FileChangedFunc) {
    GLOBAL_FILE_WATCH.lock().unwrap().file_changed_func = func;
}
