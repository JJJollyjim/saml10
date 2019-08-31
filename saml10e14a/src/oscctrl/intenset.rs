#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCRDY`"]
pub struct XOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY_W<'a> {
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
#[doc = "Reader of field `XOSCFAIL`"]
pub type XOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCFAIL`"]
pub struct XOSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OSC16MRDY`"]
pub type OSC16MRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC16MRDY`"]
pub struct OSC16MRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC16MRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DFLLULPRDY`"]
pub type DFLLULPRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLULPRDY`"]
pub struct DFLLULPRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLULPRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DFLLULPLOCK`"]
pub type DFLLULPLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLULPLOCK`"]
pub struct DFLLULPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLULPLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DFLLULPNOLOCK`"]
pub type DFLLULPNOLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLULPNOLOCK`"]
pub struct DFLLULPNOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLULPNOLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKR`"]
pub struct DPLLLCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKF`"]
pub struct DPLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DPLLLTO`"]
pub type DPLLLTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLTO`"]
pub struct DPLLLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DPLLLDRTO`"]
pub type DPLLLDRTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLDRTO`"]
pub struct DPLLLDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLDRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC16M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc16mrdy(&self) -> OSC16MRDY_R {
        OSC16MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLLULP Ready interrupt Enable"]
    #[inline(always)]
    pub fn dfllulprdy(&self) -> DFLLULPRDY_R {
        DFLLULPRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DFLLULP Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dfllulplock(&self) -> DFLLULPLOCK_R {
        DFLLULPLOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DFLLULP No Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dfllulpnolock(&self) -> DFLLULPNOLOCK_R {
        DFLLULPNOLOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPLL Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W {
        XOSCRDY_W { w: self }
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XOSCFAIL_W {
        XOSCFAIL_W { w: self }
    }
    #[doc = "Bit 4 - OSC16M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc16mrdy(&mut self) -> OSC16MRDY_W {
        OSC16MRDY_W { w: self }
    }
    #[doc = "Bit 8 - DFLLULP Ready interrupt Enable"]
    #[inline(always)]
    pub fn dfllulprdy(&mut self) -> DFLLULPRDY_W {
        DFLLULPRDY_W { w: self }
    }
    #[doc = "Bit 9 - DFLLULP Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dfllulplock(&mut self) -> DFLLULPLOCK_W {
        DFLLULPLOCK_W { w: self }
    }
    #[doc = "Bit 10 - DFLLULP No Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dfllulpnolock(&mut self) -> DFLLULPNOLOCK_W {
        DFLLULPNOLOCK_W { w: self }
    }
    #[doc = "Bit 16 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W {
        DPLLLCKR_W { w: self }
    }
    #[doc = "Bit 17 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W {
        DPLLLCKF_W { w: self }
    }
    #[doc = "Bit 18 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&mut self) -> DPLLLTO_W {
        DPLLLTO_W { w: self }
    }
    #[doc = "Bit 19 - DPLL Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpllldrto(&mut self) -> DPLLLDRTO_W {
        DPLLLDRTO_W { w: self }
    }
}
