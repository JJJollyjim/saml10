#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u32, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u32, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GTIME`"]
pub type GTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTIME`"]
pub struct GTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BRKLEN`"]
pub type BRKLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRKLEN`"]
pub struct BRKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `HDRDLY`"]
pub type HDRDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDRDLY`"]
pub struct HDRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `INACK`"]
pub type INACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INACK`"]
pub struct INACK_W<'a> {
    w: &'a mut W,
}
impl<'a> INACK_W<'a> {
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
#[doc = "Reader of field `DSNACK`"]
pub type DSNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSNACK`"]
pub struct DSNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSNACK_W<'a> {
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
#[doc = "Reader of field `MAXITER`"]
pub type MAXITER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXITER`"]
pub struct MAXITER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GTIME_R {
        GTIME_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BRKLEN_R {
        BRKLEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HDRDLY_R {
        HDRDLY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MAXITER_R {
        MAXITER_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&mut self) -> GTIME_W {
        GTIME_W { w: self }
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BRKLEN_W {
        BRKLEN_W { w: self }
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&mut self) -> HDRDLY_W {
        HDRDLY_W { w: self }
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> INACK_W {
        INACK_W { w: self }
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DSNACK_W {
        DSNACK_W { w: self }
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&mut self) -> MAXITER_W {
        MAXITER_W { w: self }
    }
}
