use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use super::Bq2515x;

/// Wrapper around [Bq2515x] to manage the *not-low-power* (/LP) pin.
///
/// Drop the [Bq2515xHighPower] object to release this higher power mode and return to low power.
pub struct Bq2515xLowPower<I2C, P, D> {
    inner: Bq2515x<I2C>,
    nlp_pin: P,
    delay: D,
}

/// Wrapper around [Bq2515x] to indicate that the device is no longer in Low Power mode. Drop to return to Low Power mode.
pub struct Bq2515xHighPower<'a, I2C, P>
where
    P: OutputPin<Error = Infallible>,
{
    inner: &'a mut Bq2515x<I2C>,
    nlp_pin: &'a mut P,
}

impl<I2C, P, D> Bq2515xLowPower<I2C, P, D>
where
    I2C: I2c,
    P: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    pub fn new(i2c: I2C, nlp_pin: P, delay: D) -> Self {
        Self {
            inner: Bq2515x::new(i2c),
            nlp_pin,
            delay,
        }
    }

    /// Activates the I2C interface by pulling the *not-low-power* (/LP) pin down and awaiting the prerequisite time.
    pub async fn activate(&mut self) -> Bq2515xHighPower<'_, I2C, P> {
        let _ = self.nlp_pin.set_high();
        self.delay.delay_ms(1).await; // tLP_EXIT_I2C
        Bq2515xHighPower {
            inner: &mut self.inner,
            nlp_pin: &mut self.nlp_pin,
        }
    }
}

impl<'a, I2C, P> Deref for Bq2515xHighPower<'a, I2C, P>
where
    P: OutputPin<Error = Infallible>,
{
    type Target = Bq2515x<I2C>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, I2C, P> DerefMut for Bq2515xHighPower<'a, I2C, P>
where
    P: OutputPin<Error = Infallible>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, I2C, P> Drop for Bq2515xHighPower<'a, I2C, P>
where
    P: OutputPin<Error = Infallible>,
{
    fn drop(&mut self) {
        let _ = self.nlp_pin.set_low();
    }
}
