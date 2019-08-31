#[doc = "Reader of register DDEVTYPE"]
pub type R = crate::R<u32, super::DDEVTYPE>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUB`"]
pub type SUB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Major type"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-type"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
