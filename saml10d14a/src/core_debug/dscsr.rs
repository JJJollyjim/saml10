#[doc = "Reader of register DSCSR"]
pub type R = crate::R<u32, super::DSCSR>;
#[doc = "Writer for register DSCSR"]
pub type W = crate::W<u32, super::DSCSR>;
#[doc = "Register DSCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBRSELEN`"]
pub type SBRSELEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBRSELEN`"]
pub struct SBRSELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSELEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SBRSEL`"]
pub type SBRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBRSEL`"]
pub struct SBRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSEL_W<'a> {
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
#[doc = "Reader of field `CDS`"]
pub type CDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDS`"]
pub struct CDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CDSKEY`"]
pub type CDSKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDSKEY`"]
pub struct CDSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Secure Banked register select enable"]
    #[inline(always)]
    pub fn sbrselen(&self) -> SBRSELEN_R {
        SBRSELEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure Banked register select"]
    #[inline(always)]
    pub fn sbrsel(&self) -> SBRSEL_R {
        SBRSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Current domain Secure"]
    #[inline(always)]
    pub fn cds(&self) -> CDS_R {
        CDS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CDS field write-enable key"]
    #[inline(always)]
    pub fn cdskey(&self) -> CDSKEY_R {
        CDSKEY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Banked register select enable"]
    #[inline(always)]
    pub fn sbrselen(&mut self) -> SBRSELEN_W {
        SBRSELEN_W { w: self }
    }
    #[doc = "Bit 1 - Secure Banked register select"]
    #[inline(always)]
    pub fn sbrsel(&mut self) -> SBRSEL_W {
        SBRSEL_W { w: self }
    }
    #[doc = "Bit 16 - Current domain Secure"]
    #[inline(always)]
    pub fn cds(&mut self) -> CDS_W {
        CDS_W { w: self }
    }
    #[doc = "Bit 17 - CDS field write-enable key"]
    #[inline(always)]
    pub fn cdskey(&mut self) -> CDSKEY_W {
        CDSKEY_W { w: self }
    }
}
