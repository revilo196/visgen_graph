mod fragment_node;
mod osc_convert;
mod parameter;
pub mod shader_target;
pub mod shapes;
mod texture_node;
mod texture_target;
mod texture_tree;

pub use fragment_node::*;
pub use parameter::*;
pub use shapes::Vertex2D;
pub use texture_node::*;
pub use texture_target::*;
pub use texture_tree::*;

pub mod generators {
    pub mod circles;
    pub mod clouds;
    pub mod stripes;
    pub mod wave;
}

pub mod util {
    pub mod ndi_stream;
    pub mod shader;
}

pub mod program {
    pub mod program;
}
pub mod combiner {
    pub mod fader_node;
    pub mod masking_node;
    pub mod shader_combiner;
}
