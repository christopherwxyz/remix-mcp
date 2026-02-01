//! OSC message building utilities.

use rosc::OscType;

/// Builder for OSC messages with typed arguments.
pub struct OscMessageBuilder {
    args: Vec<OscType>,
}

impl OscMessageBuilder {
    /// Create a new message builder.
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    /// Add an integer argument.
    pub fn int(mut self, value: i32) -> Self {
        self.args.push(OscType::Int(value));
        self
    }

    /// Add a float argument.
    pub fn float(mut self, value: f32) -> Self {
        self.args.push(OscType::Float(value));
        self
    }

    /// Add a string argument.
    pub fn string(mut self, value: impl Into<String>) -> Self {
        self.args.push(OscType::String(value.into()));
        self
    }

    /// Add a boolean argument (as int 0/1).
    pub fn bool(mut self, value: bool) -> Self {
        self.args.push(OscType::Int(if value { 1 } else { 0 }));
        self
    }

    /// Build the argument list.
    pub fn build(self) -> Vec<OscType> {
        self.args
    }
}

impl Default for OscMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
