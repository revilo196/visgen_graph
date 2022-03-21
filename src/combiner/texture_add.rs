use crate::{ModelUpdate};
use crate::{ParameterStore};
use nannou::prelude::*;
use nannou::wgpu::TextureView;
use ::wgpu::BlendComponent;

/// simple model to add n textures together
/// 
/// this uses nannou no shader needed
pub struct TextureAddModel {
    texture_size : [u32;2],
}


impl TextureAddModel {
    pub fn new(_name: String,texture_size : [u32;2], _store : &mut ParameterStore)  -> Self{

        Self {
            texture_size,
        }
    }
}

impl ModelUpdate for TextureAddModel {
    fn update_model(&mut self, _app: &App, _store: &ParameterStore ,input: Vec<TextureView>) -> Draw { 
        let draw = Draw::new();
        let draw_blend = draw.color_blend(BlendComponent::OVER);

        for t in input {
            println!("Drawing Texture");
            draw_blend.texture(&t).w_h(self.texture_size[0] as f32 , self.texture_size[1] as f32);
        }

        draw_blend
    }

}