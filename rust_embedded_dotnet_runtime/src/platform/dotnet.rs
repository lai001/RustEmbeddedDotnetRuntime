use crate::entry_info::EntryPointFn;

#[cfg(unix)]
pub type HostfxrInitializeForRuntimeConfigFn =
    super::linux::dotnet::HostfxrInitializeForRuntimeConfigFn;
#[cfg(unix)]
pub type HostfxrCloseFn = super::linux::dotnet::HostfxrCloseFn;
#[cfg(unix)]
pub type HostfxrGetRuntimeDelegateFn = super::linux::dotnet::HostfxrGetRuntimeDelegateFn;
#[cfg(unix)]
pub type LoadAssemblyAndGetFunctionPointerFn =
    super::linux::dotnet::LoadAssemblyAndGetFunctionPointerFn;

#[cfg(windows)]
pub type HostfxrInitializeForRuntimeConfigFn =
    super::windows::dotnet::HostfxrInitializeForRuntimeConfigFn;
#[cfg(windows)]
pub type HostfxrCloseFn = super::windows::dotnet::HostfxrCloseFn;
#[cfg(windows)]
pub type HostfxrGetRuntimeDelegateFn = super::windows::dotnet::HostfxrGetRuntimeDelegateFn;
#[cfg(windows)]
pub type LoadAssemblyAndGetFunctionPointerFn =
    super::windows::dotnet::LoadAssemblyAndGetFunctionPointerFn;

pub fn load_hostfxr_library() -> bool {
    #[cfg(unix)]
    return super::linux::dotnet::load_hostfxr_library();
    #[cfg(windows)]
    return super::windows::dotnet::load_hostfxr_library();
}

pub fn get_dotnet_load_assembly(config_path: String) -> *mut LoadAssemblyAndGetFunctionPointerFn {
    #[cfg(unix)]
    return super::linux::dotnet::get_dotnet_load_assembly(config_path);
    #[cfg(windows)]
    return super::windows::dotnet::get_dotnet_load_assembly(config_path);
}

pub fn get_entry_point_func() -> *mut EntryPointFn {
    #[cfg(unix)]
    return super::linux::dotnet::get_entry_point_func();
    #[cfg(windows)]
    return super::windows::dotnet::get_entry_point_func();
}
