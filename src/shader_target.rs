use ::wgpu::ShaderModuleDescriptorSpirV;
use nannou::prelude::*;
use nannou::wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Device, TextueSnapshot, Texture, TextureBuilder,
    TextureCapturer, TextureUsages, TextureView,
};
use std::marker::PhantomData;

/// Render something to a Texture using a SpirV Shader
pub struct ShaderTarget<T, U> {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    texture: Texture,
    uniforms: T,
    encoder: Option<CommandEncoder>,
    vertex_len: usize,
    index_len: usize,
    marker: PhantomData<U>,
}

impl<T, U> ShaderTarget<T, U>
where
    T: Copy,
    T: Clone,
    U: Sized,
    U: Copy,
{
    /// Create a new ShaderTarget
    /// ## Parameters
    /// - `device`: render device [Device]
    /// - `vert`: SpirV vertex shader
    /// - `frag`: SpirV vertex shader
    /// - `vertecies`: for the mesh that should be rendered
    /// - `indecies`: for the mesh that should be rendered
    /// - `uniform` : uniform data struct that is send to the shader
    pub fn new(
        device: &Device,
        texture_size: [u32; 2],
        vert: &ShaderModuleDescriptorSpirV,
        frag: &ShaderModuleDescriptorSpirV,
        vertecies: &[U],
        indecies: &[u16],
        uniform: T,
    ) -> Self {
        // create the shaders
        // spirv shades are compiled binary created outside this app, and could be unsafe
        let vs_mod = unsafe { device.create_shader_module_spirv(vert) };
        let fs_mod = unsafe { device.create_shader_module_spirv(frag) };

        // output format
        let format = Frame::TEXTURE_FORMAT;

        // FrameBuffer Texture
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

        // mesh vertex buffer
        let vertices_bytes = vertices_as_bytes(vertecies);
        let usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage,
        });

        // mesh index buffer
        let indecies_bytes = indecies_as_bytes(indecies);
        let index_usage = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST;
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: indecies_bytes,
            usage: index_usage,
        });

        // shader uniform buffer 
        let uniforms = uniform;
        let uniforms_bytes = uniforms_as_bytes(&uniforms);
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        });

        // create the bind and render layout
        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
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
            .add_vertex_buffer::<U>(&nannou::wgpu::vertex_attr_array![0 => Float32x2])
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
            marker: PhantomData,
        }
    }

    /// begin a a new command encoder Queue using this shader
    pub fn begin(&mut self, device: &Device) {
        let desc = CommandEncoderDescriptor {
            label: Some("ShaderTarget"),
        };
        self.encoder = Some(device.create_command_encoder(&desc));
    }

    /// change the uniforms_buffer
    /// must be placed between begin & submit to take effect
    /// 
    /// copies the given uniform data to the GPU
    pub fn set_uniforms(&mut self, device: &Device, uniform: T) {
        self.uniforms = uniform;
        if let Some(encoder) = self.encoder.as_mut() {
            let uniforms_size = std::mem::size_of::<T>() as wgpu::BufferAddress;
            let uniforms_bytes = uniforms_as_bytes(&self.uniforms);
            let usage = wgpu::BufferUsages::COPY_SRC;
            let new_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
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

    /// change the mesh
    /// must be placed between begin & submit to take effect
    /// 
    /// copies the given mesh data to the GPU
    pub fn set_mesh(&mut self, device: &Device, vertecies: &[U], indecies: &[u16]) {
        if let Some(encoder) = self.encoder.as_mut() {
            let vertices_bytes = vertices_as_bytes(vertecies);
            let vertex_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_SRC;
            let new_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: vertices_bytes,
                usage: vertex_usage,
            });

            let indecies_bytes = indecies_as_bytes(indecies);
            let index_usage = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_SRC;
            let new_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: indecies_bytes,
                usage: index_usage,
            });

            encoder.copy_buffer_to_buffer(
                &new_vertex_buffer,
                0,
                &self.vertex_buffer,
                0,
                indecies_bytes.len() as wgpu::BufferAddress,
            );
            self.vertex_len = vertecies.len();

            encoder.copy_buffer_to_buffer(
                &new_index_buffer,
                0,
                &self.index_buffer,
                0,
                indecies_bytes.len() as wgpu::BufferAddress,
            );
            self.index_len = indecies.len();
        }
    }

    /// render pass with shaders
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

    /// submits the commands stored since [begin()]
    pub fn end(&mut self, window: &Window) {
        let encoder = self.encoder.take();

        if let Some(encoder) = encoder {
            window.queue().submit(Some(encoder.finish()));
        }
    }

    /// get the output texture / FrameBuffer that we renderd to 
    pub fn texture_view(&self) -> TextureView {
        self.texture.view().build()
    }

    /// create a snapshot of the output texture, to be transferred to the CPU
    pub fn snapshot(&self, window: &Window, texture_capturer: &TextureCapturer) -> TextueSnapshot {
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
fn vertices_as_bytes<U>(data: &[U]) -> &[u8]
where
    U: Copy,
    U: Sized,
{
    unsafe { wgpu::bytes::from_slice(data) }
}
// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn indecies_as_bytes(data: &[u16]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn uniforms_as_bytes<T>(uniforms: &T) -> &[u8]
where
    T: Copy,
    T: Sized,
{
    unsafe { wgpu::bytes::from(uniforms) }
}

// some pre-made types using simple 2D and 3D Vertex
use crate::shapes::{Vertex2D, Vertex3D};
pub type Shader2DTarget<T> = ShaderTarget<T, Vertex2D>;
pub type Shader3DTarget<T> = ShaderTarget<T, Vertex3D>;
