#[doc = "Reader of register NSULCK"]
pub type R = crate::R<u16, super::NSULCK>;
#[doc = "Writer for register NSULCK"]
pub type W = crate::W<u16, super::NSULCK>;
#[doc = "Register NSULCK `reset()`'s with value 0"]
impl crate::ResetValue for super::NSULCK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BNS`"]
pub type BNS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BNS`"]
pub struct BNS_W<'a> {
    w: &'a mut W,
}
impl<'a> BNS_W<'a> {
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
#[doc = "Reader of field `ANS`"]
pub type ANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANS`"]
pub struct ANS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANS_W<'a> {
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
#[doc = "Reader of field `DNS`"]
pub type DNS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNS`"]
pub struct DNS_W<'a> {
    w: &'a mut W,
}
impl<'a> DNS_W<'a> {
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
#[doc = "Possible values of the field `NSLKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSLKEY_A {
    #[doc = "Write Key"]
    KEY,
}
impl crate::ToBits<u8> for NSLKEY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NSLKEY_A::KEY => 165,
        }
    }
}
#[doc = "Reader of field `NSLKEY`"]
pub type NSLKEY_R = crate::R<u8, NSLKEY_A>;
impl NSLKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NSLKEY_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(NSLKEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == NSLKEY_A::KEY
    }
}
#[doc = "Write proxy for field `NSLKEY`"]
pub struct NSLKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> NSLKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSLKEY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(NSLKEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    pub fn bns(&self) -> BNS_R {
        BNS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    pub fn ans(&self) -> ANS_R {
        ANS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    pub fn dns(&self) -> DNS_R {
        DNS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn nslkey(&self) -> NSLKEY_R {
        NSLKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    pub fn bns(&mut self) -> BNS_W {
        BNS_W { w: self }
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    pub fn ans(&mut self) -> ANS_W {
        ANS_W { w: self }
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    pub fn dns(&mut self) -> DNS_W {
        DNS_W { w: self }
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn nslkey(&mut self) -> NSLKEY_W {
        NSLKEY_W { w: self }
    }
}
