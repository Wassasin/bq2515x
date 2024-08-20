mod lowpower;

pub use lowpower::*;

use embedded_hal_async::i2c::I2c;

use crate::ll::Bq2515xDevice;
use crate::prelude::*;

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

    pub fn ll(&mut self) -> &mut Bq2515xDevice<I2C> {
        &mut self.dev
    }

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

    pub async fn adc_set_mode(&mut self, mode: AdcReadRate) -> Result<(), I2C::Error> {
        self.dev
            .adcctrl()
            .modify_async(|w| w.adc_read_rate(mode))
            .await
    }

    pub async fn adc_start_one_shot(&mut self) -> Result<(), I2C::Error> {
        self.dev
            .adcctrl()
            .modify_async(|w| {
                w.adc_read_rate(AdcReadRate::ManualRead)
                    .adc_conv_start(true)
            })
            .await
    }

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
