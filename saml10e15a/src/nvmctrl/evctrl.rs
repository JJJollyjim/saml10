#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u8, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u8, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTOWEI`"]
pub type AUTOWEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOWEI`"]
pub struct AUTOWEI_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOWEI_W<'a> {
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
#[doc = "Reader of field `AUTOWINV`"]
pub type AUTOWINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOWINV`"]
pub struct AUTOWINV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOWINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Write Event Enable"]
    #[inline(always)]
    pub fn autowei(&self) -> AUTOWEI_R {
        AUTOWEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto Write Event Polarity Inverted"]
    #[inline(always)]
    pub fn autowinv(&self) -> AUTOWINV_R {
        AUTOWINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Write Event Enable"]
    #[inline(always)]
    pub fn autowei(&mut self) -> AUTOWEI_W {
        AUTOWEI_W { w: self }
    }
    #[doc = "Bit 1 - Auto Write Event Polarity Inverted"]
    #[inline(always)]
    pub fn autowinv(&mut self) -> AUTOWINV_W {
        AUTOWINV_W { w: self }
    }
}
