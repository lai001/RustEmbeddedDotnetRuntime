extern crate libc;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rust_embedded_dotnet_runtime::{
    dotnet::{
        get_dotnet_load_assembly, load_hostfxr_library, to_wstring,
        LoadAssemblyAndGetFunctionPointerFn,
    },
    entry_info::{EntryInfo, EntryPointFn},
    file::{nativeFileWatchSet, FileChangedFunc, GLOBAL_FILE_WATCH},
    student_func_ptr::StudentFuncPtr,
};
use std::{ffi::CString, path::Path, time::Duration};

fn main() {
    if let Ok(path) = std::env::current_dir() {
        println!("current_dir: {:?}", path);
        if path.ends_with("debug") == false {
            std::env::set_current_dir("./target/debug").unwrap();
        }
    }

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
            for events in rx {
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

    let config_path = "./AppWithPlugin.runtimeconfig.json".to_string();

    let load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path);
    unsafe {
        let load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerFn =
            std::mem::transmute(load_assembly_and_get_function_pointer);
        let mut entry_point_func: *mut libc::c_void = std::ptr::null_mut();
        const UNMANAGEDCALLERSONLY_METHOD: *const u16 = -1 as i16 as *const u16;
        let status = load_assembly_and_get_function_pointer(
            to_wstring("./AppWithPlugin.dll").as_ptr(),
            to_wstring("AppWithPlugin.Entry, AppWithPlugin").as_ptr(),
            to_wstring("Main").as_ptr(),
            UNMANAGEDCALLERSONLY_METHOD,
            std::ptr::null_mut(),
            &mut entry_point_func as *mut *mut libc::c_void,
        );
        let entry_point_func: *mut EntryPointFn = entry_point_func as *mut EntryPointFn;

        if status != 0 && entry_point_func.is_null() == false {
            panic!();
        }

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
