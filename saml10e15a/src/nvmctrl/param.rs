#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Writer for register PARAM"]
pub type W = crate::W<u32, super::PARAM>;
#[doc = "Register PARAM `reset()`'s with value 0"]
impl crate::ResetValue for super::PARAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASHP`"]
pub type FLASHP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLASHP`"]
pub struct FLASHP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Possible values of the field `PSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSZ_A {
    #[doc = "8 bytes"]
    _8,
    #[doc = "16 bytes"]
    _16,
    #[doc = "32 bytes"]
    _32,
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "256 bytes"]
    _256,
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
}
impl crate::ToBits<u8> for PSZ_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PSZ_A::_8 => 0,
            PSZ_A::_16 => 1,
            PSZ_A::_32 => 2,
            PSZ_A::_64 => 3,
            PSZ_A::_128 => 4,
            PSZ_A::_256 => 5,
            PSZ_A::_512 => 6,
            PSZ_A::_1024 => 7,
        }
    }
}
#[doc = "Reader of field `PSZ`"]
pub type PSZ_R = crate::R<u8, PSZ_A>;
impl PSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSZ_A {
        match self.bits {
            0 => PSZ_A::_8,
            1 => PSZ_A::_16,
            2 => PSZ_A::_32,
            3 => PSZ_A::_64,
            4 => PSZ_A::_128,
            5 => PSZ_A::_256,
            6 => PSZ_A::_512,
            7 => PSZ_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PSZ_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSZ_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSZ_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSZ_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSZ_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSZ_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSZ_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSZ_A::_1024
    }
}
#[doc = "Write proxy for field `PSZ`"]
pub struct PSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSZ_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSZ_A::_8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSZ_A::_16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSZ_A::_32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSZ_A::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSZ_A::_128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSZ_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSZ_A::_512)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSZ_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DFLASHP`"]
pub type DFLASHP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DFLASHP`"]
pub struct DFLASHP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLASHP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline(always)]
    pub fn flashp(&self) -> FLASHP_R {
        FLASHP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline(always)]
    pub fn dflashp(&self) -> DFLASHP_R {
        DFLASHP_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline(always)]
    pub fn flashp(&mut self) -> FLASHP_W {
        FLASHP_W { w: self }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&mut self) -> PSZ_W {
        PSZ_W { w: self }
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline(always)]
    pub fn dflashp(&mut self) -> DFLASHP_W {
        DFLASHP_W { w: self }
    }
}
