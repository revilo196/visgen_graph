//! A simple demonstration on how to create and draw with a custom wgpu render pipeline in nannou!
//!
//! The aim of this example is not to show the simplest way of drawing a triangle in nannou, but
//! rather provide a reference on how to get started creating your own rendering pipeline from
//! scratch. While nannou's provided graphics-y APIs can do a lot of things quite efficiently,
//! writing a custom pipeline that does only exactly what you need it to can sometimes result in
//! better performance.

use nannou::prelude::*;
use nannou::wgpu::{
    CommandEncoderDescriptor, Device, Texture, TextureBuilder, TextureUsage, TextureView,
};

struct Model {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    texture: Texture,
}

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

// The vertices that make up the rectangle to which the image will be drawn.
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().size(512, 512).view(view).build().unwrap();

    // The gpu device associated with the window's swapchain
    let window = app.window(w_id).unwrap();
    let device = window.swap_chain_device();
    let format = Frame::TEXTURE_FORMAT;
    let texture_size = [512, 512];
    // Load shader modules.
    let vs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/vert.spv"));
    let fs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/frag.spv"));

    // Frame Texture
    let texture = TextureBuilder::new()
        .size(texture_size)
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_DST | TextureUsage::SAMPLED)
        // Use nannou's default multisampling sample count
        .sample_count(1)
        .format(format)
        // Build it!
        .build(device);

    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsage::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });

    // Create the render pipeline.
    let bind_group_layout = wgpu::BindGroupLayoutBuilder::new().build(device);
    let bind_group = wgpu::BindGroupBuilder::new().build(device, &bind_group_layout);
    let pipeline_layout = wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&nannou::wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(1)
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .build(device);

    Model {
        bind_group,
        vertex_buffer,
        render_pipeline,
        texture,
    }
}

// Draw the state of your `Model` into the given Self.texture here.
fn update(app: &App, model: &mut Model, _update: Update) {
    // Using this we will encode commands that will be submitted to the GPU.
    let desc = CommandEncoderDescriptor {
        label: Some("Texture"),
    };
    let mut encoder = app
        .main_window()
        .swap_chain_device()
        .create_command_encoder(&desc);
    let texture_view = model.texture.view().build();
    // The render pass can be thought of a single large command consisting of sub commands. Here we
    // begin a render pass that outputs to the frame's texture. Then we add sub-commands for
    // setting the bind group, render pipeline, vertex buffers and then finally drawing.
    {
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(&texture_view, |color| color)
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &model.bind_group, &[]);
        render_pass.set_pipeline(&model.render_pipeline);
        render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));

        // We want to draw the whole range of vertices, and we're only drawing one instance of them.
        let vertex_range = 0..VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }
    // Now we're done! The commands we added will be submitted after `view` completes.
    app.main_window()
        .swap_chain_queue()
        .submit(Some(encoder.finish()));
}

fn view(_app: &App, model: &Model, frame: Frame) {
    let draw = _app.draw();

    draw.texture(&model.texture.view().build());
    draw.quad().w_h(10.0, 10.0);
    draw.to_frame(_app, &frame).unwrap();
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
