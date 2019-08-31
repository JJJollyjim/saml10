#[doc = "Reader of register VREG"]
pub type R = crate::R<u32, super::VREG>;
#[doc = "Writer for register VREG"]
pub type W = crate::W<u32, super::VREG>;
#[doc = "Register VREG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::VREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "LDO selection"]
    LDO,
    #[doc = "Buck selection"]
    BUCK,
}
impl crate::ToBits<u8> for SEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SEL_A::LDO => 0,
            SEL_A::BUCK => 1,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::LDO),
            1 => Val(SEL_A::BUCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == SEL_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == SEL_A::BUCK
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(SEL_A::LDO)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(SEL_A::BUCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `STDBYPL0`"]
pub type STDBYPL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STDBYPL0`"]
pub struct STDBYPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STDBYPL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPEFF`"]
pub type LPEFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPEFF`"]
pub struct LPEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEFF_W<'a> {
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
#[doc = "Reader of field `VREFSEL`"]
pub type VREFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFSEL`"]
pub struct VREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFSEL_W<'a> {
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
#[doc = "Reader of field `VSVSTEP`"]
pub type VSVSTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSVSTEP`"]
pub struct VSVSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> VSVSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VSPER`"]
pub type VSPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSPER`"]
pub struct VSPER_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Voltage Regulator Selection in active mode"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Standby in PL0"]
    #[inline(always)]
    pub fn stdbypl0(&self) -> STDBYPL0_R {
        STDBYPL0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low Power efficiency"]
    #[inline(always)]
    pub fn lpeff(&self) -> LPEFF_R {
        LPEFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Voltage Regulator Voltage Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Scaling Voltage Step"]
    #[inline(always)]
    pub fn vsvstep(&self) -> VSVSTEP_R {
        VSVSTEP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VSPER_R {
        VSPER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Voltage Regulator Selection in active mode"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 5 - Standby in PL0"]
    #[inline(always)]
    pub fn stdbypl0(&mut self) -> STDBYPL0_W {
        STDBYPL0_W { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 8 - Low Power efficiency"]
    #[inline(always)]
    pub fn lpeff(&mut self) -> LPEFF_W {
        LPEFF_W { w: self }
    }
    #[doc = "Bit 9 - Voltage Regulator Voltage Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&mut self) -> VREFSEL_W {
        VREFSEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Voltage Scaling Voltage Step"]
    #[inline(always)]
    pub fn vsvstep(&mut self) -> VSVSTEP_W {
        VSVSTEP_W { w: self }
    }
    #[doc = "Bits 24:31 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&mut self) -> VSPER_W {
        VSPER_W { w: self }
    }
}
