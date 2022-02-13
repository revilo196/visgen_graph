use wgpu::{Device, ShaderModuleDescriptorSpirV};

use crate::{TextureNode, ParameterEnd};
use crate::{Vertex2D, ParameterEndpoint, ParameterStore, util::shader::read_shader_file, ParameterFactory};

use crate::shapes::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
use super::shader_combiner::ShaderCombiner;
use nannou::image::EncodableLayout;


#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsMasking {
    f0: f32,
} 


///
/// MaskingNode Combines 3 Textues
/// combines 1st(a) and 2nd(b)  with 3rd(c) texture as Mask 
/// a*c + b*(1-c)
pub struct MaskingNode {
    target : ShaderCombiner<UniformsMasking, Vertex2D>,
    param: [ParameterEndpoint<f32> ;1],
}

impl MaskingNode {

    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
        
            let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
            let frag_raw =  read_shader_file("shader/masking_frag.spv");
    
            let vert_data =  nannou::wgpu::util::make_spirv_raw( vert_raw.as_bytes());
            let frag_data = nannou::wgpu::util::make_spirv_raw( frag_raw.as_bytes());
    
            let vert = ShaderModuleDescriptorSpirV {
                label: Some("minimal2d_vert"),
                source : vert_data,
            };
    
            let frag = ShaderModuleDescriptorSpirV {
                label: Some("masker_frag"),
                source : frag_data,
            };
    
            let uniform = UniformsMasking {f0:0.0};
    
            let mut factory = ParameterFactory::new(name, store);
            let param = [
                factory.build_default(0.0, "f0".to_string()),
            ];
        
            let target = ShaderCombiner::new(device, texture_size,
                &vert, &frag, 3,&FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 
    
    
            Self {
                target,
                param,
            }    
        }
    
    
}

impl TextureNode for MaskingNode {
    fn update(&mut self, _app: &nannou::App, window: &nannou::window::Window, store: &ParameterStore, input: Vec<nannou::wgpu::TextureView>) {
        assert_eq!(input.len(), 3 , "Not the correct number of Textures given");
        
        let f0 = self.param[0].get(store);
        
        let uniform = UniformsMasking {f0};

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