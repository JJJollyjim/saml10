#[doc = "Reader of register RESCTRL"]
pub type R = crate::R<u8, super::RESCTRL>;
#[doc = "Writer for register RESCTRL"]
pub type W = crate::W<u8, super::RESCTRL>;
#[doc = "Register RESCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RESCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RES2OUT`"]
pub type RES2OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES2OUT`"]
pub struct RES2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RES2OUT_W<'a> {
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
#[doc = "Reader of field `RES1EN`"]
pub type RES1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES1EN`"]
pub struct RES1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RES1EN_W<'a> {
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
#[doc = "Reader of field `RES1MUX`"]
pub type RES1MUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES1MUX`"]
pub struct RES1MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> RES1MUX_W<'a> {
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
#[doc = "Reader of field `POTMUX`"]
pub type POTMUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POTMUX`"]
pub struct POTMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> POTMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `REFBUFLEVEL`"]
pub type REFBUFLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFBUFLEVEL`"]
pub struct REFBUFLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBUFLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&self) -> RES2OUT_R {
        RES2OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&self) -> RES1EN_R {
        RES1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&self) -> RES1MUX_R {
        RES1MUX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&self) -> POTMUX_R {
        POTMUX_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    pub fn refbuflevel(&self) -> REFBUFLEVEL_R {
        REFBUFLEVEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&mut self) -> RES2OUT_W {
        RES2OUT_W { w: self }
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&mut self) -> RES1EN_W {
        RES1EN_W { w: self }
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&mut self) -> RES1MUX_W {
        RES1MUX_W { w: self }
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&mut self) -> POTMUX_W {
        POTMUX_W { w: self }
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    pub fn refbuflevel(&mut self) -> REFBUFLEVEL_W {
        REFBUFLEVEL_W { w: self }
    }
}
