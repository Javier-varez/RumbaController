#![no_std]
#![no_main]

mod timer;

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::delay_ms;
use arduino_uno::prelude::*;
use embedded_hal::serial::{Read, Write};
use rumba::{prelude::*, Note, NoteName, NoteOctave, Rumba};
use timer::{AvrTimer1, U32Ext};

fn clean_a_bit<T>(rumba: &mut Rumba<T, rumba::mode::Passive>) -> Result<(), <T as Write<u8>>::Error>
where
    T: Write<u8> + Read<u8>,
{
    rumba.max_clean()?;
    delay_ms(5000);
    rumba.max_clean()?;
    delay_ms(1000);
    Ok(())
}

fn app() {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&pins.ddr),
        115200_u32.into_baudrate(),
    );

    let mut timer = AvrTimer1::new(dp.TC1);
    timer.start(115200.hz());

    let mut rumba_tx = pins.d2.into_output(&pins.ddr);
    rumba_tx.set_high().unwrap();
    let rumba_serial = bitbang_hal::serial::Serial::new(rumba_tx, pins.d3, timer);
    let rumba = Rumba::new(rumba_serial);

    ufmt::uwriteln!(&mut serial, "Starting Roomba").unwrap();
    let mut rumba = rumba.into_passive();
    delay_ms(1000);
    ufmt::uwriteln!(&mut serial, "Sending first song").unwrap();
    rumba
        .send_song(
            rumba::SongSlot::First,
            &[
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::F,
                    octave: NoteOctave::Small,
                    duration: 400.ms(),
                },
                Note {
                    name: NoteName::C,
                    octave: NoteOctave::OneLined,
                    duration: 200.ms(),
                },
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::F,
                    octave: NoteOctave::Small,
                    duration: 400.ms(),
                },
                Note {
                    name: NoteName::C,
                    octave: NoteOctave::OneLined,
                    duration: 200.ms(),
                },
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 1200.ms(),
                },
            ],
        )
        .unwrap();

    rumba
        .send_song(
            rumba::SongSlot::Second,
            &[
                Note {
                    name: NoteName::E,
                    octave: NoteOctave::OneLined,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::E,
                    octave: NoteOctave::OneLined,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::E,
                    octave: NoteOctave::OneLined,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::F,
                    octave: NoteOctave::OneLined,
                    duration: 400.ms(),
                },
                Note {
                    name: NoteName::C,
                    octave: NoteOctave::OneLined,
                    duration: 200.ms(),
                },
                Note {
                    name: NoteName::GSharp,
                    octave: NoteOctave::Small,
                    duration: 600.ms(),
                },
                Note {
                    name: NoteName::F,
                    octave: NoteOctave::Small,
                    duration: 400.ms(),
                },
                Note {
                    name: NoteName::C,
                    octave: NoteOctave::OneLined,
                    duration: 200.ms(),
                },
                Note {
                    name: NoteName::A,
                    octave: NoteOctave::Small,
                    duration: 1200.ms(),
                },
            ],
        )
        .unwrap();
    delay_ms(1000);
    let mut rumba = rumba.into_safe();
    delay_ms(1000);
    ufmt::uwriteln!(&mut serial, "Playing first song").unwrap();
    rumba.play_song(rumba::SongSlot::First).unwrap();
    delay_ms(4800); // Sufficient delay for the song
    rumba.play_song(rumba::SongSlot::Second).unwrap();
    delay_ms(4800); // Sufficient delay for the song
    clean_a_bit(&mut rumba.into_passive()).unwrap();
    ufmt::uwriteln!(&mut serial, "Done!").unwrap();
}

#[arduino_uno::entry]
fn main() -> ! {
    app();
    panic!("Application done!");
}
