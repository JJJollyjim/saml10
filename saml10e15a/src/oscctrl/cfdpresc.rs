#[doc = "Reader of register CFDPRESC"]
pub type R = crate::R<u8, super::CFDPRESC>;
#[doc = "Writer for register CFDPRESC"]
pub type W = crate::W<u8, super::CFDPRESC>;
#[doc = "Register CFDPRESC `reset()`'s with value 0"]
impl crate::ResetValue for super::CFDPRESC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFDPRESC`"]
pub type CFDPRESC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFDPRESC`"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
}
