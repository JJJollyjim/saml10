#[doc = "Reader of register DCIDR0"]
pub type R = crate::R<u32, super::DCIDR0>;
#[doc = "Reader of field `PRMBL_0`"]
pub type PRMBL_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
