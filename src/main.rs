#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;

use microbit::{
    hal::prelude::*,
    hal::gpio::Level,
    hal::uarte::{self, Baudrate, Parity, Pins},
};

use cortex_m_rt::entry;

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let uart: Pins = Pins {
	txd: board.pins.p0_02.into_push_pull_output(Level::High).into(),
        rxd: board.pins.p0_03.into_floating_input().into(),
        cts: None,
        rts: None,
    };

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            // board.uart.into(),
            uart,
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {
        write!(serial, "Hello World:\r\n").unwrap();
        let input = nb::block!(serial.read()).unwrap();
        write!(serial, "You said: {}\r\n", input as char).unwrap();
    }
}
