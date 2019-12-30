mod gl_common;
mod obj_loader;
mod shape;
mod ecs;
mod tess_manager;

use crate::obj_loader::ObjLoader;
use crate::shape::Shape;

use std::path::Path;
use clap::{Arg, App};
use luminance::render_state::RenderState;
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance::tess::TessSlice;
use specs::WorldExt;
use std::sync::Arc;
use std::sync::Mutex;
use crate::ecs::world::DoemWorld;
use crate::ecs::dispatcher::DoemDispatcher;

fn main() {
    let matches = App::new("Rusty obj viewer")
                          .version("1.0")
                          .author("Bram-Boris Meerlo and Peter-Jan Gootzen")
                          .about("Made using our own linear algebra crate rusty_linear_algebra.")
                          .arg(Arg::with_name("model_path")
                               .short("m")
                               .long("model")
                               .value_name("MODEL_PATH")
                               .help("Sets the wavefront obj model that is going to be loaded")
                               .index(1)
                               .required(true)
                               .takes_value(true))
                          .get_matches();

    let model_path_str = matches.value_of("model_path").unwrap();
    let model_path = Path::new(model_path_str);

    start(&model_path);
}

fn start(model_path: &Path) {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(1600, 900),
        "Doem",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    let shape_obj = ObjLoader::load(model_path).unwrap();
    let shape_aabb_tess = shape_obj.generate_aabb_tess(&mut surface).unwrap();
    let middle_point = shape_obj.middle_point.clone();
    let x_half_size = shape_obj.x_half_size;
    let y_half_size = shape_obj.y_half_size;
    let z_half_size = shape_obj.z_half_size;
    let shape_tess = shape_obj.to_tess(&mut surface).unwrap();
    let shape_tesselations = vec!(shape_tess, shape_aabb_tess);
    let mut shape = Shape::new(shape_tesselations, middle_point, x_half_size, y_half_size, z_half_size);

    let should_quit = Arc::new(Mutex::new(false));
    let mut world = DoemWorld::new();
    let mut dispatcher = DoemDispatcher::new(surface, should_quit.clone());
    dispatcher.setup(&mut world);
    'game_loop: loop {
        dispatcher.dispatch(&mut world);
        world.maintain();
        if *(*should_quit).lock().unwrap() {
            break 'game_loop;
        }
    }

}
