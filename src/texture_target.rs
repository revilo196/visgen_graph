use nannou::wgpu::{
    CommandEncoderDescriptor, Device, TextueSnapshot, Texture, TextureBuilder, TextureCapturer,
    TextureUsages, TextureView,
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
            .usage(
                TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_DST
                    | TextureUsages::TEXTURE_BINDING,
            )
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

    /// get the size of texture inside
    pub fn size(&self) -> [u32; 2] {
        self.texture.size()
    }

    /// Submit a Draw object filled with draw commands to the renderer
    /// and render to the texture.
    pub fn submit(&mut self, window: &Window, draw: &Draw) {
        let device = window.device();

        let ce_desc = CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self.renderer
            .render_to_texture(device, &mut encoder, draw, &self.texture);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.queue().submit(Some(encoder.finish()));
    }
}
