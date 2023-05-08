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
    fn into_osc(self) -> Type;
}

impl FromOscType for i32 {
    fn from_osc(t: Type) -> Option<Self> {
        t.int()
    }

    fn into_osc(self) -> Type {
        Type::Int(self)
    }
}

impl FromOscType for f32 {
    fn from_osc(t: Type) -> Option<Self> {
        t.float()
    }
    fn into_osc(self) -> Type {
        Type::Float(self)
    }
}

impl FromOscType for String {
    fn from_osc(t: Type) -> Option<Self> {
        t.string()
    }
    fn into_osc(self) -> Type {
        Type::String(self)
    }
}

impl FromOscType for Vec<u8> {
    fn from_osc(t: Type) -> Option<Self> {
        t.blob()
    }
    fn into_osc(self) -> Type {
        Type::Blob(self)
    }
}

impl FromOscType for i64 {
    fn from_osc(t: Type) -> Option<Self> {
        t.long()
    }
    fn into_osc(self) -> Type {
        Type::Long(self)
    }
}

impl FromOscType for f64 {
    fn from_osc(t: Type) -> Option<Self> {
        t.double()
    }
    fn into_osc(self) -> Type {
        Type::Double(self)
    }
}

impl FromOscType for char {
    fn from_osc(t: Type) -> Option<Self> {
        t.char()
    }
    fn into_osc(self) -> Type {
        Type::Char(self)
    }
}

impl FromOscType for bool {
    fn from_osc(t: Type) -> Option<Self> {
        t.bool()
    }
    fn into_osc(self) -> Type {
        Type::Bool(self)
    }
}

impl FromOscType for Color {
    fn from_osc(t: Type) -> Option<Self> {
        t.color()
    }
    fn into_osc(self) -> Type {
        Type::Color(self)
    }
}

impl FromOscType for MidiMessage {
    fn from_osc(t: Type) -> Option<Self> {
        t.midi()
    }
    fn into_osc(self) -> Type {
        Type::Midi(self)
    }
}
