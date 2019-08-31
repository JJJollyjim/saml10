#[doc = "Reader of register STDBYCFG"]
pub type R = crate::R<u16, super::STDBYCFG>;
#[doc = "Writer for register STDBYCFG"]
pub type W = crate::W<u16, super::STDBYCFG>;
#[doc = "Register STDBYCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::STDBYCFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `PDCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDCFG_A {
    #[doc = "PDSW power domain switching is handled by hardware."]
    DEFAULT,
    #[doc = "PDSW is forced ACTIVE."]
    PDSW,
}
impl crate::ToBits<bool> for PDCFG_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PDCFG_A::DEFAULT => false,
            PDCFG_A::PDSW => true,
        }
    }
}
#[doc = "Reader of field `PDCFG`"]
pub type PDCFG_R = crate::R<bool, PDCFG_A>;
impl PDCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDCFG_A {
        match self.bits {
            false => PDCFG_A::DEFAULT,
            true => PDCFG_A::PDSW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PDCFG_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `PDSW`"]
    #[inline(always)]
    pub fn is_pdsw(&self) -> bool {
        *self == PDCFG_A::PDSW
    }
}
#[doc = "Write proxy for field `PDCFG`"]
pub struct PDCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDCFG_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDSW power domain switching is handled by hardware."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PDCFG_A::DEFAULT)
    }
    #[doc = "PDSW is forced ACTIVE."]
    #[inline(always)]
    pub fn pdsw(self) -> &'a mut W {
        self.variant(PDCFG_A::PDSW)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `DPGPDSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPGPDSW_A {
    #[doc = "Dynamic Power Gating disabled"]
    _0,
    #[doc = "Dynamic Power Gating enabled"]
    _1,
}
impl crate::ToBits<bool> for DPGPDSW_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DPGPDSW_A::_0 => false,
            DPGPDSW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPGPDSW`"]
pub type DPGPDSW_R = crate::R<bool, DPGPDSW_A>;
impl DPGPDSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPGPDSW_A {
        match self.bits {
            false => DPGPDSW_A::_0,
            true => DPGPDSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPGPDSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPGPDSW_A::_1
    }
}
#[doc = "Write proxy for field `DPGPDSW`"]
pub struct DPGPDSW_W<'a> {
    w: &'a mut W,
}
impl<'a> DPGPDSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPGPDSW_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dynamic Power Gating disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPGPDSW_A::_0)
    }
    #[doc = "Dynamic Power Gating enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPGPDSW_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `VREGSMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREGSMOD_A {
    #[doc = "Automatic mode"]
    AUTO,
    #[doc = "Performance oriented"]
    PERFORMANCE,
    #[doc = "Low Power oriented"]
    LP,
}
impl crate::ToBits<u8> for VREGSMOD_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            VREGSMOD_A::AUTO => 0,
            VREGSMOD_A::PERFORMANCE => 1,
            VREGSMOD_A::LP => 2,
        }
    }
}
#[doc = "Reader of field `VREGSMOD`"]
pub type VREGSMOD_R = crate::R<u8, VREGSMOD_A>;
impl VREGSMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VREGSMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VREGSMOD_A::AUTO),
            1 => Val(VREGSMOD_A::PERFORMANCE),
            2 => Val(VREGSMOD_A::LP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMOD_A::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMOD_A::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMOD_A::LP
    }
}
#[doc = "Write proxy for field `VREGSMOD`"]
pub struct VREGSMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGSMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREGSMOD_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMOD_A::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMOD_A::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMOD_A::LP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BBIASHS`"]
pub type BBIASHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBIASHS`"]
pub struct BBIASHS_W<'a> {
    w: &'a mut W,
}
impl<'a> BBIASHS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BBIASTR`"]
pub type BBIASTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBIASTR`"]
pub struct BBIASTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBIASTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline(always)]
    pub fn pdcfg(&self) -> PDCFG_R {
        PDCFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline(always)]
    pub fn dpgpdsw(&self) -> DPGPDSW_R {
        DPGPDSW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VREGSMOD_R {
        VREGSMOD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BBIASHS_R {
        BBIASHS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline(always)]
    pub fn bbiastr(&self) -> BBIASTR_R {
        BBIASTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline(always)]
    pub fn pdcfg(&mut self) -> PDCFG_W {
        PDCFG_W { w: self }
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline(always)]
    pub fn dpgpdsw(&mut self) -> DPGPDSW_W {
        DPGPDSW_W { w: self }
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&mut self) -> VREGSMOD_W {
        VREGSMOD_W { w: self }
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline(always)]
    pub fn bbiashs(&mut self) -> BBIASHS_W {
        BBIASHS_W { w: self }
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline(always)]
    pub fn bbiastr(&mut self) -> BBIASTR_W {
        BBIASTR_W { w: self }
    }
}
