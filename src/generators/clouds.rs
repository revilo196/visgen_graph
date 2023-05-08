/// Very expensive fragment shader for rendering clouds
///
/// EXPERIMENTAL
use crate::ParameterStore;
use crate::TextureNode;

use crate::shader_target::Shader2DTarget;
use crate::shapes::{FULL_SCREEN_QUAD, FULL_SCREEN_QUAD_INDEX};
use crate::util::shader::read_shader_file;
use nannou::image::EncodableLayout;
use nannou::wgpu::{Device, TextueSnapshot, TextureCapturer};
use wgpu::ShaderModuleDescriptorSpirV;

#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsCloud {
    time: f32,
}

pub struct CloudsNode {
    target: Shader2DTarget<UniformsCloud>,
    //param: [ParameterEndpoint<f32> ;1],
}

impl CloudsNode {
    pub fn new(
        _name: String,
        texture_size: [u32; 2],
        _store: &mut ParameterStore,
        device: &Device,
    ) -> Self {
        let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
        let frag_raw = read_shader_file("shader/clouds_frag.spv");

        let vert_data = nannou::wgpu::util::make_spirv_raw(vert_raw.as_bytes());
        let frag_data = nannou::wgpu::util::make_spirv_raw(frag_raw.as_bytes());

        let vert = ShaderModuleDescriptorSpirV {
            label: Some("minimal2d_vert"),
            source: vert_data,
        };

        let frag = ShaderModuleDescriptorSpirV {
            label: Some("clouds_frag"),
            source: frag_data,
        };

        let uniform = UniformsCloud { time: 0.0 };

        let target = Shader2DTarget::new(
            device,
            texture_size,
            &vert,
            &frag,
            &FULL_SCREEN_QUAD,
            &FULL_SCREEN_QUAD_INDEX,
            uniform,
        );
        Self {
            target,
            //param,
        }
    }
}

impl TextureNode for CloudsNode {
    fn update(
        &mut self,
        app: &nannou::App,
        window: &nannou::window::Window,
        _store: &ParameterStore,
        _input: Vec<nannou::wgpu::TextureView>,
    ) {
        let time = app.time;

        let uniform = UniformsCloud { time };

        let device = window.device();

        self.target.begin(device);
        self.target.set_uniforms(device, uniform);
        self.target.render_pass();
        self.target.end(window);
    }

    fn output(&self) -> nannou::wgpu::TextureView {
        self.target.texture_view()
    }

    fn snapshot(
        &self,
        window: &nannou::window::Window,
        texture_capturer: &TextureCapturer,
    ) -> TextueSnapshot {
        self.target.snapshot(window, texture_capturer)
    }
}
