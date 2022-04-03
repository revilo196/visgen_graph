/// UNUSED CODE
use crate::{ParameterStore, TextureNode};
use nannou::prelude::*;
use nannou::wgpu::{
    CommandEncoderDescriptor, Device, Texture, TextureBuilder, TextureUsages, TextureView,
};

use ::wgpu::ShaderModuleDescriptorSpirV;

pub struct Shader2DNode {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    texture: Texture,
    vertecies: Vec<Vertex2D>,
    uniforms: Uniforms2D,
}

// The vertices that make up the rectangle to which the image will be drawn.
pub const FULL_SCREEN_QUAD: [Vertex2D; 4] = [
    Vertex2D {
        position: [-1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, -1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, -1.0],
    },
];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex2D {
    position: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms2D {
    color: [f32; 4],
    time: f32,
    trans: Mat4,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
}

impl Shader2DNode {
    pub fn new(
        device: &Device,
        texture_size: [u32; 2],
        vert: &ShaderModuleDescriptorSpirV,
        frag: &ShaderModuleDescriptorSpirV,
        vertecies: &[Vertex2D],
    ) -> Self {
        let format = Frame::TEXTURE_FORMAT;

        let vs_mod = unsafe { device.create_shader_module_spirv(vert) };
        let fs_mod = unsafe { device.create_shader_module_spirv(frag) };

        // Frame Texture
        let texture = TextureBuilder::new()
            .size(texture_size)
            .usage(
                TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_DST
                    | TextureUsages::TEXTURE_BINDING,
            )
            .sample_count(1)
            .format(format)
            .build(device);

        let vertices_bytes = vertices_as_bytes(vertecies);
        let usage = wgpu::BufferUsages::VERTEX;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage,
        });

        let uniforms = Uniforms2D {
            color: [1.0, 0.0, 0.5, 1.0],
            time: 0.5,
            trans: Mat4::IDENTITY,
            p1: 0.0,
            p2: 0.0,
            p3: 0.0,
            p4: 0.0,
            p5: 0.0,
            p6: 0.0,
        };
        let uniforms_bytes = uniforms_as_bytes(&uniforms);
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        });

        // Create the render pipeline.
        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
            .build(device);
        let bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<Uniforms2D>(&uniform_buffer, 0..1)
            .build(device, &bind_group_layout);
        let pipeline_layout =
            wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
        let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
            .fragment_shader(&fs_mod)
            .color_format(format)
            .color_blend(wgpu::BlendComponent::REPLACE)
            .alpha_blend(wgpu::BlendComponent::REPLACE)
            .add_vertex_buffer::<Vertex2D>(&nannou::wgpu::vertex_attr_array![0 => Float32x2])
            .sample_count(1)
            .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
            .build(device);

        Self {
            bind_group,
            vertex_buffer,
            render_pipeline,
            uniform_buffer,
            texture,
            vertecies: vertecies.to_vec(),
            uniforms,
        }
    }
}

impl TextureNode for Shader2DNode {
    fn update(
        &mut self,
        app: &App,
        window: &Window,
        _store: &ParameterStore,
        _input: Vec<TextureView>,
    ) {
        // Using this we will encode commands that will be submitted to the GPU.
        let desc = CommandEncoderDescriptor {
            label: Some("Texture"),
        };
        let mut encoder = window.device().create_command_encoder(&desc);
        let texture_view = self.texture.view().build();

        //update uniforms
        self.uniforms.time = app.time;
        let uniforms_size = std::mem::size_of::<Uniforms2D>() as wgpu::BufferAddress;
        let uniforms_bytes = uniforms_as_bytes(&self.uniforms);
        let usage = wgpu::BufferUsages::COPY_SRC;
        let new_uniform_buffer = window.device().create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        });
        encoder.copy_buffer_to_buffer(
            &new_uniform_buffer,
            0,
            &self.uniform_buffer,
            0,
            uniforms_size,
        );

        //println!("{},{},{}, {:?}", app.time, uniforms_size ,window.swap_chain_device().features().contains( wgpu::Features::CLEAR_COMMANDS) ,uniforms_bytes);

        // The render pass can be thought of a single large command consisting of sub commands. Here we
        // begin a render pass that outputs to the frame's texture. Then we add sub-commands for
        // setting the bind group, render pipeline, vertex buffers and then finally drawing.
        {
            let mut render_pass = wgpu::RenderPassBuilder::new()
                .color_attachment(&texture_view, |color| color)
                .begin(&mut encoder);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // We want to draw the whole range of vertices, and we're only drawing one instance of them.
            let vertex_range = 0..self.vertecies.len() as u32;
            let instance_range = 0..1;
            render_pass.draw(vertex_range, instance_range);
        }
        //wgpu::clear_texture(&texture_view, wgpu::Color::WHITE, &mut encoder);

        // Now we're done! The commands we added will be submitted after `view` completes.
        window.queue().submit(Some(encoder.finish()));

        /*   {
            let mut clear_encoder = window.swap_chain_device().create_command_encoder(&desc);
            window.swap_chain_queue().submit(Some(clear_encoder.finish()));
        } */
    }

    fn output(&self) -> TextureView {
        self.texture.view().build() //Building a TextureView and move it out
    }

    fn snapshot(
        &self,
        window: &Window,
        texture_capturer: &wgpu::TextureCapturer,
    ) -> wgpu::TextueSnapshot {
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture capture"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);

        let snapshot = texture_capturer.capture(device, &mut encoder, &self.texture);

        window.queue().submit(Some(encoder.finish()));

        snapshot
    }
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex2D]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

fn uniforms_as_bytes(uniforms: &Uniforms2D) -> &[u8] {
    unsafe { wgpu::bytes::from(uniforms) }
}
