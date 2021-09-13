use indextree::Arena;
use nannou::prelude::*;
use visgen_graph::generators::circles::CircleGenerator;
use visgen_graph::{ParameterStore, TextureModelNode, TextureNode, TextureTree};
use nannou_osc as osc;

struct Model {
    receiver: osc::Receiver,
    tree: TextureTree,
    store: ParameterStore,
}

fn main() {
    nannou::app(model)
        .update(update) // rather than `.event(event)`, now we only subscribe to updates
        .run();
}

const PORT: u16 = 6060;

fn model(app: &App) -> Model {
    let texture_size = [512, 512];
    let mut store = ParameterStore::new();

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

    let tree = build_tree(&window, texture_size, &mut store);
    
    println!("{:?}", store);

    let receiver : osc::Receiver = osc::receiver(PORT).unwrap();

    Model { tree, store,receiver }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    for (packet, _) in model.receiver.try_iter() {
        if let osc::Packet::Message(message) = packet {
            model.store.update(&message);
            println!("{:?}", message);
        }
    }

    let win = app.main_window();
    model.tree.update(app, &win, &model.store);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    draw.texture(&model.tree.output());

    draw.ellipse().x_y(0.1, 0.1).radius(5.0).color(RED);

    draw.to_frame(app, &frame).unwrap();
}

fn build_tree(win: &Window, size: [u32; 2], store: &mut ParameterStore) -> TextureTree {
    // Create a new arena

    let device = win.swap_chain_device();

    let mut arena: Arena<Box<dyn TextureNode>> = Arena::new();

    // Add some new nodes to the arena
    //let stripe = StripeGenerator::new("Stripes".to_string(), size, store);
    //let a = arena.new_node(Box::new(TextureModelNode::new(stripe,device,size)));

    /*let a = arena.new_node(Box::new(Shader2DNode::new(
        device,
        size,
        include_bytes!("shaders/vert.spv"),
        include_bytes!("shaders/frag.spv"),
        &FULL_SCREEN_QUAD,
    )));*/
    //let a = arena.new_node(Box::new(WaveTextureNode::new(
    //    "wave".to_string(),
    //    size,
    //    store,
    //    device,
    //)));

    let circles = CircleGenerator::new("circles".to_string(), size, store);
    let a = arena.new_node(Box::new(TextureModelNode::new(circles,device,size)));

    /* let b = arena.new_node(Box::new(NodeModel::new(20, device, size)));
    let c = arena.new_node(Box::new(NodeModel::new(30, device, size)));
    let d = arena.new_node(Box::new(NodeModel::new(40, device, size)));
    let e = arena.new_node(Box::new(NodeModel::new(50, device, size)));
    let f = arena.new_node(Box::new(NodeModel::new(60, device, size)));
    let g = arena.new_node(Box::new(NodeModel::new(70, device, size)));
    let h = arena.new_node(Box::new(NodeModel::new(80, device, size)));*/

    // Build tree
    //           a
    //        b     c
    //       d e    f
    //             g h
    /*  a.append(b, &mut arena);
    a.append(c, &mut arena);
    b.append(d, &mut arena);
    b.append(e, &mut arena);
    c.append(f, &mut arena);
    f.append(g, &mut arena);
    f.append(h, &mut arena);*/

    TextureTree::new(arena, a)
}
