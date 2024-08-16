#[derive(Debug, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum DeviceID {
    BQ25150 = 0x20,
    BQ25155 = 0x35,
    BQ25157 = 0x3C,
}
