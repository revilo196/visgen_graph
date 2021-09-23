use std::ops::Rem;
use crate::{ModelUpdate, ParameterEnd};
use crate::{ParameterEndpoint,ParameterFactory,ParameterStore};
use nannou::prelude::*;

pub struct StripeGenerator {
    texture_size : [u32;2],
    parameters :Vec<ParameterEndpoint<f32>>,
    color : ParameterEndpoint<f32>,
}

impl StripeGenerator {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore) -> Self {

        let mut parameters = Vec::new();
        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 3, "rgb".to_string());
        parameters.push(factory.build_default(0.3 , "width".to_string()));
        parameters.push(factory.build_default( 10.0 ,"count".to_string()));
        parameters.push(factory.build_default(0.5, "angle".to_string()));
        parameters.push(factory.build_default(1.0,"speed".to_string()));


        Self {
            texture_size,
            parameters,
            color,
        }
    }
}

impl ModelUpdate for StripeGenerator {
    fn update_model(&mut self, app: &nannou::App, store: &ParameterStore ,_input: Vec<nannou::wgpu::TextureView>) -> Draw {
        let color = self.color.get_vec(store);
        let rgb = rgb8((color[0]*255.0) as u8, (color[1]*255.0) as u8, (color[2]*255.0) as u8);
        let mut handles = self.parameters.iter().map(|f|{f.bind(store)});
        let width : f32 = handles.next().unwrap().into();
        let count : f32 = handles.next().unwrap().into();
        let angle : f32 = handles.next().unwrap().into();
        let speed : f32 = handles.next().unwrap().into();
        let time  : f32 = app.time;
        
        let height = self.texture_size[1] as f32 * 1.5;

        let margin = height* angle.sin() + ((self.texture_size[0] as f32/ count)*2.0) ;
        let tex_width = (self.texture_size[0] as f32 + margin) as f32;
        let tile = tex_width / count;
        let tile_width = tile * width; 


        let draw = Draw::new();
        let x_start = -tex_width/2.0;

        draw.background().color(BLACK);

        for i in 0..(count as i32 +2) {

            let x = ((time * speed * 10.0) + (tile * (i as f32))).rem(tex_width) + x_start; 

            draw.quad().x_y(x, 0.0).w_h(tile_width, height).rotate(angle).color(rgb);

        } 

        
        draw
    }
}