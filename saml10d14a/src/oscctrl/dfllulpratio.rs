#[doc = "Reader of register DFLLULPRATIO"]
pub type R = crate::R<u32, super::DFLLULPRATIO>;
#[doc = "Writer for register DFLLULPRATIO"]
pub type W = crate::W<u32, super::DFLLULPRATIO>;
#[doc = "Register DFLLULPRATIO `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLULPRATIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Target Tuner Ratio"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Target Tuner Ratio"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
