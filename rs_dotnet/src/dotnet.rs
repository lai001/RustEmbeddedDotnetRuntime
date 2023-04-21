use crate::entry_info::EntryPointFn;

#[cfg(windows)]
pub(crate) type HostfxrInitializeForRuntimeConfigFn =
    super::windows::dotnet::HostfxrInitializeForRuntimeConfigFn;

#[cfg(windows)]
pub(crate) type HostfxrCloseFn = super::windows::dotnet::HostfxrCloseFn;

#[cfg(windows)]
pub(crate) type HostfxrGetRuntimeDelegateFn = super::windows::dotnet::HostfxrGetRuntimeDelegateFn;

fn load_hostfxr_library() -> bool {
    #[cfg(unix)]
    return super::linux::dotnet::load_hostfxr_library();
    #[cfg(windows)]
    return super::windows::dotnet::load_hostfxr_library();
}

fn get_entry_point_func() -> *mut EntryPointFn {
    #[cfg(unix)]
    return super::linux::dotnet::get_entry_point_func();
    #[cfg(windows)]
    return super::windows::dotnet::get_entry_point_func();
}

pub fn initialize(entry_info: *mut libc::c_void) {
    if !load_hostfxr_library() {
        panic!();
    }
    let entry_point_func = get_entry_point_func();
    assert_ne!(entry_point_func, std::ptr::null_mut());
    unsafe {
        let entry_point_func: EntryPointFn = std::mem::transmute(entry_point_func);
        entry_point_func(entry_info);
    }
}
