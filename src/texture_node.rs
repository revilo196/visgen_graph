use crate::{ParameterStore, TextureNode, TextureTarget};
use nannou::wgpu::{Device, TextureView};
use nannou::window::Window;
use nannou::{App, Draw};

///
/// Defines a Model for a [TextureModelNode]
///
/// This is used to easily implement different TextureNodes
///
pub trait ModelUpdate {
    fn update_model(&mut self, app: &App, store: &ParameterStore, input: Vec<TextureView>) -> Draw;
}

pub struct TextureModelNode<T> {
    texture: TextureTarget,
    model: T,
}

impl<T> TextureNode for TextureModelNode<T>
where
    T: ModelUpdate,
{
    //input can't be cloned/copied in final version
    fn update(
        &mut self,
        app: &App,
        window: &Window,
        store: &ParameterStore,
        input: Vec<TextureView>,
    ) {
        let draw = self.model.update_model(app, store, input);
        self.texture.submit(window, &draw);
    }

    fn output(&self) -> TextureView {
        self.texture.texture_view() //Building a TextureView and move it out
    }

    fn snapshot(&self, window: &Window, texture_capturer: &nannou::wgpu::TextureCapturer) -> nannou::wgpu::TextueSnapshot {
        self.texture.snapshot(window, texture_capturer)
    }
}

impl<T> TextureModelNode<T> {
    pub fn new(model: T, device: &Device, size: [u32; 2]) -> Self {
        let texture = TextureTarget::new(device, size);

        Self { texture, model }
    }
}
