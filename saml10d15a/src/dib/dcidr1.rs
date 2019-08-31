#[doc = "Reader of register DCIDR1"]
pub type R = crate::R<u32, super::DCIDR1>;
#[doc = "Reader of field `PRMBL_1`"]
pub type PRMBL_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLASS`"]
pub type CLASS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CoreSight component class"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
