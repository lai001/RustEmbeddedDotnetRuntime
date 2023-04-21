use crate::entry_info::EntryPointFn;
use std::{ffi::CString, path::PathBuf};

#[link(name = "nethost")]
extern "C" {
    pub fn get_hostfxr_path(
        buffer: *mut u8,
        buffer_size: *mut std::os::raw::c_ulonglong,
        parameters: *const libc::c_void,
    ) -> std::os::raw::c_int;
}

pub type HostfxrInitializeForRuntimeConfigFn = unsafe extern "cdecl" fn(
    runtime_config_path: *const u8,
    parameters: *const libc::c_void,
    host_context_handle: *mut *mut libc::c_void,
) -> std::ffi::c_int;

pub type HostfxrGetRuntimeDelegateFn = unsafe extern "cdecl" fn(
    host_context_handle: *const libc::c_void,
    r#type: std::ffi::c_int,
    delegate: *mut *mut libc::c_void,
) -> std::ffi::c_int;

pub type HostfxrCloseFn =
    unsafe extern "cdecl" fn(host_context_handle: *const libc::c_void) -> std::ffi::c_int;

pub type LoadAssemblyAndGetFunctionPointerFn = unsafe extern "stdcall" fn(
    assembly_path: *const u8,
    type_name: *const u8,
    method_name: *const u8,
    delegate_type_name: *const u8,
    reserved: *mut libc::c_void,
    delegate: *mut *mut libc::c_void,
) -> std::ffi::c_int;

fn get_hostfxr_library_path() -> Option<String> {
    unsafe {
        const MAX_PATH: std::os::raw::c_ulonglong = 260;

        let mut buffer: Vec<u8> = vec![0; MAX_PATH.try_into().unwrap()];
        let mut buffer_size = MAX_PATH;

        let status = get_hostfxr_path(
            buffer.as_mut_ptr(),
            &mut buffer_size as *mut std::os::raw::c_ulonglong,
            std::ptr::null(),
        );

        if status != 0 {
            return None;
        }

        if let Ok(path) = String::from_utf8(buffer.clone()) {
            return Some(path);
        }
        return None;
    }
}

pub fn load_hostfxr_library() -> bool {
    true
}

pub fn get_dotnet_load_assembly(config_path: String) -> *mut LoadAssemblyAndGetFunctionPointerFn {
    let mut load_assembly_and_get_function_pointer: *mut libc::c_void = std::ptr::null_mut();
    let mut host_context_handle: *mut libc::c_void = std::ptr::null_mut();
    unsafe {
        let c_str = CString::new(config_path.clone()).unwrap();
        let hostfxr_library_path = get_hostfxr_library_path().unwrap();
        let hostfxr_library_path = hostfxr_library_path.trim_matches(char::from(0));
        let lib = libloading::Library::new(hostfxr_library_path).unwrap();
        let init_fptr: libloading::Symbol<HostfxrInitializeForRuntimeConfigFn> =
            lib.get(b"hostfxr_initialize_for_runtime_config").unwrap();
        let status = init_fptr(
            c_str.as_bytes().as_ptr(),
            std::ptr::null(),
            &mut host_context_handle,
        );
        if status != 0 {
            panic!();
        }
        let get_delegate_fptr: libloading::Symbol<HostfxrGetRuntimeDelegateFn> =
            lib.get(b"hostfxr_get_runtime_delegate").unwrap();
        let status = get_delegate_fptr(
            host_context_handle,
            5,
            &mut load_assembly_and_get_function_pointer as *mut *mut libc::c_void,
        );
        if status != 0 {
            panic!();
        }
        let close_fptr: libloading::Symbol<HostfxrCloseFn> = lib.get(b"hostfxr_close").unwrap();
        close_fptr(host_context_handle);
    }
    load_assembly_and_get_function_pointer as *mut LoadAssemblyAndGetFunctionPointerFn
}

pub fn get_current_exe_dir() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    let current_exe_dir = std::path::Path::new(&current_exe)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    PathBuf::from(current_exe_dir)
}

pub fn get_entry_point_func() -> *mut EntryPointFn {
    const UNMANAGEDCALLERSONLY_METHOD: *const u8 = -1 as i8 as *const u8;
    let mut entry_point_func: *mut libc::c_void = std::ptr::null_mut();
    let config_path = "./AppWithPlugin.runtimeconfig.json".to_string();
    let load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path);
    unsafe {
        let load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerFn =
            std::mem::transmute(load_assembly_and_get_function_pointer);
        let assembly_path = CString::new(
            get_current_exe_dir()
                .join("AppWithPlugin.dll")
                .to_str()
                .unwrap(),
        )
        .unwrap();
        let type_name = CString::new("AppWithPlugin.Entry, AppWithPlugin").unwrap();
        let method_name = CString::new("Main").unwrap();
        let status = load_assembly_and_get_function_pointer(
            assembly_path.as_bytes().as_ptr(),
            type_name.as_bytes().as_ptr(),
            method_name.as_bytes().as_ptr(),
            UNMANAGEDCALLERSONLY_METHOD,
            std::ptr::null_mut(),
            &mut entry_point_func as *mut *mut libc::c_void,
        );
        let entry_point_func: *mut EntryPointFn = entry_point_func as *mut EntryPointFn;
        if status != 0 && entry_point_func.is_null() == false {
            panic!();
        }
        return entry_point_func;
    }
}
