use super::win::get_func_ptr;
use super::win::to_wstring;
use super::win::LoadLibraryW;
use crate::entry_info::EntryPointFn;
use crate::windows::global_context::GLOBAL_CONTEXT;

// #[link(name = "./target/debug/nethost")]
#[link(name = "../../../target/release/nethost")]
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

fn get_host_context_handle(
    init_fptr: HostfxrInitializeForRuntimeConfigFn,
    config_path: String,
    host_context_handle: *mut *mut libc::c_void,
) {
    unsafe {
        let status = init_fptr(
            to_wstring(config_path.as_str()).as_ptr(),
            std::ptr::null(),
            host_context_handle,
        );
        if status != 0 {
            panic!();
        }
    }
}

pub fn get_dotnet_load_assembly(config_path: String) -> *mut LoadAssemblyAndGetFunctionPointerFn {
    let mut load_assembly_and_get_function_pointer: *mut libc::c_void = std::ptr::null_mut();
    let mut host_context_handle: *mut libc::c_void = std::ptr::null_mut();
    unsafe {
        let context = GLOBAL_CONTEXT.lock().unwrap();
        let init_fptr: HostfxrInitializeForRuntimeConfigFn =
            std::mem::transmute(context.initialize_for_runtime_config_func_ptr);

        get_host_context_handle(init_fptr, config_path, &mut host_context_handle);
        let get_delegate_fptr: HostfxrGetRuntimeDelegateFn =
            std::mem::transmute(context.get_runtime_delegate_func_ptr);

        let status = get_delegate_fptr(
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

pub fn get_entry_point_func() -> *mut EntryPointFn {
    const UNMANAGEDCALLERSONLY_METHOD: *const u16 = -1 as i16 as *const u16;
    let mut entry_point_func: *mut libc::c_void = std::ptr::null_mut();
    let config_path = "./AppWithPlugin.runtimeconfig.json".to_string();
    let load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path);
    unsafe {
        let load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerFn =
            std::mem::transmute(load_assembly_and_get_function_pointer);
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
        return entry_point_func;
    }
}
