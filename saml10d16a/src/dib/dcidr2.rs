#[doc = "Reader of register DCIDR2"]
pub type R = crate::R<u32, super::DCIDR2>;
#[doc = "Reader of field `PRMBL_2`"]
pub type PRMBL_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
