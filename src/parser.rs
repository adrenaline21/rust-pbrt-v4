use crate::{paramdict::ParsedParameterVector, util::error::FileLoc, Float};

pub trait ParserTarget {
    fn scale(&mut self, sx: Float, sy: Float, sz: Float, loc: FileLoc);

    fn shape(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);

    fn option(&mut self, name: &String, value: &String, loc: FileLoc);

    fn identity(&mut self, loc: FileLoc);
    fn translate(&mut self, dx: Float, dy: Float, dz: Float, loc: FileLoc);
    fn rotate(&mut self, angle: Float, ax: Float, ay: Float, az: Float, loc: FileLoc);
    fn look_at(
        &mut self,
        ex: Float,
        ey: Float,
        ez: Float,
        lx: Float,
        ly: Float,
        lz: Float,
        ux: Float,
        uy: Float,
        uz: Float,
        loc: FileLoc,
    );
    fn concat_transform(&mut self, transform: [Float; 16], loc: FileLoc);
    fn transform(&mut self, transform: [Float; 16], loc: FileLoc);
    fn coordinate_system(&mut self, name: &String, loc: FileLoc);
    fn coord_sys_transform(&mut self, name: &String, loc: FileLoc);
    fn active_transform_all(&mut self, loc: FileLoc);
    fn active_transform_end_time(&mut self, loc: FileLoc);
    fn active_transform_start_time(&mut self, loc: FileLoc);
    fn transform_times(&mut self, start: Float, end: Float, loc: FileLoc);

    fn color_space(&mut self, name: &String, loc: FileLoc);
    fn pixel_filter(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn film(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn accelerator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn integrator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn camera(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn make_named_medium(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn medium_interface(&mut self, inside_name: &String, outside_name: &String, loc: FileLoc);
    fn sampler(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);

    fn world_begin(&mut self, loc: FileLoc);
    fn attribute_begin(&mut self, loc: FileLoc);
    fn attribute_end(&mut self, loc: FileLoc);
    fn attribute(&mut self, target: &String, params: ParsedParameterVector, loc: FileLoc);
    fn texture(
        &mut self,
        name: &String,
        typename: &String,
        texname: &String,
        params: ParsedParameterVector,
        loc: FileLoc,
    );
    fn material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn make_named_material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn named_material(&mut self, name: &String, loc: FileLoc);
    fn light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn area_light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn reverse_orientation(&mut self, loc: FileLoc);
    fn object_begin(&mut self, name: &String, loc: FileLoc);
    fn object_end(&mut self, loc: FileLoc);
    fn object_instance(&mut self, name: &String, loc: FileLoc);

    fn end_of_files(&mut self);

    // fn error_exit_deferred(&mut self);
    // error_exit
}

pub fn parse_files(target: &mut dyn ParserTarget, filenames: Vec<String>) -> Result<(), &str> {
    if filenames.is_empty() {
        Err("No file description given.")
    } else {
        Ok(())
    }
}
