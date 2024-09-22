use zephyr::error::*;

pub struct Fd(i32);

impl TryFrom<i32> for Fd {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Fd(value.maybe_i32()?))
    }
}

impl Fd {
   pub fn try_from_raw(raw: i32) -> Result<Self, Error> {
        Self::try_from(raw)
   }

   pub fn as_raw(&self) -> i32 {
        self.0
   }
}