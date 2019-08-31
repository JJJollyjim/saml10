#[doc = "Reader of register DAUTHSTATUS"]
pub type R = crate::R<u32, super::DAUTHSTATUS>;
#[doc = "Possible values of the field `SID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SID_A {
    #[doc = "Security Extension not implemented"]
    NOSEC,
    #[doc = "Secure invasive debug prohibited"]
    NO,
    #[doc = "Secure invasive debug allowed"]
    YES,
}
impl crate::ToBits<u8> for SID_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SID_A::NOSEC => 0,
            SID_A::NO => 2,
            SID_A::YES => 3,
        }
    }
}
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u8, SID_A>;
impl SID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SID_A::NOSEC),
            2 => Val(SID_A::NO),
            3 => Val(SID_A::YES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SID_A::YES
    }
}
#[doc = "Possible values of the field `SNID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNID_A {
    #[doc = "Security Extension not implemented"]
    NOSEC,
    #[doc = "Secure non-invasive debug prohibited"]
    NO,
    #[doc = "Secure non-invasive debug allowed"]
    YES,
}
impl crate::ToBits<u8> for SNID_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SNID_A::NOSEC => 0,
            SNID_A::NO => 2,
            SNID_A::YES => 3,
        }
    }
}
#[doc = "Reader of field `SNID`"]
pub type SNID_R = crate::R<u8, SNID_A>;
impl SNID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SNID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SNID_A::NOSEC),
            2 => Val(SNID_A::NO),
            3 => Val(SNID_A::YES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SNID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SNID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SNID_A::YES
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
