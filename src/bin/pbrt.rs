use std::{env, rc::Rc};

use rpbrt::{
    cpu::render::render_cpu,
    parser::parse_files,
    pbrt::init_pbrt,
    scene::{BasicScene, BasicSceneBuilder},
};
// use std::fs::File;
// use std::io::{BufWriter, Write};
// let mut out = BufWriter::new(File::create("image.ppm")?);

fn main() -> Result<(), String> {
    // Convert command-line arguments to vector of strings
    let args = env::args();

    // TODO: Declare variables for parsed command line

    // Process command-line arguments
    let mut filenames: Vec<String> = Vec::new();
    for (idx, arg) in args.enumerate() {
        if idx == 0 {
            continue;
        }
        if arg.as_bytes()[0] != b'-' {
            filenames.push(arg);
        }
        // TODO: parse_args
    }

    // Initialize pbrt
    init_pbrt();

    // Parse provided scene description files
    let scene = Rc::new(BasicScene::new());
    let mut builder = BasicSceneBuilder::new(scene.clone());
    parse_files(&mut builder, filenames).unwrap();

    // Render the scene
    render_cpu(scene);

    // Clean up after rendering the scene

    Ok(())
}
