use ash::{
    extensions::khr,
    version::{EntryV1_0, InstanceV1_0},
    vk,
};

use std::ffi::{CStr, CString};
use winit::window::Window;

pub struct Surface {
    loader: khr::Surface,
    handle: vk::SurfaceKHR,
    format: vk::SurfaceFormatKHR,
    resolution: vk::Extent2D,
}

impl Surface {
    pub(crate) fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
        window: &Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let loader = khr::Surface::new(entry, instance);
        let handle = unsafe { Self::create_surface(entry, instance, window)? };

        Ok(Self {
            loader,
            handle,
            format: Default::default(),
            resolution: Default::default(),
        })
    }

    pub(crate) fn loader(&self) -> &khr::Surface {
        &self.loader
    }

    pub(crate) fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }

    pub(crate) fn format(&self) -> vk::SurfaceFormatKHR {
        self.format
    }

    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    pub(crate) fn extensions() -> Vec<&Cstr> {
        vec![khr::Surface::name(), khr::XlibSurface::name()]
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn extensions() -> Vec<CString> {
        vec![
            khr::Surface::name().into(),
            khr::Win32Surface::name().into(),
        ]
    }

    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
        window: &winit::window::Window,
    ) -> Result<vk::SurfaceKHR, vk::Result> {
        use ash::extensions::khr::XlibSurface;
        use winit::platform::unix::WindowExtUnix;

        let x11_display = window.xlib_display().unwrap();
        let x11_window = window.xlib_window().unwrap();
        let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(x11_window as vk::Window)
            .dpy(x11_display as *mut vk::Display);

        let xlib_surface_loader = XlibSurface::new(entry, instance);
        xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
    }

    #[cfg(target_os = "windows")]
    unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
        window: &winit::window::Window,
    ) -> Result<vk::SurfaceKHR, vk::Result> {
        use ash::extensions::khr::Win32Surface;
        use std::ffi::c_void;
        use std::ptr;
        use winapi::shared::windef::HWND;
        use winapi::um::libloaderapi::GetModuleHandleW;
        use winit::platform::windows::WindowExtWindows;

        let hwnd = window.hwnd() as HWND;
        let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
        let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance,
            hwnd: hwnd as *const c_void,
        };
        let win32_surface_loader = Win32Surface::new(entry, instance);
        win32_surface_loader.create_win32_surface(&win32_create_info, None)
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { self.loader.destroy_surface(self.handle, None) }
    }
}
