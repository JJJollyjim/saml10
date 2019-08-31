#[doc = "Reader of register DEBOUNCEN"]
pub type R = crate::R<u32, super::DEBOUNCEN>;
#[doc = "Writer for register DEBOUNCEN"]
pub type W = crate::W<u32, super::DEBOUNCEN>;
#[doc = "Register DEBOUNCEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBOUNCEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEBOUNCEN`"]
pub type DEBOUNCEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBOUNCEN`"]
pub struct DEBOUNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&self) -> DEBOUNCEN_R {
        DEBOUNCEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Debouncer Enable"]
    #[inline(always)]
    pub fn debouncen(&mut self) -> DEBOUNCEN_W {
        DEBOUNCEN_W { w: self }
    }
}
