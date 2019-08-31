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
#[doc = "Reader of field `CFDEO`"]
pub type CFDEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEO`"]
pub struct CFDEO_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEO_W<'a> {
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
#[doc = "Reader of field `TUNEEI`"]
pub type TUNEEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNEEI`"]
pub struct TUNEEI_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNEEI_W<'a> {
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
#[doc = "Reader of field `TUNEINV`"]
pub type TUNEINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNEINV`"]
pub struct TUNEINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNEINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo(&self) -> CFDEO_R {
        CFDEO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tune Event Input Enable"]
    #[inline(always)]
    pub fn tuneei(&self) -> TUNEEI_R {
        TUNEEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tune Event Input Invert"]
    #[inline(always)]
    pub fn tuneinv(&self) -> TUNEINV_R {
        TUNEINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo(&mut self) -> CFDEO_W {
        CFDEO_W { w: self }
    }
    #[doc = "Bit 1 - Tune Event Input Enable"]
    #[inline(always)]
    pub fn tuneei(&mut self) -> TUNEEI_W {
        TUNEEI_W { w: self }
    }
    #[doc = "Bit 2 - Tune Event Input Invert"]
    #[inline(always)]
    pub fn tuneinv(&mut self) -> TUNEINV_W {
        TUNEINV_W { w: self }
    }
}
