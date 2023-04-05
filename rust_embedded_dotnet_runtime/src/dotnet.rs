use crate::win::GetProcAddress;
use crate::win::LoadLibraryW;
use std::ffi::CString;
use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;
use std::sync::Mutex;

#[link(name = "./target/debug/nethost")]
#[cfg(windows)]
extern "stdcall" {
    pub fn get_hostfxr_path(
        buffer: *mut u16,
        buffer_size: *mut std::os::raw::c_ulonglong,
        parameters: *const libc::c_void,
    ) -> std::os::raw::c_int;
}

pub type HostfxrInitializeForRuntimeConfigFn = unsafe extern "cdecl" fn(
    runtime_config_path: *const u16,
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
    assembly_path: *const u16,
    type_name: *const u16,
    method_name: *const u16,
    delegate_type_name: *const u16,
    reserved: *mut libc::c_void,
    delegate: *mut *mut libc::c_void,
) -> std::ffi::c_int;

#[cfg(windows)]
pub fn to_wstring(str: &str) -> Vec<u16> {
    let v: Vec<u16> = OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    v
}

#[cfg(windows)]
pub fn get_func_ptr(h: *mut libc::c_void, name: &str) -> *mut libc::c_void {
    unsafe {
        let s = CString::new(name).unwrap().into_bytes();
        GetProcAddress(h, s.as_ptr())
    }
}

#[cfg(windows)]
pub fn load_hostfxr_library() -> bool {
    unsafe {
        const MAX_PATH: std::os::raw::c_ulonglong = 260;
        let mut buffer: Vec<u16> = vec![0; MAX_PATH.try_into().unwrap()];
        let mut buffer_size = MAX_PATH;

        let status = get_hostfxr_path(
            buffer.as_mut_ptr(),
            &mut buffer_size as *mut std::os::raw::c_ulonglong,
            std::ptr::null(),
        );
        if status != 0 {
            return false;
        }

        if let Ok(path) = String::from_utf16(&buffer) {
            println!(
                "hostfxr library path: {:?}",
                path.trim_matches(char::from(0))
            );
        }
        let lib = LoadLibraryW(buffer.as_ptr());
        let mut context = GLOBAL_CONTEXT.lock().unwrap();
        context.initialize_for_runtime_config_func_ptr =
            std::mem::transmute(get_func_ptr(lib, "hostfxr_initialize_for_runtime_config"));

        context.get_runtime_delegate_func_ptr =
            std::mem::transmute(get_func_ptr(lib, "hostfxr_get_runtime_delegate"));
        context.close_func_ptr = std::mem::transmute(get_func_ptr(lib, "hostfxr_close"));

        context.initialize_for_runtime_config_func_ptr != std::ptr::null_mut()
            && context.close_func_ptr != std::ptr::null_mut()
            && context.get_runtime_delegate_func_ptr != std::ptr::null_mut()
    }
}

pub fn get_dotnet_load_assembly(config_path: String) -> *mut LoadAssemblyAndGetFunctionPointerFn {
    let mut load_assembly_and_get_function_pointer: *mut libc::c_void = std::ptr::null_mut();
    let mut host_context_handle: *mut libc::c_void = std::ptr::null_mut();
    unsafe {
        let context = GLOBAL_CONTEXT.lock().unwrap();
        let init_fptr: HostfxrInitializeForRuntimeConfigFn =
            std::mem::transmute(context.initialize_for_runtime_config_func_ptr);

        let mut status = init_fptr(
            to_wstring(config_path.as_str()).as_ptr(),
            std::ptr::null(),
            &mut host_context_handle as *mut *mut libc::c_void,
        );
        if status != 0 {
            panic!();
        }

        let get_delegate_fptr: HostfxrGetRuntimeDelegateFn =
            std::mem::transmute(context.get_runtime_delegate_func_ptr);

        status = get_delegate_fptr(
            host_context_handle,
            5,
            &mut load_assembly_and_get_function_pointer as *mut *mut libc::c_void,
        );

        if status != 0 {
            panic!();
        }

        let close_fptr: HostfxrCloseFn = std::mem::transmute(context.close_func_ptr);
        close_fptr(host_context_handle);
    }

    load_assembly_and_get_function_pointer as *mut LoadAssemblyAndGetFunctionPointerFn
}

pub struct Context {
    pub initialize_for_runtime_config_func_ptr: *mut HostfxrInitializeForRuntimeConfigFn,
    pub get_runtime_delegate_func_ptr: *mut HostfxrGetRuntimeDelegateFn,
    pub close_func_ptr: *mut HostfxrCloseFn,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

lazy_static! {
    pub static ref GLOBAL_CONTEXT: Mutex<Context> = Mutex::new(Context {
        initialize_for_runtime_config_func_ptr: std::ptr::null_mut(),
        get_runtime_delegate_func_ptr: std::ptr::null_mut(),
        close_func_ptr: std::ptr::null_mut(),
    });
}
