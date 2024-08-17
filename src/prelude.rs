#[derive(Debug, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum DeviceID {
    BQ25150 = 0x20,
    BQ25155 = 0x35,
    BQ25157 = 0x3C,
}

#[derive(Debug, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum AdcMode {
    ManualRead = 0b00,
    Continuous = 0b01,
    EverySecond = 0b10,
    EveryMinute = 0b11,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Millivolts(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Milliampere(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct RawVoltage<const MAX_MV: u16>(pub u16);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct IinCurrent {
    pub raw: u16,
    pub high_range: bool,
}

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

pub struct IChargePercentage(pub u16);

impl IChargePercentage {
    pub fn to_percentage(&self) -> u8 {
        (self.0 as u32 * 80 / u16::MAX as u32) as u8
    }
}

pub struct AdcData {
    pub vin: Option<RawVoltage<6000>>,
    pub pmid: Option<RawVoltage<6000>>,
    pub iin: Option<IinCurrent>,
    pub vbat: Option<RawVoltage<6000>>,
    pub ts: Option<RawVoltage<1200>>,
    pub adcin: Option<RawVoltage<1200>>,
    pub icharge: Option<IChargePercentage>,
}
