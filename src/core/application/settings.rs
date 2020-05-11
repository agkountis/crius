use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use winit::dpi::LogicalSize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub window: WindowSettings,
    pub graphics: GraphicsSettings,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub version: Version,
    pub assets_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WindowSettings {
    pub title: String,
    pub size: Option<LogicalSize<u32>>,
    pub min_size: Option<LogicalSize<u32>>,
    pub max_size: Option<LogicalSize<u32>>,
    pub resizeable: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphicsSettings {
    pub extensions: Option<Vec<CString>>,
    pub layers: Option<Vec<CString>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
