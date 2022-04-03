/// Some Basic Shapes and Vector Formats

/// Vertexes of a QUAD that fills the full screen
pub const FULL_SCREEN_QUAD: [Vertex2D; 4] = [
    Vertex2D {
        position: [-1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, -1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, -1.0],
    },
];

/// Indices of a QUAD that fills the full screen
pub const FULL_SCREEN_QUAD_INDEX: [u16; 6] = [1, 2, 3, 4, 3, 2];

/// Vertexes of a Triangle that fills the full screen
pub const FULL_TRI: [Vertex2D; 3] = [
    Vertex2D {
        position: [-1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, -1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
    },
];
/// Indices of a Triangle that fills the full screen
pub const FULL_TRI_INDEX: [u16; 3] = [1, 2, 3];

/// simple 2D-Vector that can be send to GPU/Shader
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex2D {
    pub position: [f32; 2],
}

/// simple 3D-Vector that can be send to GPU/Shader
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex3D {
    pub position: [f32; 3],
}

/// simple 3D-Vector that can be send to GPU/Shader
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex4D {
    pub position: [f32; 4],
}
