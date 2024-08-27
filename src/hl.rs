//! High level interface for the Bq2515x family of chips providing convenience methods and a Rust-style interface.

mod lowpower;

pub use lowpower::*;

use embedded_hal_async::i2c::I2c;

use crate::ll::Bq2515xDevice;
use crate::prelude::*;

/// High level interface for the Bq2515x family of chips.
///
/// Assumes that the device is not in Low Power mode during interactions. This can be managed by either:
/// * Permanently tying the *not-low-power* (/LP) pin to low.
/// * Manually pulling the *not-low-power* (/LP) pin to low and awaiting the prequisite time when planning to use the device.
/// * Use the [Bq2515xLowPower] interface to manage the *not-low-power* (/LP) pin for you.
pub struct Bq2515x<I2C> {
    dev: Bq2515xDevice<I2C>,
}

pub enum LdoConfig {
    Off,
    Switch,
    Ldo(LDOOutputVoltage),
}

impl<I2C> Bq2515x<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            dev: Bq2515xDevice::new(i2c),
        }
    }

    /// Get access to the underlying low level device.
    pub fn ll(&mut self) -> &mut Bq2515xDevice<I2C> {
        &mut self.dev
    }

    pub fn take(self) -> I2C {
        self.dev.take()
    }

    /// Configure the LDO pin.
    pub async fn ldo(&mut self, config: LdoConfig) -> Result<(), I2C::Error> {
        self.dev
            .ldoctrl()
            .write_async(|w| match config {
                LdoConfig::Off => w.en_ls_ldo(false),
                LdoConfig::Switch => w.en_ls_ldo(true).ldo_switch_config(LdoSwitchConfig::Switch),
                LdoConfig::Ldo(v) => w
                    .en_ls_ldo(true)
                    .ldo_switch_config(LdoSwitchConfig::Ldo)
                    .vldo(v),
            })
            .await
    }

    /// Set the mode by which the ADC samples.
    pub async fn adc_set_mode(&mut self, mode: AdcReadRate) -> Result<(), I2C::Error> {
        self.dev
            .adcctrl()
            .modify_async(|w| w.adc_read_rate(mode))
            .await
    }

    /// Start an one-shot ADC acquisition.
    ///
    /// Sets the ADC mode to 'manual'.
    ///
    /// In order to check whether the one shot has completed, you can check the `flags` register.
    /// Note that when the device is powered by VIN (i.e. Power is Good) the `adc_ready` flag is never set.
    pub async fn adc_start_one_shot(&mut self) -> Result<(), I2C::Error> {
        self.dev
            .adcctrl()
            .modify_async(|w| {
                w.adc_read_rate(AdcReadRate::ManualRead)
                    .adc_conv_start(true)
            })
            .await
    }

    /// Fetch the latest ADC acquisition.
    pub async fn adc_fetch_latest(&mut self) -> Result<AdcData, I2C::Error> {
        let channels = self.dev.adc_read_en().read_async().await?;
        let ilim = self.dev.ilimctrl().read_async().await?.ilim().unwrap();
        let data = self.dev.adc_data().read_async().await?;

        Ok(AdcData {
            vin: channels.vin().then(|| RawVoltage(data.vin())),
            pmid: channels.pmid().then(|| RawVoltage(data.pmid())),
            iin: channels.iin().then(|| IinCurrent {
                raw: data.iin(),
                high_range: ilim > CurrentLimit::_150mA,
            }),
            vbat: channels.vbat().then(|| RawVoltage(data.vbat())),
            ts: channels.ts().then(|| RawVoltage(data.ts())),
            adcin: channels.adcin().then(|| RawVoltage(data.adcin())),
            icharge: channels.ichg().then(|| IChargePercentage(data.ichg())),
        })
    }
}
