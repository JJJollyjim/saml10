#[doc = "Reader of register STATUSB"]
pub type R = crate::R<u8, super::STATUSB>;
#[doc = "Possible values of the field `DAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAL_A {
    #[doc = "`0`"]
    SECURED,
    #[doc = "`1`"]
    NS_DEBUG,
    #[doc = "`10`"]
    FULL_DEBUG,
}
impl crate::ToBits<u8> for DAL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DAL_A::SECURED => 0,
            DAL_A::NS_DEBUG => 1,
            DAL_A::FULL_DEBUG => 2,
        }
    }
}
#[doc = "Reader of field `DAL`"]
pub type DAL_R = crate::R<u8, DAL_A>;
impl DAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DAL_A::SECURED),
            1 => Val(DAL_A::NS_DEBUG),
            2 => Val(DAL_A::FULL_DEBUG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SECURED`"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == DAL_A::SECURED
    }
    #[doc = "Checks if the value of the field is `NS_DEBUG`"]
    #[inline(always)]
    pub fn is_ns_debug(&self) -> bool {
        *self == DAL_A::NS_DEBUG
    }
    #[doc = "Checks if the value of the field is `FULL_DEBUG`"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == DAL_A::FULL_DEBUG
    }
}
#[doc = "Reader of field `DBGPRES`"]
pub type DBGPRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPE`"]
pub type HPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCCD0`"]
pub type DCCD0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCCD1`"]
pub type DCCD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BCCD0`"]
pub type BCCD0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BCCD1`"]
pub type BCCD1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Debugger Access Level"]
    #[inline(always)]
    pub fn dal(&self) -> DAL_R {
        DAL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DBGPRES_R {
        DBGPRES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HPE_R {
        HPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> DCCD0_R {
        DCCD0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> DCCD1_R {
        DCCD1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Boot ROM Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn bccd0(&self) -> BCCD0_R {
        BCCD0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Boot ROM Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn bccd1(&self) -> BCCD1_R {
        BCCD1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
