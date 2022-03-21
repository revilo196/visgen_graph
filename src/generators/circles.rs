use std::ops::Rem;

use rand::distributions::Uniform;
use rand::prelude::*;

use crate::{ModelUpdate, ParameterEnd};
use crate::{ParameterEndpoint,ParameterFactory,ParameterStore};
use nannou::prelude::*;

///
/// CircleGenerator model to generate a texture of circles.
/// 
/// Origin of the circle is changed after each iteration
/// 
/// # OSC Parameters used
/// 
/// | Endpoint          | Description                      |  Datatype    | Range    |
/// |-------------------|----------------------------------|--------------|----------|
/// |`./rgb`            | color of the circles             |`[3, f32]`    | (0, 1.0) |
/// |`./width`          | width of a single circle         |`f32`         | (1, 50)  |         
/// |`./count`          | count of circles to draw         |`f32`         | (1, 150) |
/// |`./res`            | resolution of the circles        |`f32`         | (3, 255) |
/// |`./speed`          | speed of the animation           |`f32`         | (0, 5)   |
/// |`./distance`       | max radius of the grow animation |`f32`         | (0, 10)  |
/// |`./rotation_speed` | speed of the rotation animation  |`f32`         | (0, 2)   |
/// |
/// 

pub struct CircleGenerator {
    texture_size : [u32;2],
    parameters :Vec<ParameterEndpoint<f32>>,
    color : ParameterEndpoint<f32>,
    last_time: f32
}

impl CircleGenerator {
    
    /// Creates a new CircleGenerator 
    /// 
    /// # Parameters
    /// - `name`: base name of the generator used for naming OSC Parameter Endpoints
    /// - `texture_size` : target texture size
    /// - `store` : global [ParameterStore] used to collect OSC Parameters
    /// 
    pub fn new(name: String,texture_size : [u32;2], store : &mut ParameterStore) -> Self {

        let mut parameters = Vec::new();
        let mut factory = ParameterFactory::new(name, store);
        let color = factory.build_array_default(1.0, 3, "rgb".to_string());
        parameters.push(factory.build_default(2.0 , "width".to_string()));
        parameters.push(factory.build_default( 50.0 ,"count".to_string()));
        parameters.push(factory.build_default(255.0, "res".to_string()));
        parameters.push(factory.build_default(1.0,"speed".to_string()));
        parameters.push(factory.build_default(1.0,"distance".to_string()));
        parameters.push(factory.build_default(1.0,"rotation_speed".to_string()));

        Self {
            texture_size,
            parameters,
            color,
            last_time: 0.0f32
        }
    }
}

impl ModelUpdate for CircleGenerator {
    fn update_model(&mut self, app: &nannou::App, store: &ParameterStore ,_input: Vec<nannou::wgpu::TextureView>) -> Draw {

        // receive parameters from the store
        let color = self.color.get_vec(store);
        let rgb = rgb8((color[0]*255.0) as u8, (color[1]*255.0) as u8, (color[2]*255.0) as u8);
        let mut handles = self.parameters.iter().map(|f|{f.bind(store)});
        let width : f32 = handles.next().unwrap().into();
        let count : f32 = handles.next().unwrap().into();
        let res : f32 = handles.next().unwrap().into();
        let speed : f32 = handles.next().unwrap().into();
        let distance : f32 = handles.next().unwrap().into();
        let rot_speed : f32 = handles.next().unwrap().into();

        // get the current runtime for animation
        let time  : f32 =  self.last_time + speed*app.fps();


        let draw = Draw::new();

        draw.background().color(BLACK);
        let scale = (self.texture_size[0] as f32 + self.texture_size[1] as f32)/2.0;
        let blend = draw.color_blend(BLEND_ADD);
        for i in 0..(count as i32 +2) {
            let time_i = time * speed * scale *0.1f32;
            let raw =  time_i + (i as f32) * distance * 10.0f32;
            let r = raw.rem(scale);
            let seed = (raw / scale).ceil() as u64;

            let f = fade_curve(5.0, r/scale);
            let color_i = fade_color(rgb, f);
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let dist = Uniform::new(-scale, scale);
            let x = rng.sample(dist);
            let y = rng.sample(dist);
            let radians = rng.sample(Uniform::new(0.0, std::f32::consts::PI *2.0)) + time_i * rot_speed;
            blend.ellipse()
                .no_fill()
                .stroke_color(color_i)
                .stroke_weight(width)
                .radius(r)
                .resolution(res)
                .rotate(radians)
                .x_y(x, y).z(f);
        }        

        self.last_time = time;

        draw
    }
}

/// calculate a fading curve over distance
fn fade_curve(n: f32, t:f32) -> f32 {
    n * t.powf(n+1.0) - (n+1.0) * t.powf(n) + 1.0
 }

/// convert single float value into color using a base color
/// - linear mapping: (0.0,1.0) -> (black, base_color) 
fn fade_color(color: Rgb8, f:f32) -> Rgb8 {
    let red = color.red as f32 * f;
    let green = color.green as f32 * f;
    let blue = color.blue as f32 * f;

    rgb8((red) as u8, (green) as u8, (blue) as u8)
 }