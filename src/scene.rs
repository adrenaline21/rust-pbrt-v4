use once_cell::sync::Lazy;

use crate::paramdict::{ParameterDictionary, ParsedParameterVector};
use crate::parser::ParserTarget;
use crate::util;
use crate::util::colorspace::{self, RGBColorSpace};
use crate::util::containers::InternCache;
use crate::util::error::FileLoc;
use crate::util::string::InternedString;
use std::rc::Rc;
pub struct BasicScene {}

impl BasicScene {
    pub fn new() -> Self {
        BasicScene {}
    }
}

// trait Scene {
//     fn create_integrator() -> Box<dyn Integrator> {
//         Box::new(RandomWalkIntegrator::new())
//     }
// }

// impl Scene for BasicScene {}

#[derive(PartialEq)]
enum BlockState {
    OptionsBlock,
    WorldBlock,
}

pub struct BasicSceneBuilder {
    scene: Rc<BasicScene>,
    current_block: BlockState,
    graphics_state: GraphicsState,
    // render_from_world: Transform,
    // transform_cache: InternCache<Transform>,
    sampler: SceneEntity,
}

impl BasicSceneBuilder {
    pub fn new(scene: Rc<BasicScene>) -> Self {
        let mut sampler = SceneEntity::default();
        sampler.name = InternedString(INTERNED_STRINGS.lookup(&String::from("zsobol")));
        Self {
            scene: scene,
            current_block: BlockState::OptionsBlock,
            graphics_state: GraphicsState::new(),
            sampler: sampler,
        }
    }

    #[inline]
    fn verify_options(&self, func_name: &str) {
        if self.current_block != BlockState::OptionsBlock {
            panic!("{} only allowed in options block.", func_name);
        }
    }

    #[inline]
    fn verify_world(&self, func_name: &str) {
        if self.current_block != BlockState::WorldBlock {
            panic!("{} only allowed in world block.", func_name);
        }
    }
}

impl ParserTarget for BasicSceneBuilder {
    fn reverse_orientation(&mut self, loc: FileLoc) {
        self.verify_world("RerverseOrientation");
        self.graphics_state.reverse_orientation = !self.graphics_state.reverse_orientation;
    }

    fn color_space(&mut self, name: &String, loc: FileLoc) {
        self.graphics_state.color_space = colorspace::get_named(name).unwrap()
    }

    fn identity(&mut self, loc: FileLoc) {}

    fn sampler(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        let dict = ParameterDictionary::new(params, self.graphics_state.color_space);
        self.verify_options("Sampler");
        self.sampler = SceneEntity::new(name, dict, loc);
    }

    fn scale(&mut self, sx: crate::Float, sy: crate::Float, sz: crate::Float, loc: FileLoc) {
        todo!()
    }

    fn shape(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn option(&mut self, name: &String, value: &String, loc: FileLoc) {
        todo!()
    }

    fn translate(&mut self, dx: crate::Float, dy: crate::Float, dz: crate::Float, loc: FileLoc) {
        todo!()
    }

    fn rotate(
        &mut self,
        angle: crate::Float,
        ax: crate::Float,
        ay: crate::Float,
        az: crate::Float,
        loc: FileLoc,
    ) {
        todo!()
    }

    fn look_at(
        &mut self,
        ex: crate::Float,
        ey: crate::Float,
        ez: crate::Float,
        lx: crate::Float,
        ly: crate::Float,
        lz: crate::Float,
        ux: crate::Float,
        uy: crate::Float,
        uz: crate::Float,
        loc: FileLoc,
    ) {
        todo!()
    }

    fn concat_transform(&mut self, transform: [crate::Float; 16], loc: FileLoc) {
        todo!()
    }

    fn transform(&mut self, transform: [crate::Float; 16], loc: FileLoc) {
        todo!()
    }

    fn coordinate_system(&mut self, name: &String, loc: FileLoc) {
        todo!()
    }

    fn coord_sys_transform(&mut self, name: &String, loc: FileLoc) {
        todo!()
    }

    fn active_transform_all(&mut self, loc: FileLoc) {
        todo!()
    }

    fn active_transform_end_time(&mut self, loc: FileLoc) {
        todo!()
    }

    fn active_transform_start_time(&mut self, loc: FileLoc) {
        todo!()
    }

    fn transform_times(&mut self, start: crate::Float, end: crate::Float, loc: FileLoc) {
        todo!()
    }

    fn pixel_filter(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn film(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn accelerator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn integrator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn camera(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn make_named_medium(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn medium_interface(&mut self, inside_name: &String, outside_name: &String, loc: FileLoc) {
        todo!()
    }

    fn world_begin(&mut self, loc: FileLoc) {
        todo!()
    }

    fn attribute_begin(&mut self, loc: FileLoc) {
        todo!()
    }

    fn attribute_end(&mut self, loc: FileLoc) {
        todo!()
    }

    fn attribute(&mut self, target: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn texture(
        &mut self,
        name: &String,
        typename: &String,
        texname: &String,
        params: ParsedParameterVector,
        loc: FileLoc,
    ) {
        todo!()
    }

    fn material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn make_named_material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn named_material(&mut self, name: &String, loc: FileLoc) {
        todo!()
    }

    fn light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn area_light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc) {
        todo!()
    }

    fn object_begin(&mut self, name: &String, loc: FileLoc) {
        todo!()
    }

    fn object_end(&mut self, loc: FileLoc) {
        todo!()
    }

    fn object_instance(&mut self, name: &String, loc: FileLoc) {
        todo!()
    }

    fn end_of_files(&mut self) {
        todo!()
    }
}

#[derive(Default)]
struct SceneEntity {
    name: InternedString,
    loc: FileLoc,
    parameters: ParameterDictionary,
}

impl SceneEntity {
    pub fn new(name: &String, parameters: ParameterDictionary, loc: FileLoc) -> Self {
        Self {
            name: InternedString(INTERNED_STRINGS.lookup(name)),
            parameters: parameters,
            loc: loc,
        }
    }
}

static INTERNED_STRINGS: Lazy<InternCache<String>> = Lazy::new(|| InternCache::new());

struct GraphicsState {
    reverse_orientation: bool,
    color_space: &'static RGBColorSpace,
}

impl GraphicsState {
    pub fn new() -> Self {
        Self {
            reverse_orientation: false,
            color_space: &colorspace::sRGB,
        }
    }
}
