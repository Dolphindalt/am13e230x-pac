#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `PER` reader - Parity error event"]
pub type PerR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - RXFIFO overflow event"]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `RX` reader - Receive event"]
pub type RxR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO Full Interrupt"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `TXFIFO_UNF` reader - TX FIFO Underflow Interrupt"]
pub type TxfifoUnfR = crate::BitReader;
#[doc = "Field `TX` reader - Transmit Event"]
pub type TxR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty interrupt mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `IDLE` reader - SPI in Idle mode"]
pub type IdleR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` reader - DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` reader - DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error event"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFIFO overflow event"]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive event"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Underflow Interrupt"]
    #[inline(always)]
    pub fn txfifo_unf(&self) -> TxfifoUnfR {
        TxfifoUnfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Event"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI in Idle mode"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("rx", &self.rx())
            .field("tx", &self.tx())
            .field("txempty", &self.txempty())
            .field("per", &self.per())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("idle", &self.idle())
            .field("txfifo_unf", &self.txfifo_unf())
            .field("rxfull", &self.rxfull())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("dma_done_tx", &self.dma_done_tx())
            .finish()
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
