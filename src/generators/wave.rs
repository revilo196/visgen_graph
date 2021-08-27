use crate::ParameterEnd;
use crate::ParameterStore;
use crate::ParameterFactory;
use crate::TextureNode;
use crate::ParameterEndpoint;
use crate::shader2d_target::Shader2DTarget;
use crate::shapes2d::{FULL_SCREEN_QUAD,FULL_SCREEN_QUAD_INDEX};
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
} 

pub struct WaveTextureNode {
    target : Shader2DTarget<UniformsWave>,
    color: ParameterEndpoint<f32>,
    param: [ParameterEndpoint<f32> ;4],
}

impl WaveTextureNode {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore,  device: &Device) -> Self {
        let vert = include_bytes!("../shaders/minimal2d_vert.spv");
        let frag = include_bytes!("../shaders/wave_frag.spv");
        let uniform = UniformsWave { color:[1.0,1.0,1.0,1.0], time:0.0, freq: 1.0, hard: 1.0, duty:0.5, angle:0.5 };

        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 4, "color".to_string());
        let param = [
            factory.build_default(1.0, "freq".to_string()),
            factory.build_default(1.0, "hard".to_string()),
            factory.build_default(0.5, "duty".to_string()),
            factory.build_default(1.0, "angle".to_string()),
        ];

        let target = Shader2DTarget::new(device, texture_size,
             vert, frag, &FULL_SCREEN_QUAD, &FULL_SCREEN_QUAD_INDEX, uniform); 
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
        let angle = self.param[2].get(store);

        let uniform = UniformsWave { color, time, freq, hard, duty, angle };

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