/// ShaderCombiner similar to [crate::shader_target::ShaderTarget] but makes it possible to add textures to the shader
/// ToDo -- Very duplicate code to ShaderTarget

use nannou::prelude::*;
use nannou::wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Device, Texture, TextureBuilder, TextureUsages,
    TextureView,TextueSnapshot,TextureCapturer
};
use ::wgpu::{ShaderModuleDescriptorSpirV, TextureSampleType};
use std::marker::PhantomData;

/// ShaderCombiner similar to [crate::shader_target::ShaderTarget] but makes it possible to add textures to the shader
pub struct ShaderCombiner<T, U> {
    bind_group_layout : wgpu::BindGroupLayout,
    sampler : wgpu::Sampler,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    texture: Texture,
    uniforms: T,
    encoder: Option<CommandEncoder>,
    vertex_len: usize,
    index_len: usize,
    num_input_textures: usize,
    marker: PhantomData<U>,
}

impl<T, U> ShaderCombiner<T, U>
where
    T: Copy,
    T: Clone,
    U: Sized,
    U: Copy,
{
    pub fn new(
        device: &Device,
        texture_size: [u32; 2],
        vert: &ShaderModuleDescriptorSpirV,
        frag: &ShaderModuleDescriptorSpirV,
        num_input_textures: usize,
        vertecies: &[U],
        indecies: &[u16],
        uniform: T,
    ) -> Self {
        let format = Frame::TEXTURE_FORMAT;
        let vs_mod = unsafe {device.create_shader_module_spirv(vert)};
        let fs_mod = unsafe {device.create_shader_module_spirv(frag)};

        // Frame Texture
        let texture = TextureBuilder::new()
            .size(texture_size)
            .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING)
            .sample_count(1)
            .format(format)
            .build(device);

        let vertices_bytes = vertices_as_bytes(vertecies);
        let usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage,
        });

        let indecies_bytes = indecies_as_bytes(indecies);
        let index_usage = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST;
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: indecies_bytes,
            usage: index_usage,
        });

        let uniforms = uniform;
        let uniforms_bytes = uniforms_as_bytes(&uniforms);
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        });

        // Create the sampler for sampling from the source texture.
        let sampler_desc = wgpu::SamplerBuilder::new().into_descriptor();
        let sampler_filtering = wgpu::sampler_filtering(&sampler_desc);
        let sampler = device.create_sampler(&sampler_desc);

        let mut bind_group_layout_builder = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false);

        //add as many texture as specified
        for _ in 0..num_input_textures {
            bind_group_layout_builder = bind_group_layout_builder.texture(wgpu::ShaderStages::FRAGMENT, false, wgpu::TextureViewDimension::D2,TextureSampleType::Float { filterable: true });
        }

        let bind_group_layout = bind_group_layout_builder.sampler(wgpu::ShaderStages::FRAGMENT, sampler_filtering)
        .build(device);

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
            bind_group_layout,
            sampler,
            vertex_buffer,
            index_buffer,
            render_pipeline,
            uniform_buffer,
            texture,
            uniforms,
            encoder: None,
            vertex_len: vertecies.len(),
            index_len: indecies.len(),
            num_input_textures,
            marker: PhantomData,
        }
    }
    pub fn begin(&mut self, device: &Device) {
        let desc = CommandEncoderDescriptor {
            label: Some("ShaderTarget"),
        };
        self.encoder = Some(device.create_command_encoder(&desc));
    }

    // change the uniforms_buffer
    // must be placed between begin & submit to take effect
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

    // change the mesh
    // must be placed between begin & submit to take effect
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

    pub fn render_pass(&mut self, device: &Device, textures : Vec<TextureView> ) {
        if let Some(encoder) = self.encoder.as_mut() {

            let mut bind_group_builder = wgpu::BindGroupBuilder::new()
                .buffer::<T>(&self.uniform_buffer, 0..1);
            
            //Add as many textures as Specified
            for item in textures.iter().take(self.num_input_textures) {
                bind_group_builder = bind_group_builder.texture_view(item)
            }

            let bind_group = bind_group_builder.sampler(&self.sampler)
                                                        .build(device, &self.bind_group_layout);

            let texture_view = self.texture.view().build();
            let mut render_pass = wgpu::RenderPassBuilder::new()
                .color_attachment(&texture_view, |color| color)
                .begin(encoder);
            render_pass.set_bind_group(0, &bind_group, &[]);
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
            window.queue().submit(Some(encoder.finish()));
        }
    }

    pub fn texture_view(&self) -> TextureView {
        self.texture.view().build()
    }

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
