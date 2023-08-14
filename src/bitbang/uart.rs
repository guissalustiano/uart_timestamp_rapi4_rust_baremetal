use embedded_hal as hal;
use nb;

use crate::{time::spin_for, gpio::pin::{Pin, PinId, Gpio42, PushPullOutput, AnyPin, SpecificPin}};
use hal::digital::v2::OutputPin;

use core::{time::Duration, convert::Infallible};

pub enum ParityMode{
    None,
    Odd,
    Even
}

#[derive(Copy, Clone)]
pub enum StopBitsOption{
    Zero,
    One,
    Two
}

pub struct SoftUartTransmitter<T> where T:AnyPin<Mode = PushPullOutput>{
    tx_pin: SpecificPin<T>,
    baud_rate: u32,    // Baud rate in bauds/s
    stop_bits: StopBitsOption,  // Number of stop bits
    parity: ParityMode,  // Parity mode
}

impl<T> SoftUartTransmitter<T> where T:AnyPin<Mode = PushPullOutput>{
    pub fn new(
        tx_pin: SpecificPin<T>,
        baud_rate: u32,    // Baud rate in bauds/s
        stop_bits: StopBitsOption,  // Number of stop bits
        parity: ParityMode,  // Parity mode
    ) -> Self {
        SoftUartTransmitter{
            tx_pin,
            baud_rate,
            stop_bits,
            parity,
        }
    }

    pub fn get_baud_rate(&self) -> u32{
        self.baud_rate
    }

}

impl <T> hal::serial::Write<u8> for SoftUartTransmitter<T> where T:AnyPin<Mode = PushPullOutput> {
    type Error = Infallible;

    fn write(&mut self, word:u8) -> nb::Result<(), Self::Error>{
        // Emmit start bit
        // println!("Simulated transmission:");  -- Maybe add debug print
        self.tx_pin.set_low().unwrap();
        spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));

        // Emmit data
        for shift in 0..8 {
            let curr_data = (word>>shift)&1;
            self.tx_pin.set_state((curr_data != 0).into()).unwrap();
            spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
        }

        // Emmit parity
        let p = word.count_ones() % 2;
        match self.parity {
            ParityMode::Even => {
                self.tx_pin.set_state((p != 0).into()).unwrap();
                spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
            }
            ParityMode::Odd => {
                self.tx_pin.set_state((p != 0).into()).unwrap();
                spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
            },
            ParityMode::None => {

            }
        }

        // Emmit stop bits
        for _i in 0..self.stop_bits as u8 {
            self.tx_pin.set_high().unwrap();
            spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
        }

        // println!("End of transmission"); -- Maybe add debug print
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Does nothing, since there is no buffer
        Ok(())
    }
}