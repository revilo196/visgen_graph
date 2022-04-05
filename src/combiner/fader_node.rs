use wgpu::{Device, ShaderModuleDescriptorSpirV};

use crate::{TextureNode, ParameterEnd};
use crate::{Vertex2D, ParameterEndpoint, ParameterStore, util::shader::read_shader_file, ParameterFactory};

use crate::shapes::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
use super::shader_combiner::ShaderCombiner;
use nannou::image::EncodableLayout;


#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsFade {
    f0: f32,
    f1: f32,
    f2: f32,
    f12: f32,
    fi1: f32,
    fi2: f32,
    f1i2: f32,
    fi12: f32,
    fi1i2: f32,
} 


/// Combining 2 textures using different parameters
/// 
///  # OSC Parameters used
/// 
/// | Endpoint              | Description                      |  Datatype    | Range    |
/// |-----------------------|----------------------------------|--------------|----------|
/// | `./f0`                | bias white to add added          | f32          | (0,1.0)  |
/// | `./f1`                | factor first texture             | f32          | (0,1.0)  |
/// | `./f2`                | factor second texture            | f32          | (0,1.0)  |
/// | `./f1_mul_2`          | factor first MUL second          | f32          | (0,1.0)  |
/// | `./f1_inv`            | factor inverted first texture    | f32          | (0,1.0)  |
/// | `./f2_inv`            | factor inverted second texture   | f32          | (0,1.0)  | 
/// | `./f1_mul_2_inv`      | combination of MUL and invert    | f32          | (0,1.0)  |
/// | `./f1_inv_mul_2_inv`  | combination of MUL and invert    | f32          | (0,1.0)  |
///     
/// # Target
/// [ShaderCombiner] is used as render target/pipeline
/// 
/// ## shaders used
/// - `shader/minimal2d.vert` shared simple vertex shader
/// - `shader/fader.frag` shader for this
pub struct FaderNode {
    target : ShaderCombiner<UniformsFade, Vertex2D>,
    param: [ParameterEndpoint<f32> ;9],
}

impl FaderNode {
    /// create new FaderNode
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
    
        let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
        let frag_raw =  read_shader_file("shader/fader_frag.spv");

        let vert_data =  nannou::wgpu::util::make_spirv_raw( vert_raw.as_bytes());
        let frag_data = nannou::wgpu::util::make_spirv_raw( frag_raw.as_bytes());

        let vert = ShaderModuleDescriptorSpirV {
            label: Some("minimal2d_vert"),
            source : vert_data,
        };

        let frag = ShaderModuleDescriptorSpirV {
            label: Some("fader_frag"),
            source : frag_data,
        };

        let uniform = UniformsFade {f0:0.0, f1:0.5, f2:0.5, f12: 0.0, fi1: 0.0, fi2: 0.0, f1i2: 0.0, fi12: 0.0, fi1i2: 0.0 };

        let mut factory = ParameterFactory::new(name, store);
        let param = [
            factory.build_default(0.0, "f0".to_string()),
            factory.build_default(0.0, "f1".to_string()),
            factory.build_default(0.05, "f2".to_string()),
            factory.build_default(0.9, "f1_mul_2".to_string()),
            factory.build_default(0.0, "f1_inv".to_string()),
            factory.build_default(0.0, "f2_inv".to_string()),
            factory.build_default(0.0, "f1_mul_2_inv".to_string()),
            factory.build_default(0.0, "f1_inv_mul_2".to_string()),
            factory.build_default(0.0, "f1_inv_mul_2_inv".to_string()),
        ];
    
        let target = ShaderCombiner::new(device, texture_size,
            &vert, &frag, 2,&FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 


        Self {
            target,
            param,
        }    
    }

}

impl TextureNode for FaderNode {
    fn update(&mut self, _app: &nannou::App, window: &nannou::window::Window, store: &ParameterStore, input: Vec<nannou::wgpu::TextureView>) {
        let f0 = self.param[0].get(store);
        let f1 = self.param[1].get(store);
        let f2 = self.param[2].get(store);

        let f12 = self.param[3].get(store);
        let fi1 = self.param[4].get(store);
        let fi2 = self.param[5].get(store);

        let f1i2 = self.param[6].get(store);
        let fi12 = self.param[7].get(store);
        let fi1i2 = self.param[8].get(store);

        let uniform = UniformsFade {f0, f1, f2, f12,fi1,fi2,f1i2,fi12,fi1i2};

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