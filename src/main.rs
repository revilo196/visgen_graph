use indextree::Arena;
use nannou::prelude::*;
use nannou::wgpu::{Device, TextureView};
use visgen_graph::{TextureNode, TextureTarget, TextureTree};

struct NodeModel {
    texture: TextureTarget,
    internal: i64,
}

impl TextureNode for NodeModel {
    //input can't be cloned/copied in final version
    fn update(&mut self, _app: &App, window: &Window, input: Vec<TextureView>) {
        let draw = Draw::new();

        for t in input {
            draw.texture(&t);
        }

        let factor = self.internal as f32;
        println!("update node : {}", factor);

        draw.quad().x_y(factor, 0f32).w_h(5.0, factor);

        draw.ellipse()
            .x_y(factor - 200.0, factor - 200.0)
            .radius(factor / 2.0)
            .color(Rgb::new(255, 128, factor as u8));

        self.texture.submit(window, &draw);
    }

    fn output(&self) -> TextureView {
        self.texture.texture_view() //Building a TextureView and move it out
    }
}

impl NodeModel {
    fn new(internal: i64, device: &Device, size: [u32; 2]) -> Self {
        let texture = TextureTarget::new(device, size);

        Self { texture, internal }
    }
}

struct Model {
    tree: TextureTree,
}

fn main() {
    nannou::app(model)
        .update(update) // rather than `.event(event)`, now we only subscribe to updates
        .run();
}

fn model(app: &App) -> Model {
    let texture_size = [512, 512];

    // Create the window.
    let [win_w, win_h] = [texture_size[0], texture_size[1]];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();

    let tree = build_tree(&window, texture_size);
    Model { tree }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.main_window();
    model.tree.update(app, &win);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.texture(&model.tree.output());

    draw.ellipse().x_y(0.1, 0.1).radius(5.0).color(RED);

    draw.to_frame(app, &frame).unwrap();
}

fn build_tree(win: &Window, size: [u32; 2]) -> TextureTree {
    // Create a new arena

    let device = win.swap_chain_device();

    let mut arena: Arena<Box<dyn TextureNode>> = Arena::new();

    // Add some new nodes to the arena
    let a = arena.new_node(Box::new(NodeModel::new(10, device, size)));
    let b = arena.new_node(Box::new(NodeModel::new(20, device, size)));
    let c = arena.new_node(Box::new(NodeModel::new(30, device, size)));
    let d = arena.new_node(Box::new(NodeModel::new(40, device, size)));
    let e = arena.new_node(Box::new(NodeModel::new(50, device, size)));
    let f = arena.new_node(Box::new(NodeModel::new(60, device, size)));
    let g = arena.new_node(Box::new(NodeModel::new(70, device, size)));
    let h = arena.new_node(Box::new(NodeModel::new(80, device, size)));

    // Build tree
    //           a
    //        b     c
    //       d e    f
    //             g h
    a.append(b, &mut arena);
    a.append(c, &mut arena);
    b.append(d, &mut arena);
    b.append(e, &mut arena);
    c.append(f, &mut arena);
    f.append(g, &mut arena);
    f.append(h, &mut arena);

    TextureTree::new(arena, a)
}
