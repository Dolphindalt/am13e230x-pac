#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x0800],
    pdbgctl: Pdbgctl,
    _reserved2: [u8; 0x04],
    int_event0: IntEvent0,
    _reserved3: [u8; 0x04],
    int_event1: IntEvent1,
    _reserved4: [u8; 0x04],
    int_event2: IntEvent2,
    _reserved5: [u8; 0x34],
    evt_mode: EvtMode,
    _reserved6: [u8; 0x1c],
    gcmccm_tag0: GcmccmTag0,
    gcmccm_tag1: GcmccmTag1,
    gcmccm_tag2: GcmccmTag2,
    gcmccm_tag3: GcmccmTag3,
    ghash_h0: GhashH0,
    ghash_h1: GhashH1,
    ghash_h2: GhashH2,
    ghash_h3: GhashH3,
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    key4: Key4,
    key5: Key5,
    key6: Key6,
    key7: Key7,
    iv0: Iv0,
    iv1: Iv1,
    iv2: Iv2,
    iv3: Iv3,
    ctrl: Ctrl,
    c_length_0: CLength0,
    c_length_1: CLength1,
    aad_length: AadLength,
    data0: Data0,
    data1: Data1,
    data2: Data2,
    data3: Data3,
    tag0: Tag0,
    tag1: Tag1,
    tag2: Tag2,
    tag3: Tag3,
    status: Status,
    data_in: DataIn,
    data_out: DataOut,
    _reserved41: [u8; 0x44],
    force_in_av: ForceInAv,
    ccm_aln_wrd: CcmAlnWrd,
    blk_cnt0: BlkCnt0,
    blk_cnt1: BlkCnt1,
    _reserved45: [u8; 0x14],
    dma_hs: DmaHs,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
    }
    #[doc = "0x1020..0x104c - INT_EVENT0"]
    #[inline(always)]
    pub const fn int_event0(&self) -> &IntEvent0 {
        &self.int_event0
    }
    #[doc = "0x1050..0x107c - INT_EVENT1"]
    #[inline(always)]
    pub const fn int_event1(&self) -> &IntEvent1 {
        &self.int_event1
    }
    #[doc = "0x1080..0x10ac - INT_EVENT2"]
    #[inline(always)]
    pub const fn int_event2(&self) -> &IntEvent2 {
        &self.int_event2
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x1100 - CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)"]
    #[inline(always)]
    pub const fn gcmccm_tag0(&self) -> &GcmccmTag0 {
        &self.gcmccm_tag0
    }
    #[doc = "0x1104 - CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
    #[inline(always)]
    pub const fn gcmccm_tag1(&self) -> &GcmccmTag1 {
        &self.gcmccm_tag1
    }
    #[doc = "0x1108 - CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
    #[inline(always)]
    pub const fn gcmccm_tag2(&self) -> &GcmccmTag2 {
        &self.gcmccm_tag2
    }
    #[doc = "0x110c - CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)"]
    #[inline(always)]
    pub const fn gcmccm_tag3(&self) -> &GcmccmTag3 {
        &self.gcmccm_tag3
    }
    #[doc = "0x1110 - CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)"]
    #[inline(always)]
    pub const fn ghash_h0(&self) -> &GhashH0 {
        &self.ghash_h0
    }
    #[doc = "0x1114 - CCM &amp; CBC-MAC second key / GCM Hash Key input"]
    #[inline(always)]
    pub const fn ghash_h1(&self) -> &GhashH1 {
        &self.ghash_h1
    }
    #[doc = "0x1118 - CCM &amp; CBC-MAC second key / GCM Hash Key input"]
    #[inline(always)]
    pub const fn ghash_h2(&self) -> &GhashH2 {
        &self.ghash_h2
    }
    #[doc = "0x111c - CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)"]
    #[inline(always)]
    pub const fn ghash_h3(&self) -> &GhashH3 {
        &self.ghash_h3
    }
    #[doc = "0x1120 - KEY (LSW)"]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x1124 - KEY"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x1128 - KEY"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x112c - KEY"]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x1130 - KEY"]
    #[inline(always)]
    pub const fn key4(&self) -> &Key4 {
        &self.key4
    }
    #[doc = "0x1134 - KEY"]
    #[inline(always)]
    pub const fn key5(&self) -> &Key5 {
        &self.key5
    }
    #[doc = "0x1138 - KEY"]
    #[inline(always)]
    pub const fn key6(&self) -> &Key6 {
        &self.key6
    }
    #[doc = "0x113c - KEY (MSW)"]
    #[inline(always)]
    pub const fn key7(&self) -> &Key7 {
        &self.key7
    }
    #[doc = "0x1140 - IV (LSW)"]
    #[inline(always)]
    pub const fn iv0(&self) -> &Iv0 {
        &self.iv0
    }
    #[doc = "0x1144 - IV"]
    #[inline(always)]
    pub const fn iv1(&self) -> &Iv1 {
        &self.iv1
    }
    #[doc = "0x1148 - IV"]
    #[inline(always)]
    pub const fn iv2(&self) -> &Iv2 {
        &self.iv2
    }
    #[doc = "0x114c - IV"]
    #[inline(always)]
    pub const fn iv3(&self) -> &Iv3 {
        &self.iv3
    }
    #[doc = "0x1150 - Input/Output Buffer Control and Mode selection"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x1154 - Crypto data length (LSW)"]
    #[inline(always)]
    pub const fn c_length_0(&self) -> &CLength0 {
        &self.c_length_0
    }
    #[doc = "0x1158 - Crypto data length (MSW)"]
    #[inline(always)]
    pub const fn c_length_1(&self) -> &CLength1 {
        &self.c_length_1
    }
    #[doc = "0x115c - AAD Data Length"]
    #[inline(always)]
    pub const fn aad_length(&self) -> &AadLength {
        &self.aad_length
    }
    #[doc = "0x1160 - Data input (LSW) / Data output (LSW)"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x1164 - Data input / Data output"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x1168 - Data input / Data output"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x116c - Data input (LSW) / Data output (MSW)"]
    #[inline(always)]
    pub const fn data3(&self) -> &Data3 {
        &self.data3
    }
    #[doc = "0x1170 - Hash result (LSW)"]
    #[inline(always)]
    pub const fn tag0(&self) -> &Tag0 {
        &self.tag0
    }
    #[doc = "0x1174 - Hash result"]
    #[inline(always)]
    pub const fn tag1(&self) -> &Tag1 {
        &self.tag1
    }
    #[doc = "0x1178 - Hash result"]
    #[inline(always)]
    pub const fn tag2(&self) -> &Tag2 {
        &self.tag2
    }
    #[doc = "0x117c - Hash result (MSW)"]
    #[inline(always)]
    pub const fn tag3(&self) -> &Tag3 {
        &self.tag3
    }
    #[doc = "0x1180 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1184 - Data in alias register"]
    #[inline(always)]
    pub const fn data_in(&self) -> &DataIn {
        &self.data_in
    }
    #[doc = "0x1188 - Data out alias register"]
    #[inline(always)]
    pub const fn data_out(&self) -> &DataOut {
        &self.data_out
    }
    #[doc = "0x11d0 - Data control register for input data"]
    #[inline(always)]
    pub const fn force_in_av(&self) -> &ForceInAv {
        &self.force_in_av
    }
    #[doc = "0x11d4 - AES-CCM AAD alignment data word"]
    #[inline(always)]
    pub const fn ccm_aln_wrd(&self) -> &CcmAlnWrd {
        &self.ccm_aln_wrd
    }
    #[doc = "0x11d8 - Internal block counter (LSW)"]
    #[inline(always)]
    pub const fn blk_cnt0(&self) -> &BlkCnt0 {
        &self.blk_cnt0
    }
    #[doc = "0x11dc - Internal block counter (MSW)"]
    #[inline(always)]
    pub const fn blk_cnt1(&self) -> &BlkCnt1 {
        &self.blk_cnt1
    }
    #[doc = "0x11f4 - Control register for DMA handshaking"]
    #[inline(always)]
    pub const fn dma_hs(&self) -> &DmaHs {
        &self.dma_hs
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "PDBGCTL (r) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`] module"]
#[doc(alias = "PDBGCTL")]
pub type Pdbgctl = crate::Reg<pdbgctl::PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod pdbgctl;
#[doc = "INT_EVENT0"]
pub use self::int_event0::IntEvent0;
#[doc = r"Cluster"]
#[doc = "INT_EVENT0"]
pub mod int_event0;
#[doc = "INT_EVENT1"]
pub use self::int_event1::IntEvent1;
#[doc = r"Cluster"]
#[doc = "INT_EVENT1"]
pub mod int_event1;
#[doc = "INT_EVENT2"]
pub use self::int_event2::IntEvent2;
#[doc = r"Cluster"]
#[doc = "INT_EVENT2"]
pub mod int_event2;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`] module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "GCMCCM_TAG0 (w) register accessor: CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmccm_tag0`] module"]
#[doc(alias = "GCMCCM_TAG0")]
pub type GcmccmTag0 = crate::Reg<gcmccm_tag0::GcmccmTag0Spec>;
#[doc = "CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)"]
pub mod gcmccm_tag0;
#[doc = "GCMCCM_TAG1 (w) register accessor: CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmccm_tag1`] module"]
#[doc(alias = "GCMCCM_TAG1")]
pub type GcmccmTag1 = crate::Reg<gcmccm_tag1::GcmccmTag1Spec>;
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
pub mod gcmccm_tag1;
#[doc = "GCMCCM_TAG2 (w) register accessor: CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmccm_tag2`] module"]
#[doc(alias = "GCMCCM_TAG2")]
pub type GcmccmTag2 = crate::Reg<gcmccm_tag2::GcmccmTag2Spec>;
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
pub mod gcmccm_tag2;
#[doc = "GCMCCM_TAG3 (w) register accessor: CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmccm_tag3`] module"]
#[doc(alias = "GCMCCM_TAG3")]
pub type GcmccmTag3 = crate::Reg<gcmccm_tag3::GcmccmTag3Spec>;
#[doc = "CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)"]
pub mod gcmccm_tag3;
#[doc = "GHASH_H0 (w) register accessor: CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash_h0`] module"]
#[doc(alias = "GHASH_H0")]
pub type GhashH0 = crate::Reg<ghash_h0::GhashH0Spec>;
#[doc = "CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)"]
pub mod ghash_h0;
#[doc = "GHASH_H1 (w) register accessor: CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash_h1`] module"]
#[doc(alias = "GHASH_H1")]
pub type GhashH1 = crate::Reg<ghash_h1::GhashH1Spec>;
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input"]
pub mod ghash_h1;
#[doc = "GHASH_H2 (w) register accessor: CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash_h2`] module"]
#[doc(alias = "GHASH_H2")]
pub type GhashH2 = crate::Reg<ghash_h2::GhashH2Spec>;
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input"]
pub mod ghash_h2;
#[doc = "GHASH_H3 (w) register accessor: CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash_h3`] module"]
#[doc(alias = "GHASH_H3")]
pub type GhashH3 = crate::Reg<ghash_h3::GhashH3Spec>;
#[doc = "CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)"]
pub mod ghash_h3;
#[doc = "KEY0 (w) register accessor: KEY (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`] module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "KEY (LSW)"]
pub mod key0;
#[doc = "KEY1 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "KEY"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "KEY"]
pub mod key2;
#[doc = "KEY3 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`] module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "KEY"]
pub mod key3;
#[doc = "KEY4 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key4`] module"]
#[doc(alias = "KEY4")]
pub type Key4 = crate::Reg<key4::Key4Spec>;
#[doc = "KEY"]
pub mod key4;
#[doc = "KEY5 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key5`] module"]
#[doc(alias = "KEY5")]
pub type Key5 = crate::Reg<key5::Key5Spec>;
#[doc = "KEY"]
pub mod key5;
#[doc = "KEY6 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key6`] module"]
#[doc(alias = "KEY6")]
pub type Key6 = crate::Reg<key6::Key6Spec>;
#[doc = "KEY"]
pub mod key6;
#[doc = "KEY7 (w) register accessor: KEY (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key7`] module"]
#[doc(alias = "KEY7")]
pub type Key7 = crate::Reg<key7::Key7Spec>;
#[doc = "KEY (MSW)"]
pub mod key7;
#[doc = "IV0 (rw) register accessor: IV (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`iv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv0`] module"]
#[doc(alias = "IV0")]
pub type Iv0 = crate::Reg<iv0::Iv0Spec>;
#[doc = "IV (LSW)"]
pub mod iv0;
#[doc = "IV1 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv1`] module"]
#[doc(alias = "IV1")]
pub type Iv1 = crate::Reg<iv1::Iv1Spec>;
#[doc = "IV"]
pub mod iv1;
#[doc = "IV2 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv2`] module"]
#[doc(alias = "IV2")]
pub type Iv2 = crate::Reg<iv2::Iv2Spec>;
#[doc = "IV"]
pub mod iv2;
#[doc = "IV3 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv3`] module"]
#[doc(alias = "IV3")]
pub type Iv3 = crate::Reg<iv3::Iv3Spec>;
#[doc = "IV"]
pub mod iv3;
#[doc = "CTRL (rw) register accessor: Input/Output Buffer Control and Mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Input/Output Buffer Control and Mode selection"]
pub mod ctrl;
#[doc = "C_LENGTH_0 (w) register accessor: Crypto data length (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c_length_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_length_0`] module"]
#[doc(alias = "C_LENGTH_0")]
pub type CLength0 = crate::Reg<c_length_0::CLength0Spec>;
#[doc = "Crypto data length (LSW)"]
pub mod c_length_0;
#[doc = "C_LENGTH_1 (w) register accessor: Crypto data length (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c_length_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_length_1`] module"]
#[doc(alias = "C_LENGTH_1")]
pub type CLength1 = crate::Reg<c_length_1::CLength1Spec>;
#[doc = "Crypto data length (MSW)"]
pub mod c_length_1;
#[doc = "AAD_LENGTH (w) register accessor: AAD Data Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_length::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_length`] module"]
#[doc(alias = "AAD_LENGTH")]
pub type AadLength = crate::Reg<aad_length::AadLengthSpec>;
#[doc = "AAD Data Length"]
pub mod aad_length;
#[doc = "DATA0 (rw) register accessor: Data input (LSW) / Data output (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "Data input (LSW) / Data output (LSW)"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "Data input / Data output"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "Data input / Data output"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: Data input (LSW) / Data output (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`] module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "Data input (LSW) / Data output (MSW)"]
pub mod data3;
#[doc = "TAG0 (r) register accessor: Hash result (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag0`] module"]
#[doc(alias = "TAG0")]
pub type Tag0 = crate::Reg<tag0::Tag0Spec>;
#[doc = "Hash result (LSW)"]
pub mod tag0;
#[doc = "TAG1 (r) register accessor: Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`tag1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag1`] module"]
#[doc(alias = "TAG1")]
pub type Tag1 = crate::Reg<tag1::Tag1Spec>;
#[doc = "Hash result"]
pub mod tag1;
#[doc = "TAG2 (r) register accessor: Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`tag2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag2`] module"]
#[doc(alias = "TAG2")]
pub type Tag2 = crate::Reg<tag2::Tag2Spec>;
#[doc = "Hash result"]
pub mod tag2;
#[doc = "TAG3 (r) register accessor: Hash result (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tag3`] module"]
#[doc(alias = "TAG3")]
pub type Tag3 = crate::Reg<tag3::Tag3Spec>;
#[doc = "Hash result (MSW)"]
pub mod tag3;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "DATA_IN (w) register accessor: Data in alias register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in`] module"]
#[doc(alias = "DATA_IN")]
pub type DataIn = crate::Reg<data_in::DataInSpec>;
#[doc = "Data in alias register"]
pub mod data_in;
#[doc = "DATA_OUT (r) register accessor: Data out alias register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_out`] module"]
#[doc(alias = "DATA_OUT")]
pub type DataOut = crate::Reg<data_out::DataOutSpec>;
#[doc = "Data out alias register"]
pub mod data_out;
#[doc = "FORCE_IN_AV (w) register accessor: Data control register for input data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_in_av::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_in_av`] module"]
#[doc(alias = "FORCE_IN_AV")]
pub type ForceInAv = crate::Reg<force_in_av::ForceInAvSpec>;
#[doc = "Data control register for input data"]
pub mod force_in_av;
#[doc = "CCM_ALN_WRD (rw) register accessor: AES-CCM AAD alignment data word\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_aln_wrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_aln_wrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm_aln_wrd`] module"]
#[doc(alias = "CCM_ALN_WRD")]
pub type CcmAlnWrd = crate::Reg<ccm_aln_wrd::CcmAlnWrdSpec>;
#[doc = "AES-CCM AAD alignment data word"]
pub mod ccm_aln_wrd;
#[doc = "BLK_CNT0 (rw) register accessor: Internal block counter (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_cnt0`] module"]
#[doc(alias = "BLK_CNT0")]
pub type BlkCnt0 = crate::Reg<blk_cnt0::BlkCnt0Spec>;
#[doc = "Internal block counter (LSW)"]
pub mod blk_cnt0;
#[doc = "BLK_CNT1 (rw) register accessor: Internal block counter (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_cnt1`] module"]
#[doc(alias = "BLK_CNT1")]
pub type BlkCnt1 = crate::Reg<blk_cnt1::BlkCnt1Spec>;
#[doc = "Internal block counter (MSW)"]
pub mod blk_cnt1;
#[doc = "DMA_HS (rw) register accessor: Control register for DMA handshaking\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_hs`] module"]
#[doc(alias = "DMA_HS")]
pub type DmaHs = crate::Reg<dma_hs::DmaHsSpec>;
#[doc = "Control register for DMA handshaking"]
pub mod dma_hs;
