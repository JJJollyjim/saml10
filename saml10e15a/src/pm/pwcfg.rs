#[doc = "Reader of register PWCFG"]
pub type R = crate::R<u8, super::PWCFG>;
#[doc = "Writer for register PWCFG"]
pub type W = crate::W<u8, super::PWCFG>;
#[doc = "Register PWCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PWCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RAMPSWC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPSWC_A {
    #[doc = "16KB Available"]
    _16KB,
    #[doc = "12KB Available"]
    _12KB,
    #[doc = "8KB Available"]
    _8KB,
    #[doc = "4KB Available"]
    _4KB,
}
impl crate::ToBits<u8> for RAMPSWC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RAMPSWC_A::_16KB => 0,
            RAMPSWC_A::_12KB => 1,
            RAMPSWC_A::_8KB => 2,
            RAMPSWC_A::_4KB => 3,
        }
    }
}
#[doc = "Reader of field `RAMPSWC`"]
pub type RAMPSWC_R = crate::R<u8, RAMPSWC_A>;
impl RAMPSWC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMPSWC_A {
        match self.bits {
            0 => RAMPSWC_A::_16KB,
            1 => RAMPSWC_A::_12KB,
            2 => RAMPSWC_A::_8KB,
            3 => RAMPSWC_A::_4KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == RAMPSWC_A::_16KB
    }
    #[doc = "Checks if the value of the field is `_12KB`"]
    #[inline(always)]
    pub fn is_12kb(&self) -> bool {
        *self == RAMPSWC_A::_12KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == RAMPSWC_A::_8KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == RAMPSWC_A::_4KB
    }
}
#[doc = "Write proxy for field `RAMPSWC`"]
pub struct RAMPSWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPSWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPSWC_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "16KB Available"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut W {
        self.variant(RAMPSWC_A::_16KB)
    }
    #[doc = "12KB Available"]
    #[inline(always)]
    pub fn _12kb(self) -> &'a mut W {
        self.variant(RAMPSWC_A::_12KB)
    }
    #[doc = "8KB Available"]
    #[inline(always)]
    pub fn _8kb(self) -> &'a mut W {
        self.variant(RAMPSWC_A::_8KB)
    }
    #[doc = "4KB Available"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(RAMPSWC_A::_4KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline(always)]
    pub fn rampswc(&self) -> RAMPSWC_R {
        RAMPSWC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline(always)]
    pub fn rampswc(&mut self) -> RAMPSWC_W {
        RAMPSWC_W { w: self }
    }
}
