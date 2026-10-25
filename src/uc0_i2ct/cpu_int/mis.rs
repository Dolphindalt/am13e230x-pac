#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RXDONE` reader - Receive Data Interrupt Signals that a byte has been received"]
pub type RxdoneR = crate::BitReader;
#[doc = "Field `TXDONE` reader - Transmit Transaction completed Interrupt"]
pub type TxdoneR = crate::BitReader;
#[doc = "Field `RXTRG` reader - Receive FIFO Trigger"]
pub type RxtrgR = crate::BitReader;
#[doc = "Field `TXTRG` reader - Transmit FIFO Trigger"]
pub type TxtrgR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RXFIFO full event"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty interrupt mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TX_UNFL` reader - Target TX FIFO underflow"]
pub type TxUnflR = crate::BitReader;
#[doc = "Field `RX_OVFL` reader - Target RX FIFO overflow"]
pub type RxOvflR = crate::BitReader;
#[doc = "Field `GENCALL` reader - General Call Interrupt"]
pub type GencallR = crate::BitReader;
#[doc = "Field `START` reader - START Detection Interrupt"]
pub type StartR = crate::BitReader;
#[doc = "Field `STOP` reader - STOP Detection Interrupt"]
pub type StopR = crate::BitReader;
#[doc = "Field `PEC_RX_ERR` reader - Target RX Pec Error Interrupt"]
pub type PecRxErrR = crate::BitReader;
#[doc = "Field `TIMEOUTA` reader - Timeout A Interrupt"]
pub type TimeoutaR = crate::BitReader;
#[doc = "Field `TIMEOUTB` reader - Timeout B Interrupt"]
pub type TimeoutbR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` reader - DMA Done on Event Channel RX"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` reader - DMA Done on Event Channel TX"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt"]
pub type ArblostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn rxdone(&self) -> RxdoneR {
        RxdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn txdone(&self) -> TxdoneR {
        TxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrg(&self) -> RxtrgR {
        RxtrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn txtrg(&self) -> TxtrgR {
        TxtrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn tx_unfl(&self) -> TxUnflR {
        TxUnflR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn rx_ovfl(&self) -> RxOvflR {
        RxOvflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - General Call Interrupt"]
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        GencallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - START Detection Interrupt"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Target RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&self) -> PecRxErrR {
        PecRxErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn timeouta(&self) -> TimeoutaR {
        TimeoutaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TimeoutbR {
        TimeoutbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("rxdone", &self.rxdone())
            .field("txdone", &self.txdone())
            .field("gencall", &self.gencall())
            .field("rxtrg", &self.rxtrg())
            .field("txtrg", &self.txtrg())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("dma_done_tx", &self.dma_done_tx())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("pec_rx_err", &self.pec_rx_err())
            .field("timeouta", &self.timeouta())
            .field("timeoutb", &self.timeoutb())
            .field("rxfull", &self.rxfull())
            .field("txempty", &self.txempty())
            .field("tx_unfl", &self.tx_unfl())
            .field("rx_ovfl", &self.rx_ovfl())
            .field("arblost", &self.arblost())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
