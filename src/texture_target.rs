use nannou::wgpu::{
    CommandEncoderDescriptor, Device, Texture, TextureBuilder, TextureUsage, TextureView,
};
use nannou::window::Window;
use nannou::{Draw, Frame};



///
/// Render to a Texture instead of the Screen
///
pub struct TextureTarget {
    /// The texture that we will draw to.
    texture: Texture,
    /// The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
}

impl TextureTarget {
    pub fn new(device: &Device, texture_size: [u32; 2]) -> Self {
        let texture = TextureBuilder::new()
            .size(texture_size)
            // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
            // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
            .usage(TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_DST | TextureUsage::SAMPLED)
            // Use nannou's default multisampling sample count.
            .format(Frame::TEXTURE_FORMAT)
            // Build it!
            .build(device);

        // Create our `Draw` instance and a renderer for it.
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        TextureTarget { texture, renderer }
    }

    /// get an texture view to the target texture.
    /// this texture can then be used to draw to oder textures
    pub fn texture_view(&self) -> TextureView {
        self.texture.view().build()
    }

    /// get the size of texture inside
    pub fn size(&self) -> [u32; 2] {
        self.texture.size()
    }

    /// Submit a Draw object filled with draw commands to the renderer
    /// and render to the texture.
    pub fn submit(&mut self, window: &Window, draw: &Draw) {
        let device = window.swap_chain_device();

        let ce_desc = CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self.renderer
            .render_to_texture(device, &mut encoder, draw, &self.texture);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.swap_chain_queue().submit(Some(encoder.finish()));
    }
}
