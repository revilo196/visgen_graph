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

pub const FULL_SCREEN_QUAD_INDEX: [u16; 6] = [1, 2, 3, 4, 3, 2];

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

pub const FULL_TRI_INDEX: [u16; 3] = [1, 2, 3];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex2D {
    pub position: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex3D {
    pub position: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex4D {
    pub position: [f32; 4],
}
