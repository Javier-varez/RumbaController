pub struct Hertz(u32);

pub trait U32Ext {
    fn mhz(self) -> Hertz;
    fn khz(self) -> Hertz;
    fn hz(self) -> Hertz;
}

impl U32Ext for u32 {
    fn mhz(self) -> Hertz {
        Hertz(self * 1_000_000)
    }

    fn khz(self) -> Hertz {
        Hertz(self * 1_000)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }
}

pub struct AvrTimer1 {
    tc1: arduino_uno::pac::TC1,
}

impl AvrTimer1 {
    pub fn new(tc1: arduino_uno::pac::TC1) -> Self {
        Self { tc1 }
    }
}

impl embedded_hal::timer::CountDown for AvrTimer1 {
    type Time = Hertz;
    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        let timer = &mut self.tc1;

        // Disable timer
        timer.tccr1b.reset();

        let count: u16 = (16_000_000 / count.into().0) as u16;
        unsafe {
            timer.tcnt1.write(|w| w.bits(0));
        }
        unsafe { timer.ocr1a.write(|w| w.bits(count - 1)) };

        // Set CTC (mode 4)
        // No prescaler
        timer.tccr1a.reset();
        timer.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().direct());
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        let timer = &mut self.tc1;

        if timer.tifr1.read().ocf1a().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            // Clear the overflow flag
            timer.tifr1.write(|w| w.ocf1a().set_bit());
            Ok(())
        }
    }
}

impl embedded_hal::timer::Periodic for AvrTimer1 {}
