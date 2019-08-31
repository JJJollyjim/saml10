#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick Control and Status Register"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x04 - SysTick Reload Value Register"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x08 - SysTick Current Value Register"]
    pub syst_cvr: SYST_CVR,
    #[doc = "0x0c - SysTick Calibration Value Register"]
    pub syst_calib: SYST_CALIB,
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_csr](syst_csr) module"]
pub type SYST_CSR = crate::Reg<u32, _SYST_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CSR;
#[doc = "`read()` method returns [syst_csr::R](syst_csr::R) reader structure"]
impl crate::Readable for SYST_CSR {}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](syst_csr::W) writer structure"]
impl crate::Writable for SYST_CSR {}
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SysTick Reload Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_rvr](syst_rvr) module"]
pub type SYST_RVR = crate::Reg<u32, _SYST_RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_RVR;
#[doc = "`read()` method returns [syst_rvr::R](syst_rvr::R) reader structure"]
impl crate::Readable for SYST_RVR {}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](syst_rvr::W) writer structure"]
impl crate::Writable for SYST_RVR {}
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_cvr](syst_cvr) module"]
pub type SYST_CVR = crate::Reg<u32, _SYST_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CVR;
#[doc = "`read()` method returns [syst_cvr::R](syst_cvr::R) reader structure"]
impl crate::Readable for SYST_CVR {}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](syst_cvr::W) writer structure"]
impl crate::Writable for SYST_CVR {}
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SysTick Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_calib](syst_calib) module"]
pub type SYST_CALIB = crate::Reg<u32, _SYST_CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CALIB;
#[doc = "`read()` method returns [syst_calib::R](syst_calib::R) reader structure"]
impl crate::Readable for SYST_CALIB {}
#[doc = "SysTick Calibration Value Register"]
pub mod syst_calib;
