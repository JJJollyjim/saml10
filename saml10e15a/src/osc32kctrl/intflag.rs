#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u32, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u32, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC32KRDY`"]
pub struct XOSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32KRDY_W<'a> {
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
#[doc = "Reader of field `CLKFAIL`"]
pub type CLKFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKFAIL`"]
pub struct CLKFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn clkfail(&self) -> CLKFAIL_R {
        CLKFAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W {
        XOSC32KRDY_W { w: self }
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn clkfail(&mut self) -> CLKFAIL_W {
        CLKFAIL_W { w: self }
    }
}
