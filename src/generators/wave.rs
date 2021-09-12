use crate::ParameterEnd;
use crate::ParameterStore;
use crate::ParameterFactory;
use crate::TextureNode;
use crate::ParameterEndpoint;
use crate::shader2d_target::Shader2DTarget;
use crate::shapes2d::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
use crate::util::shader::read_shader_file;
use nannou::image::EncodableLayout;
use nannou::wgpu::Device;


#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsWave {
    color: [f32;4],
    time: f32,
    freq: f32,
    hard: f32,
    duty: f32,
    angle: f32,
    noise_amp: f32,
    noise_scale: f32,
    noise_speed: f32,
} 

pub struct WaveTextureNode {
    target : Shader2DTarget<UniformsWave>,
    color: ParameterEndpoint<f32>,
    param: [ParameterEndpoint<f32> ;7],
}

impl WaveTextureNode {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
        let vert = read_shader_file("shader/minimal2d_vert.spv");
        let frag = read_shader_file("shader/wave_frag.spv");
        let uniform = UniformsWave { color:[1.0,1.0,1.0,1.0], time:0.0, freq: 1.0, hard: 1.0, duty:0.5, angle:0.5 , noise_amp:0.0, noise_scale:0.0, noise_speed:1.0 };

        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 4, "color".to_string());
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
             vert.as_bytes(), frag.as_bytes(), &FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 
        Self {
            param,
            color,
            target
        }
    }
}

impl TextureNode for WaveTextureNode {
    fn update(&mut self, app: &nannou::App, window: &nannou::window::Window, store: &ParameterStore, _input: Vec<nannou::wgpu::TextureView>) {
        let color_vec = self.color.get_vec(store);
        let color = [color_vec[0], color_vec[1], color_vec[2], color_vec[3]];
        let time = app.time;
        let freq = self.param[0].get(store);
        let hard = self.param[1].get(store);
        let duty = self.param[2].get(store);
        let angle = self.param[3].get(store);
        let noise_amp = self.param[4].get(store);
        let noise_scale = self.param[5].get(store);
        let noise_speed = self.param[6].get(store);

        let uniform = UniformsWave { color, time, freq, hard, duty, angle, noise_amp, noise_scale, noise_speed };

        let device = window.swap_chain_device();

        self.target.begin(device);
        self.target.set_uniforms(device, uniform);
        self.target.render_pass();
        self.target.end(&window);
    }

    fn output(&self) -> nannou::wgpu::TextureView {
        self.target.texture_view()
    }
}