#[doc = "Reader of register INPUTCTRL"]
pub type R = crate::R<u16, super::INPUTCTRL>;
#[doc = "Writer for register INPUTCTRL"]
pub type W = crate::W<u16, super::INPUTCTRL>;
#[doc = "Register INPUTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `MUXPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXPOS_A {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
    #[doc = "ADC AIN8 Pin"]
    AIN8,
    #[doc = "ADC AIN9 Pin"]
    AIN9,
    #[doc = "ADC AIN10 Pin"]
    AIN10,
    #[doc = "ADC AIN11 Pin"]
    AIN11,
    #[doc = "ADC AIN12 Pin"]
    AIN12,
    #[doc = "ADC AIN13 Pin"]
    AIN13,
    #[doc = "ADC AIN14 Pin"]
    AIN14,
    #[doc = "ADC AIN15 Pin"]
    AIN15,
    #[doc = "ADC AIN16 Pin"]
    AIN16,
    #[doc = "ADC AIN17 Pin"]
    AIN17,
    #[doc = "ADC AIN18 Pin"]
    AIN18,
    #[doc = "ADC AIN19 Pin"]
    AIN19,
    #[doc = "ADC AIN20 Pin"]
    AIN20,
    #[doc = "ADC AIN21 Pin"]
    AIN21,
    #[doc = "ADC AIN22 Pin"]
    AIN22,
    #[doc = "ADC AIN23 Pin"]
    AIN23,
    #[doc = "Temperature Sensor"]
    TEMP,
    #[doc = "Bandgap Voltage"]
    BANDGAP,
    #[doc = "1/4 Scaled Core Supply"]
    SCALEDCOREVCC,
    #[doc = "1/4 Scaled I/O Supply"]
    SCALEDIOVCC,
    #[doc = "DAC Output"]
    DAC,
    #[doc = "1/4 Scaled VBAT Supply"]
    SCALEDVBAT,
    #[doc = "OPAMP0 or OPAMP1 output"]
    OPAMP01,
    #[doc = "OPAMP2 output"]
    OPAMP2,
}
impl crate::ToBits<u8> for MUXPOS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MUXPOS_A::AIN0 => 0,
            MUXPOS_A::AIN1 => 1,
            MUXPOS_A::AIN2 => 2,
            MUXPOS_A::AIN3 => 3,
            MUXPOS_A::AIN4 => 4,
            MUXPOS_A::AIN5 => 5,
            MUXPOS_A::AIN6 => 6,
            MUXPOS_A::AIN7 => 7,
            MUXPOS_A::AIN8 => 8,
            MUXPOS_A::AIN9 => 9,
            MUXPOS_A::AIN10 => 10,
            MUXPOS_A::AIN11 => 11,
            MUXPOS_A::AIN12 => 12,
            MUXPOS_A::AIN13 => 13,
            MUXPOS_A::AIN14 => 14,
            MUXPOS_A::AIN15 => 15,
            MUXPOS_A::AIN16 => 16,
            MUXPOS_A::AIN17 => 17,
            MUXPOS_A::AIN18 => 18,
            MUXPOS_A::AIN19 => 19,
            MUXPOS_A::AIN20 => 20,
            MUXPOS_A::AIN21 => 21,
            MUXPOS_A::AIN22 => 22,
            MUXPOS_A::AIN23 => 23,
            MUXPOS_A::TEMP => 24,
            MUXPOS_A::BANDGAP => 25,
            MUXPOS_A::SCALEDCOREVCC => 26,
            MUXPOS_A::SCALEDIOVCC => 27,
            MUXPOS_A::DAC => 28,
            MUXPOS_A::SCALEDVBAT => 29,
            MUXPOS_A::OPAMP01 => 30,
            MUXPOS_A::OPAMP2 => 31,
        }
    }
}
#[doc = "Reader of field `MUXPOS`"]
pub type MUXPOS_R = crate::R<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXPOS_A {
        match self.bits {
            0 => MUXPOS_A::AIN0,
            1 => MUXPOS_A::AIN1,
            2 => MUXPOS_A::AIN2,
            3 => MUXPOS_A::AIN3,
            4 => MUXPOS_A::AIN4,
            5 => MUXPOS_A::AIN5,
            6 => MUXPOS_A::AIN6,
            7 => MUXPOS_A::AIN7,
            8 => MUXPOS_A::AIN8,
            9 => MUXPOS_A::AIN9,
            10 => MUXPOS_A::AIN10,
            11 => MUXPOS_A::AIN11,
            12 => MUXPOS_A::AIN12,
            13 => MUXPOS_A::AIN13,
            14 => MUXPOS_A::AIN14,
            15 => MUXPOS_A::AIN15,
            16 => MUXPOS_A::AIN16,
            17 => MUXPOS_A::AIN17,
            18 => MUXPOS_A::AIN18,
            19 => MUXPOS_A::AIN19,
            20 => MUXPOS_A::AIN20,
            21 => MUXPOS_A::AIN21,
            22 => MUXPOS_A::AIN22,
            23 => MUXPOS_A::AIN23,
            24 => MUXPOS_A::TEMP,
            25 => MUXPOS_A::BANDGAP,
            26 => MUXPOS_A::SCALEDCOREVCC,
            27 => MUXPOS_A::SCALEDIOVCC,
            28 => MUXPOS_A::DAC,
            29 => MUXPOS_A::SCALEDVBAT,
            30 => MUXPOS_A::OPAMP01,
            31 => MUXPOS_A::OPAMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOS_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOS_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOS_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOS_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOS_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOS_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOS_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOS_A::AIN7
    }
    #[doc = "Checks if the value of the field is `AIN8`"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOS_A::AIN8
    }
    #[doc = "Checks if the value of the field is `AIN9`"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOS_A::AIN9
    }
    #[doc = "Checks if the value of the field is `AIN10`"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOS_A::AIN10
    }
    #[doc = "Checks if the value of the field is `AIN11`"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOS_A::AIN11
    }
    #[doc = "Checks if the value of the field is `AIN12`"]
    #[inline(always)]
    pub fn is_ain12(&self) -> bool {
        *self == MUXPOS_A::AIN12
    }
    #[doc = "Checks if the value of the field is `AIN13`"]
    #[inline(always)]
    pub fn is_ain13(&self) -> bool {
        *self == MUXPOS_A::AIN13
    }
    #[doc = "Checks if the value of the field is `AIN14`"]
    #[inline(always)]
    pub fn is_ain14(&self) -> bool {
        *self == MUXPOS_A::AIN14
    }
    #[doc = "Checks if the value of the field is `AIN15`"]
    #[inline(always)]
    pub fn is_ain15(&self) -> bool {
        *self == MUXPOS_A::AIN15
    }
    #[doc = "Checks if the value of the field is `AIN16`"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == MUXPOS_A::AIN16
    }
    #[doc = "Checks if the value of the field is `AIN17`"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == MUXPOS_A::AIN17
    }
    #[doc = "Checks if the value of the field is `AIN18`"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == MUXPOS_A::AIN18
    }
    #[doc = "Checks if the value of the field is `AIN19`"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == MUXPOS_A::AIN19
    }
    #[doc = "Checks if the value of the field is `AIN20`"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == MUXPOS_A::AIN20
    }
    #[doc = "Checks if the value of the field is `AIN21`"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == MUXPOS_A::AIN21
    }
    #[doc = "Checks if the value of the field is `AIN22`"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == MUXPOS_A::AIN22
    }
    #[doc = "Checks if the value of the field is `AIN23`"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == MUXPOS_A::AIN23
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOS_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOS_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOS_A::DAC
    }
    #[doc = "Checks if the value of the field is `SCALEDVBAT`"]
    #[inline(always)]
    pub fn is_scaledvbat(&self) -> bool {
        *self == MUXPOS_A::SCALEDVBAT
    }
    #[doc = "Checks if the value of the field is `OPAMP01`"]
    #[inline(always)]
    pub fn is_opamp01(&self) -> bool {
        *self == MUXPOS_A::OPAMP01
    }
    #[doc = "Checks if the value of the field is `OPAMP2`"]
    #[inline(always)]
    pub fn is_opamp2(&self) -> bool {
        *self == MUXPOS_A::OPAMP2
    }
}
#[doc = "Write proxy for field `MUXPOS`"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXPOS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn ain12(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn ain13(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn ain14(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn ain15(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN15)
    }
    #[doc = "ADC AIN16 Pin"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN16)
    }
    #[doc = "ADC AIN17 Pin"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN17)
    }
    #[doc = "ADC AIN18 Pin"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN18)
    }
    #[doc = "ADC AIN19 Pin"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN19)
    }
    #[doc = "ADC AIN20 Pin"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN20)
    }
    #[doc = "ADC AIN21 Pin"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN21)
    }
    #[doc = "ADC AIN22 Pin"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN22)
    }
    #[doc = "ADC AIN23 Pin"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN23)
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOS_A::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOS_A::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDIOVCC)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOS_A::DAC)
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn scaledvbat(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDVBAT)
    }
    #[doc = "OPAMP0 or OPAMP1 output"]
    #[inline(always)]
    pub fn opamp01(self) -> &'a mut W {
        self.variant(MUXPOS_A::OPAMP01)
    }
    #[doc = "OPAMP2 output"]
    #[inline(always)]
    pub fn opamp2(self) -> &'a mut W {
        self.variant(MUXPOS_A::OPAMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `MUXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXNEG_A {
    #[doc = "ADC AIN0 Pin"]
    AIN0,
    #[doc = "ADC AIN1 Pin"]
    AIN1,
    #[doc = "ADC AIN2 Pin"]
    AIN2,
    #[doc = "ADC AIN3 Pin"]
    AIN3,
    #[doc = "ADC AIN4 Pin"]
    AIN4,
    #[doc = "ADC AIN5 Pin"]
    AIN5,
    #[doc = "ADC AIN6 Pin"]
    AIN6,
    #[doc = "ADC AIN7 Pin"]
    AIN7,
}
impl crate::ToBits<u8> for MUXNEG_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MUXNEG_A::AIN0 => 0,
            MUXNEG_A::AIN1 => 1,
            MUXNEG_A::AIN2 => 2,
            MUXNEG_A::AIN3 => 3,
            MUXNEG_A::AIN4 => 4,
            MUXNEG_A::AIN5 => 5,
            MUXNEG_A::AIN6 => 6,
            MUXNEG_A::AIN7 => 7,
        }
    }
}
#[doc = "Reader of field `MUXNEG`"]
pub type MUXNEG_R = crate::R<u8, MUXNEG_A>;
impl MUXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXNEG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXNEG_A::AIN0),
            1 => Val(MUXNEG_A::AIN1),
            2 => Val(MUXNEG_A::AIN2),
            3 => Val(MUXNEG_A::AIN3),
            4 => Val(MUXNEG_A::AIN4),
            5 => Val(MUXNEG_A::AIN5),
            6 => Val(MUXNEG_A::AIN6),
            7 => Val(MUXNEG_A::AIN7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXNEG_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEG_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEG_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEG_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEG_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEG_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXNEG_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXNEG_A::AIN7
    }
}
#[doc = "Write proxy for field `MUXNEG`"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXNEG_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
}
