#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    _reserved8: [u8; 20usize],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    _reserved11: [u8; 20usize],
    #[doc = "0x54 - Peripheral non-secure status - Bridge A"]
    pub nonseca: NONSECA,
    #[doc = "0x58 - Peripheral non-secure status - Bridge B"]
    pub nonsecb: NONSECB,
    #[doc = "0x5c - Peripheral non-secure status - Bridge C"]
    pub nonsecc: NONSECC,
    _reserved14: [u8; 20usize],
    #[doc = "0x74 - Peripheral secure status locked - Bridge A"]
    pub seclocka: SECLOCKA,
    #[doc = "0x78 - Peripheral secure status locked - Bridge B"]
    pub seclockb: SECLOCKB,
    #[doc = "0x7c - Peripheral secure status locked - Bridge C"]
    pub seclockc: SECLOCKC,
}
#[doc = "Write control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wrctrl](wrctrl) module"]
pub type WRCTRL = crate::Reg<u32, _WRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRCTRL;
#[doc = "`read()` method returns [wrctrl::R](wrctrl::R) reader structure"]
impl crate::Readable for WRCTRL {}
#[doc = "`write(|w| ..)` method takes [wrctrl::W](wrctrl::W) writer structure"]
impl crate::Writable for WRCTRL {}
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "Event control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event control"]
pub mod evctrl;
#[doc = "Interrupt enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt enable set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflagahb](intflagahb) module"]
pub type INTFLAGAHB = crate::Reg<u32, _INTFLAGAHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGAHB;
#[doc = "`read()` method returns [intflagahb::R](intflagahb::R) reader structure"]
impl crate::Readable for INTFLAGAHB {}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](intflagahb::W) writer structure"]
impl crate::Writable for INTFLAGAHB {}
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "Peripheral interrupt flag status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflaga](intflaga) module"]
pub type INTFLAGA = crate::Reg<u32, _INTFLAGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGA;
#[doc = "`read()` method returns [intflaga::R](intflaga::R) reader structure"]
impl crate::Readable for INTFLAGA {}
#[doc = "`write(|w| ..)` method takes [intflaga::W](intflaga::W) writer structure"]
impl crate::Writable for INTFLAGA {}
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "Peripheral interrupt flag status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflagb](intflagb) module"]
pub type INTFLAGB = crate::Reg<u32, _INTFLAGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGB;
#[doc = "`read()` method returns [intflagb::R](intflagb::R) reader structure"]
impl crate::Readable for INTFLAGB {}
#[doc = "`write(|w| ..)` method takes [intflagb::W](intflagb::W) writer structure"]
impl crate::Writable for INTFLAGB {}
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflagc](intflagc) module"]
pub type INTFLAGC = crate::Reg<u32, _INTFLAGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGC;
#[doc = "`read()` method returns [intflagc::R](intflagc::R) reader structure"]
impl crate::Readable for INTFLAGC {}
#[doc = "`write(|w| ..)` method takes [intflagc::W](intflagc::W) writer structure"]
impl crate::Writable for INTFLAGC {}
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "Peripheral write protection status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusa](statusa) module"]
pub type STATUSA = crate::Reg<u32, _STATUSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSA;
#[doc = "`read()` method returns [statusa::R](statusa::R) reader structure"]
impl crate::Readable for STATUSA {}
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "Peripheral write protection status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusb](statusb) module"]
pub type STATUSB = crate::Reg<u32, _STATUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSB;
#[doc = "`read()` method returns [statusb::R](statusb::R) reader structure"]
impl crate::Readable for STATUSB {}
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [statusc](statusc) module"]
pub type STATUSC = crate::Reg<u32, _STATUSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSC;
#[doc = "`read()` method returns [statusc::R](statusc::R) reader structure"]
impl crate::Readable for STATUSC {}
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "Peripheral non-secure status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonseca](nonseca) module"]
pub type NONSECA = crate::Reg<u32, _NONSECA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSECA;
#[doc = "`read()` method returns [nonseca::R](nonseca::R) reader structure"]
impl crate::Readable for NONSECA {}
#[doc = "Peripheral non-secure status - Bridge A"]
pub mod nonseca;
#[doc = "Peripheral non-secure status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonsecb](nonsecb) module"]
pub type NONSECB = crate::Reg<u32, _NONSECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSECB;
#[doc = "`read()` method returns [nonsecb::R](nonsecb::R) reader structure"]
impl crate::Readable for NONSECB {}
#[doc = "Peripheral non-secure status - Bridge B"]
pub mod nonsecb;
#[doc = "Peripheral non-secure status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonsecc](nonsecc) module"]
pub type NONSECC = crate::Reg<u32, _NONSECC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSECC;
#[doc = "`read()` method returns [nonsecc::R](nonsecc::R) reader structure"]
impl crate::Readable for NONSECC {}
#[doc = "Peripheral non-secure status - Bridge C"]
pub mod nonsecc;
#[doc = "Peripheral secure status locked - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seclocka](seclocka) module"]
pub type SECLOCKA = crate::Reg<u32, _SECLOCKA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECLOCKA;
#[doc = "`read()` method returns [seclocka::R](seclocka::R) reader structure"]
impl crate::Readable for SECLOCKA {}
#[doc = "Peripheral secure status locked - Bridge A"]
pub mod seclocka;
#[doc = "Peripheral secure status locked - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seclockb](seclockb) module"]
pub type SECLOCKB = crate::Reg<u32, _SECLOCKB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECLOCKB;
#[doc = "`read()` method returns [seclockb::R](seclockb::R) reader structure"]
impl crate::Readable for SECLOCKB {}
#[doc = "Peripheral secure status locked - Bridge B"]
pub mod seclockb;
#[doc = "Peripheral secure status locked - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seclockc](seclockc) module"]
pub type SECLOCKC = crate::Reg<u32, _SECLOCKC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECLOCKC;
#[doc = "`read()` method returns [seclockc::R](seclockc::R) reader structure"]
impl crate::Readable for SECLOCKC {}
#[doc = "Peripheral secure status locked - Bridge C"]
pub mod seclockc;
