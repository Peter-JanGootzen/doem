use luminance_glfw::WindowEvent;

pub struct DoemEvents(pub Vec<WindowEvent>);

impl Default for DoemEvents {
    fn default() -> DoemEvents {
        DoemEvents(Vec::<WindowEvent>::new())
    }
}

