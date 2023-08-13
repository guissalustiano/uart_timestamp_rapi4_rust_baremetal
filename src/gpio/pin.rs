use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::v2::{InputPin, OutputPin};

use super::dynpin::{DynInput, DynOutput, DynPinId, DynPinMode};
use super::reg::RegisterInterface;

pub enum Floating {}
pub enum PullDown {}
pub enum PullUp {}
pub enum BusKeep {}

pub trait InputConfig {
    const DYN: DynInput;
}

impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
}
impl InputConfig for BusKeep {
    const DYN: DynInput = DynInput::BusKeep;
}

pub struct Input<C: InputConfig> {
    _config: PhantomData<C>,
}

pub type FloatingInput = Input<Floating>;
pub type PullDownInput = Input<PullDown>;
pub type PullUpInput = Input<PullUp>;
pub type BusKeepInput = Input<BusKeep>;

pub trait OutputConfig {
    const DYN: DynOutput;
}

pub enum PushPull {}
pub enum Readable {}

impl OutputConfig for PushPull {
    const DYN: DynOutput = DynOutput::PushPull;
}
impl OutputConfig for Readable {
    const DYN: DynOutput = DynOutput::Readable;
}

pub struct Output<C: OutputConfig> {
    _config: PhantomData<C>,
}

pub type PushPullOutput = Output<PushPull>;
pub type ReadableOutput = Output<Readable>;

pub trait PinMode {
    const DYN: DynPinMode;
}

impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
}
impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}

pub trait PinId {
    const DYN: DynPinId;
    type Reset;
}

pub struct Registers<I: PinId> {
    _id: PhantomData<I>,
}
unsafe impl<I: PinId> RegisterInterface for Registers<I> {
    #[inline]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

impl<I: PinId> Registers<I> {
    /// Create a new instance of [`Registers`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PinId`]
    #[inline]
    unsafe fn new() -> Self {
        Registers { _id: PhantomData }
    }

    /// Provide a type-level equivalent for the
    /// [`RegisterInterface::change_mode`] method.
    #[inline(always)]
    fn change_mode<M: PinMode>(&mut self) {
        RegisterInterface::change_mode(self, M::DYN);
    }
}

pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    registers: Registers<I>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    pub unsafe fn new() -> Self {
        Pin {
            registers: Registers::new(),
            mode: PhantomData,
        }
    }

    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        if N::DYN != M::DYN {
            self.registers.change_mode::<N>();
        }
        unsafe { Pin::new() }
    }
}

pub trait AnyPin
where
    Self: From<SpecificPin<Self>>,
    Self: Into<SpecificPin<Self>>,
{
    type Id: PinId;
    type Mode: PinMode;
}

impl<I, M> AnyPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Id = I;
    type Mode = M;
}

pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

/// [`embedded_hal`] traits
impl<I, C> OutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    type Error = Infallible;
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.registers.write_pin(true);
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.registers.write_pin(false);
        Ok(())
    }
}

impl<I, C> InputPin for Pin<I, Input<C>>
where
    I: PinId,
    C: InputConfig,
{
    type Error = Infallible;
    #[allow(clippy::bool_comparison)] // more explicit this way
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.registers.read_pin() == true)
    }
    #[allow(clippy::bool_comparison)] // more explicit this way
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.registers.read_pin() == false)
    }
}

/// Specific pin implementations
pub enum Gpio42 {}
impl PinId for Gpio42 {
    type Reset = PullDownInput;
    const DYN: DynPinId = DynPinId { num: 42 };
}

pub enum Gpio0 {}
impl PinId for Gpio0 {
    type Reset = PullUpInput;
    const DYN: DynPinId = DynPinId { num: 0 };
}
