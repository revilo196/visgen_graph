use std::ops::Rem;

use rand::distributions::Uniform;
use rand::prelude::*;

use crate::{ModelUpdate, ParameterEnd};
use crate::{ParameterEndpoint,ParameterFactory,ParameterStore};
use nannou::prelude::*;


pub struct CircleGenerator {
    texture_size : [u32;2],
    parameters :Vec<ParameterEndpoint<f32>>,
    color : ParameterEndpoint<f32>,
}

impl CircleGenerator {
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore) -> Self {

        let mut parameters = Vec::new();
        let mut factory = ParameterFactory::new(name.clone(), store);
        let color = factory.build_array_default(1.0, 3, "rgb".to_string());
        parameters.push(factory.build_default(2.0 , "width".to_string()));
        parameters.push(factory.build_default( 50.0 ,"count".to_string()));
        parameters.push(factory.build_default(255.0, "res".to_string()));
        parameters.push(factory.build_default(1.0,"speed".to_string()));


        Self {
            texture_size,
            parameters,
            color,
        }
    }
}

impl ModelUpdate for CircleGenerator {
    fn update_model(&mut self, app: &nannou::App, store: &ParameterStore ,_input: Vec<nannou::wgpu::TextureView>) -> Draw {
        let color = self.color.get_vec(store);
        let rgb = rgb8((color[0]*255.0) as u8, (color[1]*255.0) as u8, (color[2]*255.0) as u8);
        let mut handles = self.parameters.iter().map(|f|{f.bind(store)});
        let width : f32 = handles.next().unwrap().into();
        let count : f32 = handles.next().unwrap().into();
        let res : f32 = handles.next().unwrap().into();
        let speed : f32 = handles.next().unwrap().into();
        let time  : f32 = app.time;


        let draw = Draw::new();

        draw.background().color(BLACK);
        let scale = (self.texture_size[0] as f32 + self.texture_size[1] as f32)/2.0;

        for i in 0..(count as i32 +2) {
            let time_i = time * speed * scale *0.1f32;
            let raw =  time_i + (i as f32) * 50.0f32;
            let r = raw.rem(scale);
            let seed = (raw / scale).ceil() as u64;

            let mut rng = rand::rngs::StdRng::seed_from_u64(seed+(i as u64));
            let dist = Uniform::new(-scale, scale);
            let x = rng.sample(dist);
            let y = rng.sample(dist);
            draw.ellipse().no_fill().stroke_color(rgb).stroke_weight(width).radius(r).resolution(res).x_y(x, y);
        }        
        draw
    }
}