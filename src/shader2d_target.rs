
use crate::Vertex2D;
use nannou::prelude::*;
use nannou::wgpu::{
    CommandEncoderDescriptor, CommandEncoder, Device, Texture, TextureBuilder, TextureUsage, TextureView,
};

pub struct Shader2DTarget<T> {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    texture: Texture,
    uniforms : T,
    encoder: Option<CommandEncoder>,
    vertex_len : usize,
    index_len: usize,
}

impl<T> Shader2DTarget<T> where T : Copy, T: Clone {
    pub fn new(device: &Device,
        texture_size: [u32; 2],
        vert: &[u8],
        frag: &[u8],
        vertecies: &[Vertex2D],
        indecies: &[u16],
        uniform: T) -> Self{
        let format = Frame::TEXTURE_FORMAT;
        let vs_mod = wgpu::shader_from_spirv_bytes(device, vert);
        let fs_mod = wgpu::shader_from_spirv_bytes(device, frag);

        // Frame Texture
        let texture = TextureBuilder::new()
        .size(texture_size)
        .usage(TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_DST | TextureUsage::SAMPLED )
        .sample_count(1)
        .format(format)
        .build(device);

        let vertices_bytes = vertices_as_bytes(&vertecies[..]);
        let usage = wgpu::BufferUsage::VERTEX;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage,
        });

        let indecies_bytes = indecies_as_bytes(&indecies[..]);
        let index_usage = wgpu::BufferUsage::INDEX;
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: indecies_bytes,
            usage: index_usage,
        });

        let uniforms = uniform;
        let uniforms_bytes = uniforms_as_bytes(&uniforms);
        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        });

        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::VERTEX_FRAGMENT, false)
            .build(device);
        let bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<T>(&uniform_buffer, 0..1)
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
            index_buffer,
            render_pipeline,
            uniform_buffer,
            texture,
            uniforms,
            encoder: None,
            vertex_len: vertecies.len(),
            index_len: indecies.len(),
        }

    }
    pub fn begin(&mut self, device: &Device) {
        let desc = CommandEncoderDescriptor {
            label: Some("Shader2DTarget"), 
        };
        self.encoder = Some(device.create_command_encoder(&desc));
    }

    // change the uniforms_buffer 
    // must be placed between begin & submit to take effect
    pub fn set_uniforms(&mut self,device: &Device, uniform: T) {
        self.uniforms = uniform;
        if let Some(encoder) = self.encoder.as_mut() {
            let uniforms_size = std::mem::size_of::<T>() as wgpu::BufferAddress;
            let uniforms_bytes = uniforms_as_bytes(&self.uniforms);
            let usage = wgpu::BufferUsage::COPY_SRC;
            let new_uniform_buffer =
                device
                    .create_buffer_init(&BufferInitDescriptor {
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
        }
    }

    // change the mesh 
    // must be placed between begin & submit to take effect
    pub fn set_mesh(&mut self, vertecies: &[Vertex2D],  indecies: &[u16]){
        if let Some(encoder) = self.encoder.as_mut() {
            todo!();
        }
    }

    pub fn render_pass(&mut self) {
        if let Some(encoder) = self.encoder.as_mut() {

        let texture_view = self.texture.view().build();
        let mut render_pass = wgpu::RenderPassBuilder::new()
                .color_attachment(&texture_view, |color| color)
                .begin(encoder);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // We want to draw the whole range of vertices, and we're only drawing one instance of them.
            let vertex_range = 0..self.vertex_len as u32;
            let instance_range = 0..1;
            render_pass.draw(vertex_range, instance_range);

        }
    }

    // submits the comands stored since [begin()]
    pub fn end(&mut self, window: &Window) {
        let encoder = self.encoder.take();

        if let Some(encoder) = encoder {
            window.swap_chain_queue().submit(Some(encoder.finish()));
        }
    }   

    pub fn texture_view(&self) -> TextureView {
        self.texture.view().build()
    }
    
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex2D]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn indecies_as_bytes(data: &[u16]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
fn uniforms_as_bytes<T>(uniforms: &T) -> &[u8] where T : Copy {
    unsafe { wgpu::bytes::from(uniforms) }
}
