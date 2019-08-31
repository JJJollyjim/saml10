#[doc = "Reader of register SLEEPCFG"]
pub type R = crate::R<u8, super::SLEEPCFG>;
#[doc = "Writer for register SLEEPCFG"]
pub type W = crate::W<u8, super::SLEEPCFG>;
#[doc = "Register SLEEPCFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SLEEPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Possible values of the field `SLEEPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPMODE_A {
    #[doc = "CPU, AHB, APB clocks are OFF"]
    IDLE,
    #[doc = "All Clocks are OFF"]
    STANDBY,
    #[doc = "All power domains are powered OFF"]
    OFF,
}
impl crate::ToBits<u8> for SLEEPMODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLEEPMODE_A::IDLE => 2,
            SLEEPMODE_A::STANDBY => 4,
            SLEEPMODE_A::OFF => 6,
        }
    }
}
#[doc = "Reader of field `SLEEPMODE`"]
pub type SLEEPMODE_R = crate::R<u8, SLEEPMODE_A>;
impl SLEEPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLEEPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SLEEPMODE_A::IDLE),
            4 => Val(SLEEPMODE_A::STANDBY),
            6 => Val(SLEEPMODE_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPMODE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODE_A::OFF
    }
}
#[doc = "Write proxy for field `SLEEPMODE`"]
pub struct SLEEPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPMODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CPU, AHB, APB clocks are OFF"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE)
    }
    #[doc = "All Clocks are OFF"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::STANDBY)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&self) -> SLEEPMODE_R {
        SLEEPMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&mut self) -> SLEEPMODE_W {
        SLEEPMODE_W { w: self }
    }
}
