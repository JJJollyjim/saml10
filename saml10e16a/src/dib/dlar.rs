#[doc = "Writer for register DLAR"]
pub type W = crate::W<u32, super::DLAR>;
#[doc = "Register DLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_AW {
    #[doc = "Unlock key value"]
    UNLOCK,
}
impl crate::ToBits<u32> for KEY_AW {
    #[inline(always)]
    fn _bits(&self) -> u32 {
        match *self {
            KEY_AW::UNLOCK => 3316436565,
        }
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Unlock key value"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(KEY_AW::UNLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Lock access control"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
