#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Control C"]
    pub ctrlc: CTRLC,
    _reserved3: [u8; 1usize],
    #[doc = "0x0a - Event Control"]
    pub evctrl: EVCTRL,
    _reserved4: [u8; 1usize],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 3usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved6: [u8; 3usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 3usize],
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    _reserved8: [u8; 2usize],
    #[doc = "0x1c - Address"]
    pub addr: ADDR,
    #[doc = "0x20 - Secure Unlock Register"]
    pub sulck: SULCK,
    #[doc = "0x22 - Non-Secure Unlock Register"]
    pub nsulck: NSULCK,
    #[doc = "0x24 - NVM Parameter"]
    pub param: PARAM,
    _reserved12: [u8; 8usize],
    #[doc = "0x30 - Data Scramble Configuration"]
    pub dscc: DSCC,
    #[doc = "0x34 - Security Control"]
    pub secctrl: SECCTRL,
    #[doc = "0x38 - Secure Boot Configuration"]
    pub scfgb: SCFGB,
    #[doc = "0x3c - Secure Application and Data Configuration"]
    pub scfgad: SCFGAD,
    #[doc = "0x40 - Non-secure Write Enable"]
    pub nonsec: NONSEC,
    #[doc = "0x44 - Non-secure Write Reference Value"]
    pub nschk: NSCHK,
}
#[doc = "Control A\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u16, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u32, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlc](ctrlc) module"]
pub type CTRLC = crate::Reg<u8, _CTRLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLC;
#[doc = "`read()` method returns [ctrlc::R](ctrlc::R) reader structure"]
impl crate::Readable for CTRLC {}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](ctrlc::W) writer structure"]
impl crate::Writable for CTRLC {}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u16, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address"]
pub mod addr;
#[doc = "Secure Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sulck](sulck) module"]
pub type SULCK = crate::Reg<u16, _SULCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SULCK;
#[doc = "`read()` method returns [sulck::R](sulck::R) reader structure"]
impl crate::Readable for SULCK {}
#[doc = "`write(|w| ..)` method takes [sulck::W](sulck::W) writer structure"]
impl crate::Writable for SULCK {}
#[doc = "Secure Unlock Register"]
pub mod sulck;
#[doc = "Non-Secure Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nsulck](nsulck) module"]
pub type NSULCK = crate::Reg<u16, _NSULCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSULCK;
#[doc = "`read()` method returns [nsulck::R](nsulck::R) reader structure"]
impl crate::Readable for NSULCK {}
#[doc = "`write(|w| ..)` method takes [nsulck::W](nsulck::W) writer structure"]
impl crate::Writable for NSULCK {}
#[doc = "Non-Secure Unlock Register"]
pub mod nsulck;
#[doc = "NVM Parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "`write(|w| ..)` method takes [param::W](param::W) writer structure"]
impl crate::Writable for PARAM {}
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "Data Scramble Configuration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscc](dscc) module"]
pub type DSCC = crate::Reg<u32, _DSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCC;
#[doc = "`write(|w| ..)` method takes [dscc::W](dscc::W) writer structure"]
impl crate::Writable for DSCC {}
#[doc = "Data Scramble Configuration"]
pub mod dscc;
#[doc = "Security Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secctrl](secctrl) module"]
pub type SECCTRL = crate::Reg<u32, _SECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECCTRL;
#[doc = "`read()` method returns [secctrl::R](secctrl::R) reader structure"]
impl crate::Readable for SECCTRL {}
#[doc = "`write(|w| ..)` method takes [secctrl::W](secctrl::W) writer structure"]
impl crate::Writable for SECCTRL {}
#[doc = "Security Control"]
pub mod secctrl;
#[doc = "Secure Boot Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scfgb](scfgb) module"]
pub type SCFGB = crate::Reg<u32, _SCFGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFGB;
#[doc = "`read()` method returns [scfgb::R](scfgb::R) reader structure"]
impl crate::Readable for SCFGB {}
#[doc = "`write(|w| ..)` method takes [scfgb::W](scfgb::W) writer structure"]
impl crate::Writable for SCFGB {}
#[doc = "Secure Boot Configuration"]
pub mod scfgb;
#[doc = "Secure Application and Data Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scfgad](scfgad) module"]
pub type SCFGAD = crate::Reg<u32, _SCFGAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFGAD;
#[doc = "`read()` method returns [scfgad::R](scfgad::R) reader structure"]
impl crate::Readable for SCFGAD {}
#[doc = "`write(|w| ..)` method takes [scfgad::W](scfgad::W) writer structure"]
impl crate::Writable for SCFGAD {}
#[doc = "Secure Application and Data Configuration"]
pub mod scfgad;
#[doc = "Non-secure Write Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonsec](nonsec) module"]
pub type NONSEC = crate::Reg<u32, _NONSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSEC;
#[doc = "`read()` method returns [nonsec::R](nonsec::R) reader structure"]
impl crate::Readable for NONSEC {}
#[doc = "`write(|w| ..)` method takes [nonsec::W](nonsec::W) writer structure"]
impl crate::Writable for NONSEC {}
#[doc = "Non-secure Write Enable"]
pub mod nonsec;
#[doc = "Non-secure Write Reference Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nschk](nschk) module"]
pub type NSCHK = crate::Reg<u32, _NSCHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSCHK;
#[doc = "`read()` method returns [nschk::R](nschk::R) reader structure"]
impl crate::Readable for NSCHK {}
#[doc = "`write(|w| ..)` method takes [nschk::W](nschk::W) writer structure"]
impl crate::Writable for NSCHK {}
#[doc = "Non-secure Write Reference Value"]
pub mod nschk;
