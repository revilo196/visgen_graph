use crate::ParameterEnd;
use crate::ParameterStore;
use crate::ParameterFactory;
use crate::TextureNode;
use crate::ParameterEndpoint;
use crate::shader_target::Shader2DTarget;
use crate::shapes::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
use crate::util::shader::read_shader_file;
use nannou::image::EncodableLayout;
use nannou::wgpu::{Device, TextueSnapshot,TextureCapturer};
use wgpu::ShaderModuleDescriptorSpirV;


/// Uniform data passed on to render Wave Texture for [WaveTextureNode]
#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsWave {
    color: [f32;3],
    time: f32,
    freq: f32,
    hard: f32,
    duty: f32,
    angle: f32,
    noise_amp: f32,
    noise_scale: f32,
    noise_speed: f32,
} 

/// generating a wavelike texture
/// 
/// # OSC Parameters used
/// 
/// | Endpoint          | Description                        |  Datatype    | Range        |
/// |-------------------|------------------------------------|--------------|--------------|
/// |`./color`          | color of the waves                 |`[3, f32]`    | (0, 1.0)     |
/// |`./freq`           | how many waves                     |`f32`         | (1, ...)     |         
/// |`./hard `          | 'hardness'/ slope                  |`f32`         | (0, ...) LOG |
/// |`./duty            | duty cycle / thickness             |`f32`         | (0, ...)     |
/// |`./angle`          | angle of the waves                 |`f32`         | (0, ...)     |
/// |`./noise_amp`      | amplitude of the noise in the waves|`f32`         | (0, ...)     |
/// |`./noise_scale`    | scale of the noise in the waves    |`f32`         | (0, ...)     |
/// |`./noise_speed`    | speed of the noise in the waves    |`f32`         | (0, ...)     |
/// |
/// 
pub struct WaveTextureNode {
    target : Shader2DTarget<UniformsWave>,
    color: ParameterEndpoint<f32>,
    param: [ParameterEndpoint<f32> ;7],
}

impl WaveTextureNode {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
        let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
        let frag_raw =  read_shader_file("shader/wave_frag.spv");

        let vert_data =  nannou::wgpu::util::make_spirv_raw( vert_raw.as_bytes());
        let frag_data = nannou::wgpu::util::make_spirv_raw( frag_raw.as_bytes());

        let vert = ShaderModuleDescriptorSpirV {
            label: Some("minimal2d_vert"),
            source : vert_data,
        };

        let frag = ShaderModuleDescriptorSpirV {
            label: Some("wave_frag"),
            source : frag_data,
        };

        let uniform = UniformsWave { color:[1.0,1.0,1.0], time:0.0, freq: 1.0, hard: 1.0, duty:0.5, angle:0.5 , noise_amp:0.0, noise_scale:0.0, noise_speed:1.0 };

        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 3, "color".to_string());
        let param = [
            factory.build_default(3.0, "freq".to_string()),
            factory.build_default(0.0, "hard".to_string()),
            factory.build_default(2.0, "duty".to_string()),
            factory.build_default(1.0, "angle".to_string()),
            factory.build_default(30.0, "noise_amp".to_string()),
            factory.build_default(1.5, "noise_scale".to_string()),
            factory.build_default(0.15, "noise_speed".to_string()),

        ];



        let target = Shader2DTarget::new(device, texture_size,
             &vert, &frag, &FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 
        Self {
            target,
            color,
            param,
        }
    }
}

impl TextureNode for WaveTextureNode {
    fn update(&mut self, app: &nannou::App, window: &nannou::window::Window, store: &ParameterStore, _input: Vec<nannou::wgpu::TextureView>) {
        // get parameter from osc parameter store
        let color_vec = self.color.get_vec(store);
        let color = [color_vec[0], color_vec[1], color_vec[2]];
        let time = app.time;
        let freq = self.param[0].get(store);
        let hard = self.param[1].get(store);
        let duty = self.param[2].get(store);
        let angle = self.param[3].get(store);
        let noise_amp = self.param[4].get(store);
        let noise_scale = self.param[5].get(store);
        let noise_speed = self.param[6].get(store);

        // build uniform for the shader
        let uniform = UniformsWave { color, time, freq, hard, duty, angle, noise_amp, noise_scale, noise_speed };

        let device = window.device();

        // render
        self.target.begin(device);
        self.target.set_uniforms(device, uniform);
        self.target.render_pass();
        self.target.end(&window);
    }

    fn output(&self) -> nannou::wgpu::TextureView {
        self.target.texture_view()
    }

    fn snapshot(&self, window: &nannou::window::Window, texture_capturer: &TextureCapturer) -> TextueSnapshot {
        self.target.snapshot(window, texture_capturer)
    }
}