use nannou::prelude::*;
use visgen_graph::shader_target::*;
use visgen_graph::shapes::FULL_SCREEN_QUAD;
use ::wgpu::include_spirv_raw;

fn main() {
    nannou::app(model).update(update).run();
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExampleUniform {
    color: [f32; 4],
    time: f32,
    trans: Mat4,
}

struct Model {
    target: Shader2DTarget<ExampleUniform>,
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().size(512, 512).view(view).build().unwrap();

    // The gpu device associated with the window's swapchain
    let window = app.window(w_id).unwrap();
    let device = window.device();
    let texture_size = [512, 512];
    let vert = include_spirv_raw!("shaders/vert.spv");
    let frag = include_spirv_raw!("shaders/frag.spv");
    let uniform = ExampleUniform {
        time: app.time,
        color: [1.0, 0.8, 0.7, 1.0],
        trans: Mat4::IDENTITY,
    };
    let target = Shader2DTarget::new(
        device,
        texture_size,
        &vert,
        &frag,
        &FULL_SCREEN_QUAD,
        &[],
        uniform,
    );

    Model { target }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let uniform = ExampleUniform {
        time: app.time,
        color: [1.0, 0.5, 0.25, 1.0],
        trans: Mat4::IDENTITY,
    };
    let window = app.main_window();
    let device = window.device();
    model.target.begin(device);
    model.target.set_uniforms(device, uniform);
    model.target.render_pass();
    model.target.end(&window);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.texture(&model.target.texture_view());
    draw.quad().w_h(10.0, 10.0);
    draw.to_frame(_app, &frame).unwrap();
}
