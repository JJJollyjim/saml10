#[doc = "Reader of register PINSTATE"]
pub type R = crate::R<u32, super::PINSTATE>;
#[doc = "Reader of field `PINSTATE`"]
pub type PINSTATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Pin State"]
    #[inline(always)]
    pub fn pinstate(&self) -> PINSTATE_R {
        PINSTATE_R::new((self.bits & 0xff) as u8)
    }
}
