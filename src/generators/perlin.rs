use crate::shader_target::Shader2DTarget;
use crate::shapes::{FULL_SCREEN_QUAD, FULL_SCREEN_QUAD_INDEX};
use crate::util::shader::read_shader_file;
use crate::ParameterEnd;
use crate::ParameterEndpoint;
use crate::ParameterFactory;
use crate::ParameterStore;
use crate::TextureNode;
use nannou::image::EncodableLayout;
use nannou::wgpu::{Device, TextueSnapshot, TextureCapturer};
use wgpu::ShaderModuleDescriptorSpirV;

/// Uniform data passed on to render Wave Texture for [WaveTextureNode]
#[repr(C)]
#[derive(Clone, Copy)]
struct UniformsPerlin {
    color: [f32; 3],
    tx: f32,
    ty: f32,
    tz: f32,
    sx: f32,
    sy: f32,
    octave: i32,
}

/// generating a wavelike texture
///
/// # OSC Parameters used
///
/// | Endpoint          | Description                        |  Datatype    | Range        |
/// |-------------------|------------------------------------|--------------|--------------|
/// |`./color`          | color of the waves                 |`[3, f32]`    | (0, 1.0)     |
/// |`./speedx`         | how many waves                     |`f32`         | (1, ...)     |         
/// |`./speedy`         | 'hardness'/ slope                  |`f32`         | (0, ...) LOG |
/// |`./speedz          | duty cycle / thickness             |`f32`         | (0, ...)     |
/// |`./scalex`         | angle of the waves                 |`f32`         | (0, ...)     |
/// |`./scaley   `      | amplitude of the noise in the waves|`f32`         | (0, ...)     |
/// |`./octave   `      | how many noise octaves are calculated|`f32`(i32)  | (0, ...)     |
///
pub struct PerlinTextureNode {
    target: Shader2DTarget<UniformsPerlin>,
    color: ParameterEndpoint<f32>,
    param: [ParameterEndpoint<f32>; 6],
}

impl PerlinTextureNode {
    pub fn new(
        name: String,
        texture_size: [u32; 2],
        store: &mut ParameterStore,
        device: &Device,
    ) -> Self {
        let vert_raw = read_shader_file("shader/minimal2d_vert.spv");
        let frag_raw = read_shader_file("shader/perlin_frag.spv");

        let vert_data = nannou::wgpu::util::make_spirv_raw(vert_raw.as_bytes());
        let frag_data = nannou::wgpu::util::make_spirv_raw(frag_raw.as_bytes());

        let vert = ShaderModuleDescriptorSpirV {
            label: Some("minimal2d_vert"),
            source: vert_data,
        };

        let frag = ShaderModuleDescriptorSpirV {
            label: Some("wave_frag"),
            source: frag_data,
        };

        let uniform = UniformsPerlin {
            color: [1.0, 1.0, 1.0],
            tx: 0.0,
            ty: 1.0,
            tz: 1.0,
            sx: 0.5,
            sy: 0.5,
            octave: 4,
        };

        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 3, "color".to_string());
        let param = [
            factory.build_default(0.0, "speedx".to_string()),
            factory.build_default(0.0, "speedy".to_string()),
            factory.build_default(0.1, "speedz".to_string()),
            factory.build_default(0.8, "scalex".to_string()),
            factory.build_default(0.8, "scaley".to_string()),
            factory.build_default(4.0, "octave".to_string()),
        ];

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
            color,
            param,
        }
    }
}

impl TextureNode for PerlinTextureNode {
    fn update(
        &mut self,
        app: &nannou::App,
        window: &nannou::window::Window,
        store: &ParameterStore,
        _input: Vec<nannou::wgpu::TextureView>,
    ) {
        // get parameter from osc parameter store
        let color_vec = self.color.get_vec(store);
        let color = [color_vec[0], color_vec[1], color_vec[2]];
        let tx = app.time * self.param[0].get(store);
        let ty = app.time * self.param[1].get(store);
        let tz = app.time * self.param[2].get(store);
        let sx = self.param[3].get(store);
        let sy = self.param[4].get(store);
        let octave: i32 = self.param[5].get(store) as i32;

        // build uniform for the shader
        let uniform = UniformsPerlin {
            color,
            tx,
            ty,
            tz,
            sx,
            sy,
            octave,
        };

        let device = window.device();

        // render
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
