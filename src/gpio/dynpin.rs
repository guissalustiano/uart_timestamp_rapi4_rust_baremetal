/// Value-level `enum` for disabled configurations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DynDisabled {
    Floating,
    PullDown,
    PullUp,
    BusKeep,
}

/// Value-level `enum` for input configurations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DynInput {
    Floating,
    PullDown,
    PullUp,
    BusKeep,
}

/// Value-level `enum` for output configurations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DynOutput {
    PushPull,
    Readable,
}

/// Value-level `enum` for output configurations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DynFunction {
    Spi,
    Xip,
    Uart,
    I2C,
    Pwm,
    Pio0,
    Pio1,
    Clock,
    UsbAux,
}

/// Value-level `enum` representing pin modes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DynPinMode {
    Disabled(DynDisabled),
    Input(DynInput),
    Output(DynOutput),
    Function(DynFunction),
}

/// Value-level `struct` representing pin IDs
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub struct DynPinId {
    pub num: u8,
}
impl DynPinId {
    pub fn group(&self) -> u8 {
        self.num / 32
    }

    pub fn fsel_group(&self) -> u8 {
        self.num / 10
    }

    pub fn pull_group(&self) -> u8 {
        self.num / 15
    }
}
