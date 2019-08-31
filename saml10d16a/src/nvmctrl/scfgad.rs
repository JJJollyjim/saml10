#[doc = "Reader of register SCFGAD"]
pub type R = crate::R<u32, super::SCFGAD>;
#[doc = "Writer for register SCFGAD"]
pub type W = crate::W<u32, super::SCFGAD>;
#[doc = "Register SCFGAD `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SCFGAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `URWEN`"]
pub type URWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URWEN`"]
pub struct URWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URWEN_W<'a> {
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
    #[doc = "Bit 0 - User Row Write Enable"]
    #[inline(always)]
    pub fn urwen(&self) -> URWEN_R {
        URWEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User Row Write Enable"]
    #[inline(always)]
    pub fn urwen(&mut self) -> URWEN_W {
        URWEN_W { w: self }
    }
}
