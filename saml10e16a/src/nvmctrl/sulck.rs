#[doc = "Reader of register SULCK"]
pub type R = crate::R<u16, super::SULCK>;
#[doc = "Writer for register SULCK"]
pub type W = crate::W<u16, super::SULCK>;
#[doc = "Register SULCK `reset()`'s with value 0"]
impl crate::ResetValue for super::SULCK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BS`"]
pub type BS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BS`"]
pub struct BS_W<'a> {
    w: &'a mut W,
}
impl<'a> BS_W<'a> {
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
#[doc = "Reader of field `AS`"]
pub type AS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AS`"]
pub struct AS_W<'a> {
    w: &'a mut W,
}
impl<'a> AS_W<'a> {
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
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `SLKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLKEY_A {
    #[doc = "Write Key"]
    KEY,
}
impl crate::ToBits<u8> for SLKEY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLKEY_A::KEY => 165,
        }
    }
}
#[doc = "Reader of field `SLKEY`"]
pub type SLKEY_R = crate::R<u8, SLKEY_A>;
impl SLKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLKEY_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(SLKEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == SLKEY_A::KEY
    }
}
#[doc = "Write proxy for field `SLKEY`"]
pub struct SLKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLKEY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(SLKEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Secure Boot Region"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure Application Region"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Secure Region"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn slkey(&self) -> SLKEY_R {
        SLKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot Region"]
    #[inline(always)]
    pub fn bs(&mut self) -> BS_W {
        BS_W { w: self }
    }
    #[doc = "Bit 1 - Secure Application Region"]
    #[inline(always)]
    pub fn as_(&mut self) -> AS_W {
        AS_W { w: self }
    }
    #[doc = "Bit 2 - Data Secure Region"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn slkey(&mut self) -> SLKEY_W {
        SLKEY_W { w: self }
    }
}
