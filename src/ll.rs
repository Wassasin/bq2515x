use bitvec::array::BitArray;
use device_driver::{AddressableDevice, AsyncRegisterDevice};
use embedded_hal_async::i2c::I2c;

const MAX_TRANSACTION_SIZE: usize = 5;
const ADDRESS: u8 = 0x6B;

pub struct Bq2515xDevice<I2C> {
    i2c: I2C,
}

impl<I2C> AddressableDevice for Bq2515xDevice<I2C> {
    type AddressType = u8;
}

impl<I2C> AsyncRegisterDevice for Bq2515xDevice<I2C>
where
    I2C: I2c,
{
    type Error = I2C::Error;

    async fn write_register<const SIZE_BYTES: usize>(
        &mut self,
        address: Self::AddressType,
        data: &BitArray<[u8; SIZE_BYTES]>,
    ) -> Result<(), Self::Error> {
        let data = data.as_raw_slice();

        let mut buf = [0u8; MAX_TRANSACTION_SIZE];
        buf[0] = address;
        buf[1..data.len() + 1].copy_from_slice(data);
        let buf = &buf[0..data.len() + 1];

        self.i2c.write(ADDRESS, buf).await
    }

    async fn read_register<const SIZE_BYTES: usize>(
        &mut self,
        address: Self::AddressType,
        data: &mut BitArray<[u8; SIZE_BYTES]>,
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(ADDRESS, &[address], data.as_raw_mut_slice())
            .await
    }
}

impl<I2C> Bq2515xDevice<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

pub mod registers {
    use super::*;
    use crate::prelude::*;

    #[device_driver_macros::implement_device_from_file(yaml = "src/ll.yaml")]
    impl<I2C> Bq2515xDevice<I2C> {}
}
