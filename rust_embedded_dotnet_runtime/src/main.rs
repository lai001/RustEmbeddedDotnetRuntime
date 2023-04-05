extern crate libc;
use rust_embedded_dotnet_runtime::{
    dotnet::{
        get_dotnet_load_assembly, load_hostfxr_library, to_wstring,
        LoadAssemblyAndGetFunctionPointerFn,
    },
    student_func_ptr::{EntryPointFn, StudentFuncPtr},
};
use std::ffi::CString;

fn main() {
    if let Ok(path) = std::env::current_dir() {
        println!("current_dir: {:?}", path);
        if path.ends_with("debug") == false {
            std::env::set_current_dir("./target/debug").unwrap();
        }
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

        entry_point_func(c_args.as_ptr(), c_args.len() as i32, student_func_ptr);
    }
}
