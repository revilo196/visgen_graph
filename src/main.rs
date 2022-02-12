use std::sync::mpsc;

use indextree::Arena;
use nannou::prelude::*;
use nannou_osc as osc;
use visgen_graph::combiner::fader_node::FaderNode;
use visgen_graph::combiner::masking_node::MaskingNode;
use visgen_graph::generators::circles::CircleGenerator;
use visgen_graph::program::program::ProgramManager;
use visgen_graph::generators::clouds::CloudsNode;
use visgen_graph::generators::stripes::StripeGenerator;
use visgen_graph::generators::wave::WaveTextureNode;
use visgen_graph::{ParameterStore, TextureModelNode, TextureNode, TextureTree};

pub const DEFAULT_POWER_PREFERENCE : wgpu::PowerPreference = wgpu::PowerPreference::HighPerformance;

struct Model {
    receiver: osc::Receiver,
    tree: TextureTree,
    store: ParameterStore,
    program: ProgramManager,
    texture_capturer: wgpu::TextureCapturer,
    ndi_stream: visgen_graph::util::ndi_stream::NdiStream,
    lasttime: f32,
}

fn main() {
    ndi::initialize().unwrap();

    nannou::app(model).backends(wgpu::Backends::VULKAN)
        .update(update) // rather than `.event(event)`, now we only subscribe to updates
        .exit(exit)
        .run();

    unsafe {ndi::cleanup();}
}

const PORT: u16 = 6060;

fn model(app: &App) -> Model {
    let texture_size = [512, 512];
    let mut store = ParameterStore::new();

    // to use precompiled SPIRV(GLSL) shaders without decompilation(naga)
    let device_desc = wgpu::DeviceDescriptor{
        label: Some("SPIRV SHADERS"),
        features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
        limits: wgpu::Limits::default(),
    };

    // Create the window.
    let [win_w, win_h] = [texture_size[0], texture_size[1]];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .device_descriptor(device_desc)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();
    let tree = build_tree(&window, texture_size, &mut store);

    println!("{:?}", store);

    let receiver: osc::Receiver = osc::receiver(PORT).unwrap();
    let program = ProgramManager::new();

    let texture_capturer = wgpu::TextureCapturer::default();
    let ndi_stream = visgen_graph::util::ndi_stream::NdiStream::new("visgen_graph".to_string(), 60);

    Model {
        receiver,
        tree,
        store,
        program,
        texture_capturer,
        ndi_stream,
        lasttime : 0f32,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    //OSC Receiving
    for (packet, _) in model.receiver.try_iter() {
        if let osc::Packet::Message(message) = packet {
            model.store.update(&message);
            model.program.update_osc(app.time, &model.store, &message);
            println!("{:?}", message);
        }
    }

    model.program.update(app.time, &mut model.store);
    println!("runtime {} timing {} fps {} ", app.time, app.time-model.lasttime, 1.0f32/(app.time-model.lasttime));
    model.lasttime = app.time;
    // Update the Model Tree
    let win = app.main_window();
    model.tree.update(app, &win, &model.store);


    let snapshot =  model.tree.snapshot(&win, &model.texture_capturer);
    let timecode = (app.time * 10000f32) as i64; 

    // send the last queued image in the stream, and queue the next snapshot
    // this is slow but for now this works 20-30fps
    model.ndi_stream.update_snapshot(snapshot, timecode);

}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    frame.clear(BLACK);

    draw.texture(&model.tree.output());
    
    //draw.ellipse().x_y(0.1, 0.1).radius(5.0).color(RED); // test primitive

    draw.to_frame(app, &frame).unwrap();
}

fn build_tree(win: &Window, size: [u32; 2], store: &mut ParameterStore) -> TextureTree {
    // Create a new arena

    let device = win.device();

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
   /*  let b = arena.new_node(Box::new(WaveTextureNode::new(
        "wave".to_string(),
        size,
        store,
        device,
    )));*/



    //let circles = CircleGenerator::new("circles".to_string(), size, store);
    //let c = arena.new_node(Box::new(TextureModelNode::new(circles, device, size)));

/*     let d = arena.new_node( Box::new(MaskingNode::new(
        "mask".to_string(),
        size,
        store,
        device
    )));*/


    let e = arena.new_node(Box::new(CloudsNode::new(
        "clouds".to_string(),
        size,
        store,
        device,
    )));

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
    
    c.append(f, &mut arena);
    f.append(g, &mut arena);
    f.append(h, &mut arena);*/

    //d.append(b, &mut arena);
    //d.append(c, &mut arena);
    //d.append(a, &mut arena); // stripes as mask

    TextureTree::new(arena, e)
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
    println!("Done!");
}