extern crate embedded_hal as hal;
extern crate nb;

use crate::{time::spin_for, gpio::pin::{Pin, PinId, Gpio42, PushPullOutput}};

use core::time::Duration;

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

pub struct SoftUartTransmitter{
    baud_rate: u32,    // Baud rate in bauds/s
    stop_bits: StopBitsOption,  // Number of stop bits
    parity: ParityMode,  // Parity mode
}

impl SoftUartTransmitter{
    pub fn new(
        baud_rate: u32,    // Baud rate in bauds/s
        stop_bits: StopBitsOption,  // Number of stop bits
        parity: ParityMode,  // Parity mode
    ) -> Self {
        SoftUartTransmitter{
            baud_rate: baud_rate,
            stop_bits: stop_bits,
            parity: parity,
        }
    }

    pub fn get_baud_rate(&self) -> u32{
        self.baud_rate
    }

}

#[derive(Debug)]
pub struct UartError;

impl hal::serial::Write<u8> for SoftUartTransmitter {
    type Error = UartError;

    fn write(&mut self, word:u8) -> nb::Result<(), Self::Error>{
        // Emmit start bit
        // println!("Simulated transmission:");  -- Maybe add debug print
        println!("0"); // Replace with GPIO
        spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));

        // Emmit data
        for shift in 0..8 {
            let curr_data = (word>>shift)&1;
            println!("{}", curr_data);
            spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
        }

        // Emmit parity
        let p = word.count_ones() % 2;
        match self.parity {
            ParityMode::Even => {
                println!("{}", p);
                spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
            }
            ParityMode::Odd => {
                println!("{}", !(p==1) as u8);
                spin_for(Duration::from_nanos(1_000_000_000/self.baud_rate as u64));
            },
            ParityMode::None => {

            }
        }

        // Emmit stop bits
        for _i in 0..self.stop_bits as u8 {
            println!("1");
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