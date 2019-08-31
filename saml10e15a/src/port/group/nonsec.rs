#[doc = "Reader of register NONSEC"]
pub type R = crate::R<u32, super::NONSEC>;
#[doc = "Writer for register NONSEC"]
pub type W = crate::W<u32, super::NONSEC>;
#[doc = "Register NONSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::NONSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NONSEC`"]
pub type NONSEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NONSEC`"]
pub struct NONSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NONSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    pub fn nonsec(&self) -> NONSEC_R {
        NONSEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    pub fn nonsec(&mut self) -> NONSEC_W {
        NONSEC_W { w: self }
    }
}
