#[doc = "Reader of register DFLLULPCTRL"]
pub type R = crate::R<u16, super::DFLLULPCTRL>;
#[doc = "Writer for register DFLLULPCTRL"]
pub type W = crate::W<u16, super::DFLLULPCTRL>;
#[doc = "Register DFLLULPCTRL `reset()`'s with value 0x0504"]
impl crate::ResetValue for super::DFLLULPCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0504
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BINSE`"]
pub type BINSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BINSE`"]
pub struct BINSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BINSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SAFE`"]
pub type SAFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAFE`"]
pub struct SAFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAFE_W<'a> {
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
#[doc = "Reader of field `DITHER`"]
pub type DITHER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHER`"]
pub struct DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ONDEMAND`"]
pub type ONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONDEMAND`"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_A {
    #[doc = "Frequency Divided by 1"]
    DIV1,
    #[doc = "Frequency Divided by 2"]
    DIV2,
    #[doc = "Frequency Divided by 4"]
    DIV4,
    #[doc = "Frequency Divided by 8"]
    DIV8,
    #[doc = "Frequency Divided by 16"]
    DIV16,
    #[doc = "Frequency Divided by 32"]
    DIV32,
}
impl crate::ToBits<u8> for DIV_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DIV_A::DIV1 => 0,
            DIV_A::DIV2 => 1,
            DIV_A::DIV4 => 2,
            DIV_A::DIV8 => 3,
            DIV_A::DIV16 => 4,
            DIV_A::DIV32 => 5,
        }
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, DIV_A>;
impl DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIV_A::DIV1),
            1 => Val(DIV_A::DIV2),
            2 => Val(DIV_A::DIV4),
            3 => Val(DIV_A::DIV8),
            4 => Val(DIV_A::DIV16),
            5 => Val(DIV_A::DIV32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIV_A::DIV32
    }
}
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Frequency Divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIV_A::DIV1)
    }
    #[doc = "Frequency Divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIV_A::DIV2)
    }
    #[doc = "Frequency Divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIV_A::DIV4)
    }
    #[doc = "Frequency Divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIV_A::DIV8)
    }
    #[doc = "Frequency Divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIV_A::DIV16)
    }
    #[doc = "Frequency Divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIV_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Binary Search Enable"]
    #[inline(always)]
    pub fn binse(&self) -> BINSE_R {
        BINSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tuner Safe Mode"]
    #[inline(always)]
    pub fn safe(&self) -> SAFE_R {
        SAFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Tuner Dither Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Binary Search Enable"]
    #[inline(always)]
    pub fn binse(&mut self) -> BINSE_W {
        BINSE_W { w: self }
    }
    #[doc = "Bit 4 - Tuner Safe Mode"]
    #[inline(always)]
    pub fn safe(&mut self) -> SAFE_W {
        SAFE_W { w: self }
    }
    #[doc = "Bit 5 - Tuner Dither Mode"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:10 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
