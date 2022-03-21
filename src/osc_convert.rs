use nannou_osc::{Color, MidiMessage, Type};

// todo: look into macros for these?

///
/// defines type based conversion from [Type]
///
pub trait FromOscType
where
    Self: Sized,
{
    /// convert from [Type] to some other type
    fn from_osc(t: Type) -> Option<Self>;
}

impl FromOscType for i32 {
    fn from_osc(t: Type) -> Option<Self> {
        t.int()
    }
}

impl FromOscType for f32 {
    fn from_osc(t: Type) -> Option<Self> {
        t.float()
    }
}

impl FromOscType for String {
    fn from_osc(t: Type) -> Option<Self> {
        t.string()
    }
}

impl FromOscType for Vec<u8> {
    fn from_osc(t: Type) -> Option<Self> {
        t.blob()
    }
}

impl FromOscType for i64 {
    fn from_osc(t: Type) -> Option<Self> {
        t.long()
    }
}

impl FromOscType for f64 {
    fn from_osc(t: Type) -> Option<Self> {
        t.double()
    }
}

impl FromOscType for char {
    fn from_osc(t: Type) -> Option<Self> {
        t.char()
    }
}

impl FromOscType for bool {
    fn from_osc(t: Type) -> Option<Self> {
        t.bool()
    }
}

impl FromOscType for Color {
    fn from_osc(t: Type) -> Option<Self> {
        t.color()
    }
}

impl FromOscType for MidiMessage {
    fn from_osc(t: Type) -> Option<Self> {
        t.midi()
    }
}
