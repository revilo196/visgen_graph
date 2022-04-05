use wgpu::{Device, ShaderModuleDescriptorSpirV};

use crate::{TextureNode, ParameterEnd};
use crate::{Vertex2D, ParameterEndpoint, ParameterStore, util::shader::read_shader_file, ParameterFactory};

use crate::shapes::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
use crate::combiner::shader_combiner::ShaderCombiner;
use nannou::image::EncodableLayout;


#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsColorRamp {
    c0: [f32;3],
    f0: f32,
    c1: [f32;3],
    f1: f32,
    c2: [f32;3],
    f2: f32,
    mode: i32,
} 


/// Color Ramp textures using different parameters
/// 
/// Colorize an image using 3 colors - currently. 
/// low color, mid color and high color.
/// 
/// ramps, mode and setpoints can be configured 
///
/// | Endpoint              | Description                      |  Datatype    | Range       |
/// |-----------------------|----------------------------------|--------------|-------------|
/// | `./c0`                | low value color                  | `[3, f32]`   | (0, 1.0)    |
/// | `./c1`                | mid value color                  | `[3, f32]`   | (0, 1.0)    |
/// | `./c2`                | high value color                 | `[3, f32]`   | (0, 1.0)    |
/// | `./f0`                | low value setpoint               | `f32`        | (0,1.0)     |
/// | `./f1`                | mid value setpoint               | `f32`        | (0,1.0)     |
/// | `./f2`                | high value setpoint              | `f32`        | (0,1.0)     |
/// | `./mode`              | interpolation mode               | `i32`        | (0,1,2,3,4) |

/// # Target
/// [ShaderCombiner] is used as render target/pipeline
/// 
/// ## shaders used
/// - `shader/minimal2d.vert` shared simple vertex shader
/// - `shader/color_ramp.frag` shader for this
/// 
pub struct ColorRampNode {
    target : ShaderCombiner<UniformsColorRamp, Vertex2D>,
    colors:  [ParameterEndpoint<f32>; 3],
    param: [ParameterEndpoint<f32> ;3],
    mode: ParameterEndpoint<i32>,
}

impl ColorRampNode {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
    
        let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
        let frag_raw =  read_shader_file("shader/color_ramp_frag.spv");

        let vert_data =  nannou::wgpu::util::make_spirv_raw( vert_raw.as_bytes());
        let frag_data = nannou::wgpu::util::make_spirv_raw( frag_raw.as_bytes());

        let vert = ShaderModuleDescriptorSpirV {
            label: Some("minimal2d_vert"),
            source : vert_data,
        };

        let frag = ShaderModuleDescriptorSpirV {
            label: Some("color_ramp_frag"),
            source : frag_data,
        };

        let uniform = UniformsColorRamp {c0:[1.0,1.0,1.0], c1:[1.0,1.0,1.0], c2:[1.0,1.0,1.0], f0:0.0, f1:0.5, f2:1.0, mode: 0};

        let mut factory = ParameterFactory::new(name, store);

        let colors = [
            factory.build_array_default(0.0, 3, "c0".to_string()),
            factory.build_array_default(0.7, 3, "c1".to_string()),
            factory.build_array_default(1.0, 3, "c2".to_string()),
        ];

        let param = [
            factory.build_default(0.0, "f0".to_string()),
            factory.build_default(0.5, "f1".to_string()),
            factory.build_default(1.0, "f2".to_string()),
        ];

        let mode = factory.build_default(0, "mode".to_string());
    
        let target = ShaderCombiner::new(device, texture_size,
            &vert, &frag, 1,&FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 


        Self {
            target,
            colors,
            param,
            mode
        }    
    }

}

impl TextureNode for ColorRampNode {
    fn update(&mut self, _app: &nannou::App, window: &nannou::window::Window, store: &ParameterStore, input: Vec<nannou::wgpu::TextureView>) {
        let c0_vec = self.colors[0].get_vec(store);
        let c0 = [c0_vec[0], c0_vec[1], c0_vec[2]];
        let c1_vec = self.colors[1].get_vec(store);
        let c1 = [c1_vec[0], c1_vec[1], c1_vec[2]];
        let c2_vec = self.colors[2].get_vec(store);
        let c2 = [c2_vec[0], c2_vec[1], c2_vec[2]];
        let f0 = self.param[0].get(store);
        let f1 = self.param[1].get(store);
        let f2 = self.param[2].get(store);
        let mode = self.mode.get(store);

        let uniform = UniformsColorRamp {c0, f0, c1, f1, c2, f2, mode};

        let device = window.device();

        self.target.begin(device);
        self.target.set_uniforms(device, uniform);
        self.target.render_pass(device,input );
        self.target.end(&window);
    }

    fn output(&self) -> nannou::wgpu::TextureView {
        self.target.texture_view()
    }

    fn snapshot(&self, window: &nannou::window::Window, texture_capturer: &nannou::wgpu::TextureCapturer) -> nannou::wgpu::TextueSnapshot {
        self.target.snapshot(window, texture_capturer)
    }
}