#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u8, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u8, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PROGE`"]
pub type PROGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGE`"]
pub struct PROGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LOCKE`"]
pub type LOCKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKE`"]
pub struct LOCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `NVME`"]
pub type NVME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVME`"]
pub struct NVME_W<'a> {
    w: &'a mut W,
}
impl<'a> NVME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `KEYE`"]
pub type KEYE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYE`"]
pub struct KEYE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NSCHK`"]
pub type NSCHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSCHK`"]
pub struct NSCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NSCHK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NVM Done Interrupt Clear"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Error Status Interrupt Clear"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Error Status Interrupt Clear"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NVM Error Interrupt Clear"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Key Write Error Interrupt Clear"]
    #[inline(always)]
    pub fn keye(&self) -> KEYE_R {
        KEYE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NS configuration change detected Interrupt Clear"]
    #[inline(always)]
    pub fn nschk(&self) -> NSCHK_R {
        NSCHK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Done Interrupt Clear"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 1 - Programming Error Status Interrupt Clear"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W { w: self }
    }
    #[doc = "Bit 2 - Lock Error Status Interrupt Clear"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W { w: self }
    }
    #[doc = "Bit 3 - NVM Error Interrupt Clear"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NVME_W {
        NVME_W { w: self }
    }
    #[doc = "Bit 4 - Key Write Error Interrupt Clear"]
    #[inline(always)]
    pub fn keye(&mut self) -> KEYE_W {
        KEYE_W { w: self }
    }
    #[doc = "Bit 5 - NS configuration change detected Interrupt Clear"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NSCHK_W {
        NSCHK_W { w: self }
    }
}
