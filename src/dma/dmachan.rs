#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "DMACHAN\\[%s\\]"]
#[doc(alias = "DMACHAN")]
pub struct Dmachan {
    dmactl: Dmactl,
    dmasa: Dmasa,
    dmada: Dmada,
    dmasz: Dmasz,
}
impl Dmachan {
    #[doc = "0x00 - DMA Channel Control"]
    #[inline(always)]
    pub const fn dmactl(&self) -> &Dmactl {
        &self.dmactl
    }
    #[doc = "0x04 - DMA Channel Source Address"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
    #[doc = "0x08 - DMA Channel Destination Address"]
    #[inline(always)]
    pub const fn dmada(&self) -> &Dmada {
        &self.dmada
    }
    #[doc = "0x0c - DMA Channel Size"]
    #[inline(always)]
    pub const fn dmasz(&self) -> &Dmasz {
        &self.dmasz
    }
}
#[doc = "DMACTL (rw) register accessor: DMA Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`] module"]
#[doc(alias = "DMACTL")]
pub type Dmactl = crate::Reg<dmactl::DmactlSpec>;
#[doc = "DMA Channel Control"]
pub mod dmactl;
#[doc = "DMASA (rw) register accessor: DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`] module"]
#[doc(alias = "DMASA")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Channel Source Address"]
pub mod dmasa;
#[doc = "DMADA (rw) register accessor: DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmada::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmada::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmada`] module"]
#[doc(alias = "DMADA")]
pub type Dmada = crate::Reg<dmada::DmadaSpec>;
#[doc = "DMA Channel Destination Address"]
pub mod dmada;
#[doc = "DMASZ (rw) register accessor: DMA Channel Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasz`] module"]
#[doc(alias = "DMASZ")]
pub type Dmasz = crate::Reg<dmasz::DmaszSpec>;
#[doc = "DMA Channel Size"]
pub mod dmasz;
