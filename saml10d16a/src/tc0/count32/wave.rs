#[doc = "Reader of register WAVE"]
pub type R = crate::R<u8, super::WAVE>;
#[doc = "Writer for register WAVE"]
pub type W = crate::W<u8, super::WAVE>;
#[doc = "Register WAVE `reset()`'s with value 0"]
impl crate::ResetValue for super::WAVE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `WAVEGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGEN_A {
    #[doc = "Normal frequency"]
    NFRQ,
    #[doc = "Match frequency"]
    MFRQ,
    #[doc = "Normal PWM"]
    NPWM,
    #[doc = "Match PWM"]
    MPWM,
}
impl crate::ToBits<u8> for WAVEGEN_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WAVEGEN_A::NFRQ => 0,
            WAVEGEN_A::MFRQ => 1,
            WAVEGEN_A::NPWM => 2,
            WAVEGEN_A::MPWM => 3,
        }
    }
}
#[doc = "Reader of field `WAVEGEN`"]
pub type WAVEGEN_R = crate::R<u8, WAVEGEN_A>;
impl WAVEGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVEGEN_A {
        match self.bits {
            0 => WAVEGEN_A::NFRQ,
            1 => WAVEGEN_A::MFRQ,
            2 => WAVEGEN_A::NPWM,
            3 => WAVEGEN_A::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGEN_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGEN_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGEN_A::NPWM
    }
    #[doc = "Checks if the value of the field is `MPWM`"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGEN_A::MPWM
    }
}
#[doc = "Write proxy for field `WAVEGEN`"]
pub struct WAVEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVEGEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NPWM)
    }
    #[doc = "Match PWM"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut W {
        self.variant(WAVEGEN_A::MPWM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&mut self) -> WAVEGEN_W {
        WAVEGEN_W { w: self }
    }
}
