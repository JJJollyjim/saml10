#[doc = "Reader of register DFLLULPDITHER"]
pub type R = crate::R<u8, super::DFLLULPDITHER>;
#[doc = "Writer for register DFLLULPDITHER"]
pub type W = crate::W<u8, super::DFLLULPDITHER>;
#[doc = "Register DFLLULPDITHER `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLULPDITHER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `STEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEP_A {
    #[doc = "Dither Step = 1"]
    STEP1,
    #[doc = "Dither Step = 2"]
    STEP2,
    #[doc = "Dither Step = 4"]
    STEP4,
    #[doc = "Dither Step = 8"]
    STEP8,
}
impl crate::ToBits<u8> for STEP_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            STEP_A::STEP1 => 0,
            STEP_A::STEP2 => 1,
            STEP_A::STEP4 => 2,
            STEP_A::STEP8 => 3,
        }
    }
}
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u8, STEP_A>;
impl STEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STEP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STEP_A::STEP1),
            1 => Val(STEP_A::STEP2),
            2 => Val(STEP_A::STEP4),
            3 => Val(STEP_A::STEP8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STEP1`"]
    #[inline(always)]
    pub fn is_step1(&self) -> bool {
        *self == STEP_A::STEP1
    }
    #[doc = "Checks if the value of the field is `STEP2`"]
    #[inline(always)]
    pub fn is_step2(&self) -> bool {
        *self == STEP_A::STEP2
    }
    #[doc = "Checks if the value of the field is `STEP4`"]
    #[inline(always)]
    pub fn is_step4(&self) -> bool {
        *self == STEP_A::STEP4
    }
    #[doc = "Checks if the value of the field is `STEP8`"]
    #[inline(always)]
    pub fn is_step8(&self) -> bool {
        *self == STEP_A::STEP8
    }
}
#[doc = "Write proxy for field `STEP`"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STEP_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dither Step = 1"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut W {
        self.variant(STEP_A::STEP1)
    }
    #[doc = "Dither Step = 2"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut W {
        self.variant(STEP_A::STEP2)
    }
    #[doc = "Dither Step = 4"]
    #[inline(always)]
    pub fn step4(self) -> &'a mut W {
        self.variant(STEP_A::STEP4)
    }
    #[doc = "Dither Step = 8"]
    #[inline(always)]
    pub fn step8(self) -> &'a mut W {
        self.variant(STEP_A::STEP8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `PER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    #[doc = "Dither Over 1 Reference Clock Period"]
    PER1,
    #[doc = "Dither Over 2 Reference Clock Period"]
    PER2,
    #[doc = "Dither Over 4 Reference Clock Period"]
    PER4,
    #[doc = "Dither Over 8 Reference Clock Period"]
    PER8,
    #[doc = "Dither Over 16 Reference Clock Period"]
    PER16,
    #[doc = "Dither Over 32 Reference Clock Period"]
    PER32,
}
impl crate::ToBits<u8> for PER_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PER_A::PER1 => 0,
            PER_A::PER2 => 1,
            PER_A::PER4 => 2,
            PER_A::PER8 => 3,
            PER_A::PER16 => 4,
            PER_A::PER32 => 5,
        }
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u8, PER_A>;
impl PER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PER_A::PER1),
            1 => Val(PER_A::PER2),
            2 => Val(PER_A::PER4),
            3 => Val(PER_A::PER8),
            4 => Val(PER_A::PER16),
            5 => Val(PER_A::PER32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PER1`"]
    #[inline(always)]
    pub fn is_per1(&self) -> bool {
        *self == PER_A::PER1
    }
    #[doc = "Checks if the value of the field is `PER2`"]
    #[inline(always)]
    pub fn is_per2(&self) -> bool {
        *self == PER_A::PER2
    }
    #[doc = "Checks if the value of the field is `PER4`"]
    #[inline(always)]
    pub fn is_per4(&self) -> bool {
        *self == PER_A::PER4
    }
    #[doc = "Checks if the value of the field is `PER8`"]
    #[inline(always)]
    pub fn is_per8(&self) -> bool {
        *self == PER_A::PER8
    }
    #[doc = "Checks if the value of the field is `PER16`"]
    #[inline(always)]
    pub fn is_per16(&self) -> bool {
        *self == PER_A::PER16
    }
    #[doc = "Checks if the value of the field is `PER32`"]
    #[inline(always)]
    pub fn is_per32(&self) -> bool {
        *self == PER_A::PER32
    }
}
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dither Over 1 Reference Clock Period"]
    #[inline(always)]
    pub fn per1(self) -> &'a mut W {
        self.variant(PER_A::PER1)
    }
    #[doc = "Dither Over 2 Reference Clock Period"]
    #[inline(always)]
    pub fn per2(self) -> &'a mut W {
        self.variant(PER_A::PER2)
    }
    #[doc = "Dither Over 4 Reference Clock Period"]
    #[inline(always)]
    pub fn per4(self) -> &'a mut W {
        self.variant(PER_A::PER4)
    }
    #[doc = "Dither Over 8 Reference Clock Period"]
    #[inline(always)]
    pub fn per8(self) -> &'a mut W {
        self.variant(PER_A::PER8)
    }
    #[doc = "Dither Over 16 Reference Clock Period"]
    #[inline(always)]
    pub fn per16(self) -> &'a mut W {
        self.variant(PER_A::PER16)
    }
    #[doc = "Dither Over 32 Reference Clock Period"]
    #[inline(always)]
    pub fn per32(self) -> &'a mut W {
        self.variant(PER_A::PER32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
