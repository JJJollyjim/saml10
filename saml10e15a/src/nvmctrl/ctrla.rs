#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u16, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_AW {
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    ER,
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP,
    #[doc = "Sets the power reduction mode."]
    SPRM,
    #[doc = "Clears the power reduction mode."]
    CPRM,
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    PBC,
    #[doc = "Invalidate all cache lines."]
    INVALL,
    #[doc = "Set DAL=0"]
    SDAL0,
    #[doc = "Set DAL=1"]
    SDAL1,
}
impl crate::ToBits<u8> for CMD_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CMD_AW::ER => 2,
            CMD_AW::WP => 4,
            CMD_AW::SPRM => 66,
            CMD_AW::CPRM => 67,
            CMD_AW::PBC => 68,
            CMD_AW::INVALL => 70,
            CMD_AW::SDAL0 => 75,
            CMD_AW::SDAL1 => 76,
        }
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn er(self) -> &'a mut W {
        self.variant(CMD_AW::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMD_AW::WP)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMD_AW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMD_AW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMD_AW::PBC)
    }
    #[doc = "Invalidate all cache lines."]
    #[inline(always)]
    pub fn invall(self) -> &'a mut W {
        self.variant(CMD_AW::INVALL)
    }
    #[doc = "Set DAL=0"]
    #[inline(always)]
    pub fn sdal0(self) -> &'a mut W {
        self.variant(CMD_AW::SDAL0)
    }
    #[doc = "Set DAL=1"]
    #[inline(always)]
    pub fn sdal1(self) -> &'a mut W {
        self.variant(CMD_AW::SDAL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Possible values of the field `CMDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEX_AW {
    #[doc = "Execution Key"]
    KEY,
}
impl crate::ToBits<u8> for CMDEX_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CMDEX_AW::KEY => 165,
        }
    }
}
#[doc = "Write proxy for field `CMDEX`"]
pub struct CMDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEX_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEX_AW::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&mut self) -> CMDEX_W {
        CMDEX_W { w: self }
    }
}
