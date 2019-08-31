#[doc = "Reader of register VREGSUSP"]
pub type R = crate::R<u32, super::VREGSUSP>;
#[doc = "Writer for register VREGSUSP"]
pub type W = crate::W<u32, super::VREGSUSP>;
#[doc = "Register VREGSUSP `reset()`'s with value 0"]
impl crate::ResetValue for super::VREGSUSP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREGSEN`"]
pub type VREGSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGSEN`"]
pub struct VREGSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Voltage Regulator Suspend"]
    #[inline(always)]
    pub fn vregsen(&self) -> VREGSEN_R {
        VREGSEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Voltage Regulator Suspend"]
    #[inline(always)]
    pub fn vregsen(&mut self) -> VREGSEN_W {
        VREGSEN_W { w: self }
    }
}
