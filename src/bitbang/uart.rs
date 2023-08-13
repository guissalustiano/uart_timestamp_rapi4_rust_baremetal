extern crate embedded_hal as hal;
extern crate nb;

use std::{thread, time::Duration};

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
    system_clock: u32, // System clock in Hz
    baud_rate: u32,    // Baud rate in bauds/s
    stop_bits: StopBitsOption,  // Number of stop bits
    parity: ParityMode,  // Parity mode

    num_clocks_to_push: u32,
}

impl SoftUartTransmitter{
    pub fn new(
        system_clock: u32, // System clock in Hz
        baud_rate: u32,    // Baud rate in bauds/s
        stop_bits: StopBitsOption,  // Number of stop bits
        parity: ParityMode,  // Parity mode
    ) -> Self {
        SoftUartTransmitter{
            system_clock: system_clock,
            baud_rate: baud_rate,
            stop_bits: stop_bits,
            parity: parity,
            num_clocks_to_push: system_clock/baud_rate
        }
    }

    pub fn get_baud_rate(&self) -> u32{
        self.baud_rate
    }

    pub fn get_system_clock(&self) -> u32{
        self.system_clock
    }

    pub fn get_clocks_iter(&self) -> u32{
        self.num_clocks_to_push
    }
}

#[derive(Debug)]
pub struct UartError;

impl hal::serial::Write<u8> for SoftUartTransmitter {
    type Error = UartError;

    fn write(&mut self, word:u8) -> nb::Result<(), Self::Error>{
        // Emmit start bit
        println!("Simulated transmission:");
        println!("0"); // Replace with GPIO
        thread::sleep(Duration::from_secs(1/self.baud_rate as u64));

        // Emmit data
        for shift in 0..8 {
            let curr_data = (word>>shift)&1;
            println!("{}", curr_data);
            thread::sleep(Duration::from_secs(1/self.baud_rate as u64));
        }

        // Emmit parity
        let p = word.count_ones() % 2;
        match self.parity {
            ParityMode::Even => {
                println!("{}", p);
                thread::sleep(Duration::from_secs(1/self.baud_rate as u64));
            }
            ParityMode::Odd => {
                println!("{}", !(p==1) as u8);
                thread::sleep(Duration::from_secs(1/self.baud_rate as u64));
            },
            ParityMode::None => {

            }
        }

        // Emmit stop bits
        for _i in 0..self.stop_bits as u8 {
            println!("1");
            thread::sleep(Duration::from_secs(1/self.baud_rate as u64));
        }

        println!("");
        println!("End of transmission");
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Does nothing, since there is no buffer
        Ok(())
    }
}