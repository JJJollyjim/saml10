#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u8, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u8, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRLC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MANW`"]
pub type MANW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MANW`"]
pub struct MANW_W<'a> {
    w: &'a mut W,
}
impl<'a> MANW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Manual Write"]
    #[inline(always)]
    pub fn manw(&self) -> MANW_R {
        MANW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Manual Write"]
    #[inline(always)]
    pub fn manw(&mut self) -> MANW_W {
        MANW_W { w: self }
    }
}
