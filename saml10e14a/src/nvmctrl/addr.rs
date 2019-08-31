#[doc = "Reader of register ADDR"]
pub type R = crate::R<u32, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u32, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AOFFSET`"]
pub type AOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AOFFSET`"]
pub struct AOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> AOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Possible values of the field `ARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRAY_A {
    #[doc = "FLASH Array"]
    FLASH,
    #[doc = "DATA FLASH Array"]
    DATAFLASH,
    #[doc = "Auxilliary Space"]
    AUX,
}
impl crate::ToBits<u8> for ARRAY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ARRAY_A::FLASH => 0,
            ARRAY_A::DATAFLASH => 1,
            ARRAY_A::AUX => 2,
        }
    }
}
#[doc = "Reader of field `ARRAY`"]
pub type ARRAY_R = crate::R<u8, ARRAY_A>;
impl ARRAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ARRAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ARRAY_A::FLASH),
            1 => Val(ARRAY_A::DATAFLASH),
            2 => Val(ARRAY_A::AUX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == ARRAY_A::FLASH
    }
    #[doc = "Checks if the value of the field is `DATAFLASH`"]
    #[inline(always)]
    pub fn is_dataflash(&self) -> bool {
        *self == ARRAY_A::DATAFLASH
    }
    #[doc = "Checks if the value of the field is `AUX`"]
    #[inline(always)]
    pub fn is_aux(&self) -> bool {
        *self == ARRAY_A::AUX
    }
}
#[doc = "Write proxy for field `ARRAY`"]
pub struct ARRAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARRAY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FLASH Array"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(ARRAY_A::FLASH)
    }
    #[doc = "DATA FLASH Array"]
    #[inline(always)]
    pub fn dataflash(self) -> &'a mut W {
        self.variant(ARRAY_A::DATAFLASH)
    }
    #[doc = "Auxilliary Space"]
    #[inline(always)]
    pub fn aux(self) -> &'a mut W {
        self.variant(ARRAY_A::AUX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline(always)]
    pub fn aoffset(&self) -> AOFFSET_R {
        AOFFSET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline(always)]
    pub fn array(&self) -> ARRAY_R {
        ARRAY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline(always)]
    pub fn aoffset(&mut self) -> AOFFSET_W {
        AOFFSET_W { w: self }
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline(always)]
    pub fn array(&mut self) -> ARRAY_W {
        ARRAY_W { w: self }
    }
}
