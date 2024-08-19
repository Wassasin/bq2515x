pub use crate::ll::registers::{AdcReadRate, IchargeRange, PmidMode, PmidRegCtrl};

#[derive(Debug, PartialEq, PartialOrd, num_enum::IntoPrimitive, num_enum::FromPrimitive)]
#[repr(u8)]
pub enum AdcCompChannel {
    #[num_enum(default)]
    Disabled,
    AdcIn,
    TS,
    VBat,
    ICharge,
    Vin,
    PMid,
    IIn,
}

#[derive(Debug, PartialEq, PartialOrd, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum CurrentLimit {
    _50mA = 0b000,
    _100mA = 0b001,
    _150mA = 0b010,
    _200mA = 0b011,
    _300mA = 0b100,
    _400mA = 0b101,
    _500mA = 0b110,
    _600mA = 0b111,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Millivolts(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Milliampere(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawVoltage<const MAX_MV: u16>(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IinCurrent {
    pub raw: u16,
    pub high_range: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IChargePercentage(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, derive_more::From, derive_more::Into)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VBatRegulationVoltage(pub u8);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, derive_more::From, derive_more::Into)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FastChargeCurrent(pub u8);

#[inline(always)]
fn scale_u16(x: u16, old_range: u16, new_range: u16) -> u16 {
    (x as u32 * new_range as u32 / old_range as u32) as u16
}

impl<const MAX_MV: u16> From<RawVoltage<MAX_MV>> for Millivolts {
    fn from(val: RawVoltage<MAX_MV>) -> Self {
        Millivolts(scale_u16(val.0, u16::MAX, MAX_MV))
    }
}

impl<const MAX_MV: u16> From<Millivolts> for RawVoltage<MAX_MV> {
    fn from(val: Millivolts) -> Self {
        RawVoltage(scale_u16(val.0, MAX_MV, u16::MAX))
    }
}

impl From<VBatRegulationVoltage> for Millivolts {
    fn from(value: VBatRegulationVoltage) -> Self {
        let value: u16 = 3600 + value.0 as u16 * 10;
        Millivolts(value.clamp(3600, 4600))
    }
}

impl From<Millivolts> for VBatRegulationVoltage {
    fn from(value: Millivolts) -> Self {
        let value = value.0.clamp(3600, 4600);
        let value = (value - 3600) / 10;
        VBatRegulationVoltage(value as u8)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Millivolts {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}mV", self.0)
    }
}

impl IinCurrent {
    pub fn range(&self) -> Milliampere {
        Milliampere(if self.high_range { 750 } else { 375 })
    }
}

impl From<IinCurrent> for Milliampere {
    fn from(val: IinCurrent) -> Self {
        Milliampere(scale_u16(val.raw, u16::MAX, val.range().0))
    }
}

impl From<Milliampere> for IinCurrent {
    fn from(val: Milliampere) -> Self {
        IinCurrent {
            raw: scale_u16(val.0, 750, u16::MAX),
            high_range: true,
        }
    }
}

impl FastChargeCurrent {
    pub fn from_milliampere(ma: Milliampere, range: IchargeRange) -> Self {
        let step_100 = match range {
            IchargeRange::Step1MilliA25 => 125,
            IchargeRange::Step2MilliA5 => 250,
        };
        Self((ma.0 * 100 / step_100) as u8)
    }
}

impl IChargePercentage {
    pub fn to_percentage(&self) -> u8 {
        (self.0 as u32 * 80 / u16::MAX as u32) as u8
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcData {
    pub vin: Option<RawVoltage<6000>>,
    pub pmid: Option<RawVoltage<6000>>,
    pub iin: Option<IinCurrent>,
    pub vbat: Option<RawVoltage<6000>>,
    pub ts: Option<RawVoltage<1200>>,
    pub adcin: Option<RawVoltage<1200>>,
    pub icharge: Option<IChargePercentage>,
}
