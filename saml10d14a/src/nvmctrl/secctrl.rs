#[doc = "Reader of register SECCTRL"]
pub type R = crate::R<u32, super::SECCTRL>;
#[doc = "Writer for register SECCTRL"]
pub type W = crate::W<u32, super::SECCTRL>;
#[doc = "Register SECCTRL `reset()`'s with value 0x30"]
impl crate::ResetValue for super::SECCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Reader of field `TAMPEEN`"]
pub type TAMPEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPEEN`"]
pub struct TAMPEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPEEN_W<'a> {
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
#[doc = "Reader of field `SILACC`"]
pub type SILACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SILACC`"]
pub struct SILACC_W<'a> {
    w: &'a mut W,
}
impl<'a> SILACC_W<'a> {
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
#[doc = "Reader of field `DSCEN`"]
pub type DSCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSCEN`"]
pub struct DSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DXN`"]
pub type DXN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXN`"]
pub struct DXN_W<'a> {
    w: &'a mut W,
}
impl<'a> DXN_W<'a> {
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
#[doc = "Reader of field `TEROW`"]
pub type TEROW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEROW`"]
pub struct TEROW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_A {
    #[doc = "Write Key"]
    KEY,
}
impl crate::ToBits<u8> for KEY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEY_A::KEY => 165,
        }
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(KEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEY_A::KEY
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    pub fn tampeen(&self) -> TAMPEEN_R {
        TAMPEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&self) -> SILACC_R {
        SILACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&self) -> DSCEN_R {
        DSCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    pub fn dxn(&self) -> DXN_R {
        DXN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    pub fn terow(&self) -> TEROW_R {
        TEROW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    pub fn tampeen(&mut self) -> TAMPEEN_W {
        TAMPEEN_W { w: self }
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&mut self) -> SILACC_W {
        SILACC_W { w: self }
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&mut self) -> DSCEN_W {
        DSCEN_W { w: self }
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    pub fn dxn(&mut self) -> DXN_W {
        DXN_W { w: self }
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    pub fn terow(&mut self) -> TEROW_W {
        TEROW_W { w: self }
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
