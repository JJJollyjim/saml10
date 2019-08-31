#[doc = "Reader of register NONSECB"]
pub type R = crate::R<u32, super::NONSECB>;
#[doc = "Reader of field `IDAU_`"]
pub type IDAU__R = crate::R<bool, bool>;
#[doc = "Reader of field `DSU_`"]
pub type DSU__R = crate::R<bool, bool>;
#[doc = "Reader of field `NVMCTRL_`"]
pub type NVMCTRL__R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAC_`"]
pub type DMAC__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IDAU Non-Secure"]
    #[inline(always)]
    pub fn idau_(&self) -> IDAU__R {
        IDAU__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU Non-Secure"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL Non-Secure"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAC Non-Secure"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
