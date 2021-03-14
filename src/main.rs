#![no_std]
#![no_main]

mod timer;

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::delay_ms;
use arduino_uno::prelude::*;
use rumba::Rumba;
use timer::{AvrTimer1, U32Ext};

fn app() {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        115200_u32.into_baudrate(),
    );

    let mut timer = AvrTimer1::new(dp.TC1);
    timer.start(115200.hz());

    let rumba_serial =
        bitbang_hal::serial::Serial::new(pins.d2.into_output(&mut pins.ddr), pins.d3, timer);
    let rumba = Rumba::new(rumba_serial);

    ufmt::uwriteln!(&mut serial, "Starting Roomba").unwrap();
    let mut rumba = rumba.into_passive();
    delay_ms(1000);
    ufmt::uwriteln!(&mut serial, "Sending first song").unwrap();
    rumba.send_song(rumba::SongSlot::First).unwrap();
    delay_ms(1000);
    let mut rumba = rumba.into_safe();
    delay_ms(1000);
    ufmt::uwriteln!(&mut serial, "Playing first song").unwrap();
    rumba.play_song(rumba::SongSlot::First).unwrap();
    delay_ms(2500);
    ufmt::uwriteln!(&mut serial, "Stoping Roomba").unwrap();

    rumba.into_off();
}

#[arduino_uno::entry]
fn main() -> ! {
    app();
    loop {}
}
