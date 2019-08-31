#[doc = "Reader of register DFLLULPDLY"]
pub type R = crate::R<u32, super::DFLLULPDLY>;
#[doc = "Writer for register DFLLULPDLY"]
pub type W = crate::W<u32, super::DFLLULPDLY>;
#[doc = "Register DFLLULPDLY `reset()`'s with value 0x80"]
impl crate::ResetValue for super::DFLLULPDLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Delay Value"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay Value"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
}
