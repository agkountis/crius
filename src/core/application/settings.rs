use serde::{Deserialize, Serialize};
use winit::dpi::LogicalSize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub version: Version,
    pub assets_path: String,
    pub window: WindowSettings,
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
