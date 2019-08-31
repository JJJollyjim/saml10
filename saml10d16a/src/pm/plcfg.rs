#[doc = "Reader of register PLCFG"]
pub type R = crate::R<u8, super::PLCFG>;
#[doc = "Writer for register PLCFG"]
pub type W = crate::W<u8, super::PLCFG>;
#[doc = "Register PLCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PLCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `PLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLSEL_A {
    #[doc = "Performance Level 0"]
    PL0,
    #[doc = "Performance Level 2"]
    PL2,
}
impl crate::ToBits<u8> for PLSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PLSEL_A::PL0 => 0,
            PLSEL_A::PL2 => 2,
        }
    }
}
#[doc = "Reader of field `PLSEL`"]
pub type PLSEL_R = crate::R<u8, PLSEL_A>;
impl PLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLSEL_A::PL0),
            2 => Val(PLSEL_A::PL2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PL0`"]
    #[inline(always)]
    pub fn is_pl0(&self) -> bool {
        *self == PLSEL_A::PL0
    }
    #[doc = "Checks if the value of the field is `PL2`"]
    #[inline(always)]
    pub fn is_pl2(&self) -> bool {
        *self == PLSEL_A::PL2
    }
}
#[doc = "Write proxy for field `PLSEL`"]
pub struct PLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Performance Level 0"]
    #[inline(always)]
    pub fn pl0(self) -> &'a mut W {
        self.variant(PLSEL_A::PL0)
    }
    #[doc = "Performance Level 2"]
    #[inline(always)]
    pub fn pl2(self) -> &'a mut W {
        self.variant(PLSEL_A::PL2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLDIS`"]
pub type PLDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLDIS`"]
pub struct PLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline(always)]
    pub fn plsel(&self) -> PLSEL_R {
        PLSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline(always)]
    pub fn pldis(&self) -> PLDIS_R {
        PLDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline(always)]
    pub fn plsel(&mut self) -> PLSEL_W {
        PLSEL_W { w: self }
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline(always)]
    pub fn pldis(&mut self) -> PLDIS_W {
        PLDIS_W { w: self }
    }
}
