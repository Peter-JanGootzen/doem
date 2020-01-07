use luminance_glfw::WindowEvent;
use std::collections::HashSet;

pub struct DoemEvents(pub Vec<WindowEvent>);

impl Default for DoemEvents {
    fn default() -> DoemEvents {
        DoemEvents(Vec::<WindowEvent>::new())
    }
}
