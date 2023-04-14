extern crate libc;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rust_embedded_dotnet_runtime::{
    entry_info::{EntryInfo, EntryPointFn},
    file::{nativeFileWatchSet, FileChangedFunc, GLOBAL_FILE_WATCH},
    platform::dotnet::{get_entry_point_func, load_hostfxr_library},
    student_func_ptr::StudentFuncPtr,
};
use std::{
    ffi::CString,
    path::Path,
    time::{Duration, SystemTime},
};

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n < 3 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

fn test() {
    let current = SystemTime::now();
    for i in 0..45 {
        fib(i);
    }
    let duration = SystemTime::now()
        .duration_since(current)
        .unwrap()
        .as_millis();
    println!("{duration}");
}

fn main() {
    if let (Ok(current_dir), Ok(current_exe)) = (std::env::current_dir(), std::env::current_exe()) {
        let current_exe_dir = std::path::Path::new(&current_exe)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        let current_dir = current_dir.to_str().unwrap();
        if current_dir != current_exe_dir {
            std::env::set_current_dir(current_exe_dir).unwrap();
        }
    }
    test();
    {
        std::thread::spawn(|| {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx).unwrap();
            debouncer
                .watcher()
                .watch(
                    Path::new("../../../AppWithPlugin/HelloPlugin"),
                    RecursiveMode::NonRecursive,
                )
                .unwrap();
            for _ in rx {
                let file_changed_func = GLOBAL_FILE_WATCH.lock().unwrap().file_changed_func;
                if file_changed_func.is_null() == false {
                    unsafe {
                        let file_changed_func: FileChangedFunc =
                            std::mem::transmute(file_changed_func);
                        file_changed_func();
                    };
                }
            }
        });
    }

    let args: Vec<String> = std::env::args().collect();
    let mut c_str_args: Vec<CString> = vec![];
    let mut c_args: Vec<*const u8> = vec![];

    for arg in args {
        c_str_args.push(CString::new(arg.as_str()).unwrap());
    }

    for c_str_arg in &c_str_args {
        let arg = c_str_arg.as_ptr();
        c_args.push(arg as *const u8);
    }

    if !load_hostfxr_library() {
        panic!();
    }

    let entry_point_func = get_entry_point_func();
    unsafe {
        let student_func_ptr = StudentFuncPtr::new();
        let entry_point_func: EntryPointFn = std::mem::transmute(entry_point_func);

        let entry_info = EntryInfo {
            file_watch_set_func_ptr: nativeFileWatchSet as *const libc::c_void,
            args: c_args.as_ptr(),
            args_length: c_args.len() as i32,
            student_func_ptr,
        };
        entry_point_func(entry_info);
    }
}
