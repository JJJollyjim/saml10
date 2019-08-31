#[doc = "Reader of register LUTCTRL[%s]"]
pub type R = crate::R<u32, super::LUTCTRL>;
#[doc = "Writer for register LUTCTRL[%s]"]
pub type W = crate::W<u32, super::LUTCTRL>;
#[doc = "Register LUTCTRL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::LUTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `FILTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSEL_A {
    #[doc = "Filter disabled"]
    DISABLE,
    #[doc = "Synchronizer enabled"]
    SYNCH,
    #[doc = "Filter enabled"]
    FILTER,
}
impl crate::ToBits<u8> for FILTSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FILTSEL_A::DISABLE => 0,
            FILTSEL_A::SYNCH => 1,
            FILTSEL_A::FILTER => 2,
        }
    }
}
#[doc = "Reader of field `FILTSEL`"]
pub type FILTSEL_R = crate::R<u8, FILTSEL_A>;
impl FILTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTSEL_A::DISABLE),
            1 => Val(FILTSEL_A::SYNCH),
            2 => Val(FILTSEL_A::FILTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == FILTSEL_A::SYNCH
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == FILTSEL_A::FILTER
    }
}
#[doc = "Write proxy for field `FILTSEL`"]
pub struct FILTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTSEL_A::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut W {
        self.variant(FILTSEL_A::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(FILTSEL_A::FILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EDGESEL`"]
pub type EDGESEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGESEL`"]
pub struct EDGESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGESEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `INSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL0_A {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = "Alternate 2 TC input source"]
    ALT2TC,
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    ASYNCEVENT,
}
impl crate::ToBits<u8> for INSEL0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INSEL0_A::MASK => 0,
            INSEL0_A::FEEDBACK => 1,
            INSEL0_A::LINK => 2,
            INSEL0_A::EVENT => 3,
            INSEL0_A::IO => 4,
            INSEL0_A::AC => 5,
            INSEL0_A::TC => 6,
            INSEL0_A::ALTTC => 7,
            INSEL0_A::TCC => 8,
            INSEL0_A::SERCOM => 9,
            INSEL0_A::ALT2TC => 10,
            INSEL0_A::ASYNCEVENT => 11,
        }
    }
}
#[doc = "Reader of field `INSEL0`"]
pub type INSEL0_R = crate::R<u8, INSEL0_A>;
impl INSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INSEL0_A::MASK),
            1 => Val(INSEL0_A::FEEDBACK),
            2 => Val(INSEL0_A::LINK),
            3 => Val(INSEL0_A::EVENT),
            4 => Val(INSEL0_A::IO),
            5 => Val(INSEL0_A::AC),
            6 => Val(INSEL0_A::TC),
            7 => Val(INSEL0_A::ALTTC),
            8 => Val(INSEL0_A::TCC),
            9 => Val(INSEL0_A::SERCOM),
            10 => Val(INSEL0_A::ALT2TC),
            11 => Val(INSEL0_A::ASYNCEVENT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL0_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL0_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL0_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == INSEL0_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL0_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == INSEL0_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == INSEL0_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL0_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL0_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL0_A::SERCOM
    }
    #[doc = "Checks if the value of the field is `ALT2TC`"]
    #[inline(always)]
    pub fn is_alt2tc(&self) -> bool {
        *self == INSEL0_A::ALT2TC
    }
    #[doc = "Checks if the value of the field is `ASYNCEVENT`"]
    #[inline(always)]
    pub fn is_asyncevent(&self) -> bool {
        *self == INSEL0_A::ASYNCEVENT
    }
}
#[doc = "Write proxy for field `INSEL0`"]
pub struct INSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL0_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL0_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL0_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL0_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL0_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL0_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL0_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL0_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL0_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL0_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL0_A::SERCOM)
    }
    #[doc = "Alternate 2 TC input source"]
    #[inline(always)]
    pub fn alt2tc(self) -> &'a mut W {
        self.variant(INSEL0_A::ALT2TC)
    }
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    #[inline(always)]
    pub fn asyncevent(self) -> &'a mut W {
        self.variant(INSEL0_A::ASYNCEVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `INSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL1_A {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = "Alternate 2 TC input source"]
    ALT2TC,
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    ASYNCEVENT,
}
impl crate::ToBits<u8> for INSEL1_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INSEL1_A::MASK => 0,
            INSEL1_A::FEEDBACK => 1,
            INSEL1_A::LINK => 2,
            INSEL1_A::EVENT => 3,
            INSEL1_A::IO => 4,
            INSEL1_A::AC => 5,
            INSEL1_A::TC => 6,
            INSEL1_A::ALTTC => 7,
            INSEL1_A::TCC => 8,
            INSEL1_A::SERCOM => 9,
            INSEL1_A::ALT2TC => 10,
            INSEL1_A::ASYNCEVENT => 11,
        }
    }
}
#[doc = "Reader of field `INSEL1`"]
pub type INSEL1_R = crate::R<u8, INSEL1_A>;
impl INSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INSEL1_A::MASK),
            1 => Val(INSEL1_A::FEEDBACK),
            2 => Val(INSEL1_A::LINK),
            3 => Val(INSEL1_A::EVENT),
            4 => Val(INSEL1_A::IO),
            5 => Val(INSEL1_A::AC),
            6 => Val(INSEL1_A::TC),
            7 => Val(INSEL1_A::ALTTC),
            8 => Val(INSEL1_A::TCC),
            9 => Val(INSEL1_A::SERCOM),
            10 => Val(INSEL1_A::ALT2TC),
            11 => Val(INSEL1_A::ASYNCEVENT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL1_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL1_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL1_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == INSEL1_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL1_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == INSEL1_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == INSEL1_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL1_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL1_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL1_A::SERCOM
    }
    #[doc = "Checks if the value of the field is `ALT2TC`"]
    #[inline(always)]
    pub fn is_alt2tc(&self) -> bool {
        *self == INSEL1_A::ALT2TC
    }
    #[doc = "Checks if the value of the field is `ASYNCEVENT`"]
    #[inline(always)]
    pub fn is_asyncevent(&self) -> bool {
        *self == INSEL1_A::ASYNCEVENT
    }
}
#[doc = "Write proxy for field `INSEL1`"]
pub struct INSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL1_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL1_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL1_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL1_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL1_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL1_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL1_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL1_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL1_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL1_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL1_A::SERCOM)
    }
    #[doc = "Alternate 2 TC input source"]
    #[inline(always)]
    pub fn alt2tc(self) -> &'a mut W {
        self.variant(INSEL1_A::ALT2TC)
    }
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    #[inline(always)]
    pub fn asyncevent(self) -> &'a mut W {
        self.variant(INSEL1_A::ASYNCEVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `INSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL2_A {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = "Alternate 2 TC input source"]
    ALT2TC,
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    ASYNCEVENT,
}
impl crate::ToBits<u8> for INSEL2_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INSEL2_A::MASK => 0,
            INSEL2_A::FEEDBACK => 1,
            INSEL2_A::LINK => 2,
            INSEL2_A::EVENT => 3,
            INSEL2_A::IO => 4,
            INSEL2_A::AC => 5,
            INSEL2_A::TC => 6,
            INSEL2_A::ALTTC => 7,
            INSEL2_A::TCC => 8,
            INSEL2_A::SERCOM => 9,
            INSEL2_A::ALT2TC => 10,
            INSEL2_A::ASYNCEVENT => 11,
        }
    }
}
#[doc = "Reader of field `INSEL2`"]
pub type INSEL2_R = crate::R<u8, INSEL2_A>;
impl INSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INSEL2_A::MASK),
            1 => Val(INSEL2_A::FEEDBACK),
            2 => Val(INSEL2_A::LINK),
            3 => Val(INSEL2_A::EVENT),
            4 => Val(INSEL2_A::IO),
            5 => Val(INSEL2_A::AC),
            6 => Val(INSEL2_A::TC),
            7 => Val(INSEL2_A::ALTTC),
            8 => Val(INSEL2_A::TCC),
            9 => Val(INSEL2_A::SERCOM),
            10 => Val(INSEL2_A::ALT2TC),
            11 => Val(INSEL2_A::ASYNCEVENT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL2_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL2_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL2_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == INSEL2_A::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL2_A::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == INSEL2_A::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == INSEL2_A::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL2_A::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL2_A::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL2_A::SERCOM
    }
    #[doc = "Checks if the value of the field is `ALT2TC`"]
    #[inline(always)]
    pub fn is_alt2tc(&self) -> bool {
        *self == INSEL2_A::ALT2TC
    }
    #[doc = "Checks if the value of the field is `ASYNCEVENT`"]
    #[inline(always)]
    pub fn is_asyncevent(&self) -> bool {
        *self == INSEL2_A::ASYNCEVENT
    }
}
#[doc = "Write proxy for field `INSEL2`"]
pub struct INSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL2_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL2_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL2_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL2_A::LINK)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL2_A::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL2_A::IO)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL2_A::AC)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL2_A::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL2_A::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL2_A::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL2_A::SERCOM)
    }
    #[doc = "Alternate 2 TC input source"]
    #[inline(always)]
    pub fn alt2tc(self) -> &'a mut W {
        self.variant(INSEL2_A::ALT2TC)
    }
    #[doc = "Asynchronous event input source. The EVENT input will bypass edge detection logic."]
    #[inline(always)]
    pub fn asyncevent(self) -> &'a mut W {
        self.variant(INSEL2_A::ASYNCEVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `INVEI`"]
pub type INVEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI`"]
pub struct INVEI_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LUTEI`"]
pub type LUTEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUTEI`"]
pub struct LUTEI_W<'a> {
    w: &'a mut W,
}
impl<'a> LUTEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LUTEO`"]
pub type LUTEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUTEO`"]
pub struct LUTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> LUTEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TRUTH`"]
pub type TRUTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRUTH`"]
pub struct TRUTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn invei(&self) -> INVEI_R {
        INVEI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&self) -> LUTEI_R {
        LUTEI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&self) -> LUTEO_R {
        LUTEO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FILTSEL_W {
        FILTSEL_W { w: self }
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EDGESEL_W {
        EDGESEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&mut self) -> INSEL0_W {
        INSEL0_W { w: self }
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&mut self) -> INSEL1_W {
        INSEL1_W { w: self }
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&mut self) -> INSEL2_W {
        INSEL2_W { w: self }
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn invei(&mut self) -> INVEI_W {
        INVEI_W { w: self }
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&mut self) -> LUTEI_W {
        LUTEI_W { w: self }
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&mut self) -> LUTEO_W {
        LUTEO_W { w: self }
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&mut self) -> TRUTH_W {
        TRUTH_W { w: self }
    }
}
