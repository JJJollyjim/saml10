#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved3: [u8; 7usize],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20 - -"]
    pub channel: [CHANNEL; 8],
    _reserved8: [u8; 192usize],
    #[doc = "0x120 - User Multiplexer n"]
    pub user: [USER; 23],
    _reserved9: [u8; 157usize],
    #[doc = "0x1d4 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x1d5 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1d6 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved12: [u8; 1usize],
    #[doc = "0x1d8 - Channels Security Attribution"]
    pub nonsecchan: NONSECCHAN,
    #[doc = "0x1dc - Non-Secure Channels Check"]
    pub nschkchan: NSCHKCHAN,
    #[doc = "0x1e0 - Users Security Attribution"]
    pub nonsecuser: [NONSECUSER; 1],
    _reserved15: [u8; 12usize],
    #[doc = "0x1f0 - Non-Secure Users Check"]
    pub nschkuser: [NSCHKUSER; 1],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channel: self::channel::CHANNEL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r"Register block"]
#[doc = "-"]
pub mod channel;
#[doc = "Control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Software Event\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swevt](swevt) module"]
pub type SWEVT = crate::Reg<u32, _SWEVT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVT;
#[doc = "`write(|w| ..)` method takes [swevt::W](swevt::W) writer structure"]
impl crate::Writable for SWEVT {}
#[doc = "Software Event"]
pub mod swevt;
#[doc = "Priority Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prictrl](prictrl) module"]
pub type PRICTRL = crate::Reg<u8, _PRICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRICTRL;
#[doc = "`read()` method returns [prictrl::R](prictrl::R) reader structure"]
impl crate::Readable for PRICTRL {}
#[doc = "`write(|w| ..)` method takes [prictrl::W](prictrl::W) writer structure"]
impl crate::Writable for PRICTRL {}
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "Channel Pending Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u16, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "`write(|w| ..)` method takes [intpend::W](intpend::W) writer structure"]
impl crate::Writable for INTPEND {}
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [busych](busych) module"]
pub type BUSYCH = crate::Reg<u32, _BUSYCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSYCH;
#[doc = "`read()` method returns [busych::R](busych::R) reader structure"]
impl crate::Readable for BUSYCH {}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Ready Users\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [readyusr](readyusr) module"]
pub type READYUSR = crate::Reg<u32, _READYUSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READYUSR;
#[doc = "`read()` method returns [readyusr::R](readyusr::R) reader structure"]
impl crate::Readable for READYUSR {}
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "User Multiplexer n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user](user) module"]
pub type USER = crate::Reg<u8, _USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER;
#[doc = "`read()` method returns [user::R](user::R) reader structure"]
impl crate::Readable for USER {}
#[doc = "`write(|w| ..)` method takes [user::W](user::W) writer structure"]
impl crate::Writable for USER {}
#[doc = "User Multiplexer n"]
pub mod user;
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
#[doc = "Channels Security Attribution\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonsecchan](nonsecchan) module"]
pub type NONSECCHAN = crate::Reg<u32, _NONSECCHAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSECCHAN;
#[doc = "`read()` method returns [nonsecchan::R](nonsecchan::R) reader structure"]
impl crate::Readable for NONSECCHAN {}
#[doc = "`write(|w| ..)` method takes [nonsecchan::W](nonsecchan::W) writer structure"]
impl crate::Writable for NONSECCHAN {}
#[doc = "Channels Security Attribution"]
pub mod nonsecchan;
#[doc = "Non-Secure Channels Check\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nschkchan](nschkchan) module"]
pub type NSCHKCHAN = crate::Reg<u32, _NSCHKCHAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSCHKCHAN;
#[doc = "`read()` method returns [nschkchan::R](nschkchan::R) reader structure"]
impl crate::Readable for NSCHKCHAN {}
#[doc = "`write(|w| ..)` method takes [nschkchan::W](nschkchan::W) writer structure"]
impl crate::Writable for NSCHKCHAN {}
#[doc = "Non-Secure Channels Check"]
pub mod nschkchan;
#[doc = "Users Security Attribution\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonsecuser](nonsecuser) module"]
pub type NONSECUSER = crate::Reg<u32, _NONSECUSER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONSECUSER;
#[doc = "`read()` method returns [nonsecuser::R](nonsecuser::R) reader structure"]
impl crate::Readable for NONSECUSER {}
#[doc = "`write(|w| ..)` method takes [nonsecuser::W](nonsecuser::W) writer structure"]
impl crate::Writable for NONSECUSER {}
#[doc = "Users Security Attribution"]
pub mod nonsecuser;
#[doc = "Non-Secure Users Check\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nschkuser](nschkuser) module"]
pub type NSCHKUSER = crate::Reg<u32, _NSCHKUSER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSCHKUSER;
#[doc = "`read()` method returns [nschkuser::R](nschkuser::R) reader structure"]
impl crate::Readable for NSCHKUSER {}
#[doc = "`write(|w| ..)` method takes [nschkuser::W](nschkuser::W) writer structure"]
impl crate::Writable for NSCHKUSER {}
#[doc = "Non-Secure Users Check"]
pub mod nschkuser;
