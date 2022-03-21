use crate::{ParameterStore, TextureNode, TextureTarget};
use nannou::wgpu::{Device, TextureView};
use nannou::window::Window;
use nannou::{App, Draw};

///
/// Defines a Model for a [TextureModelNode]
///
/// This is used to easily implement different TextureNodes
/// A Model gets updated every frame and generates a [nannou::Draw] output that can be rendered
///
pub trait ModelUpdate {
    ///
    /// update the model and generate a output draw from the model
    /// 
    /// # Parameters
    /// - `app`:  nannou app
    /// - `store`: input osc parameter 
    /// - `input`: vector of input textures if any 
    /// 
    /// # Returns:
    /// [nannou::Draw] with the drawn frame in it
    fn update_model(&mut self, app: &App, store: &ParameterStore, input: Vec<TextureView>) -> Draw;
}

/// Node that applies a Model to a texture.
/// this node can be part of a [crate::TextureTree]
pub struct TextureModelNode<T> {
    texture: TextureTarget,
    model: T,
}

impl<T> TextureNode for TextureModelNode<T>
where
    T: ModelUpdate,
{
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

    fn snapshot(
        &self,
        window: &Window,
        texture_capturer: &nannou::wgpu::TextureCapturer,
    ) -> nannou::wgpu::TextueSnapshot {
        self.texture.snapshot(window, texture_capturer)
    }
}

impl<T> TextureModelNode<T> {
    pub fn new(model: T, device: &Device, size: [u32; 2]) -> Self {
        let texture = TextureTarget::new(device, size);

        Self { texture, model }
    }
}
