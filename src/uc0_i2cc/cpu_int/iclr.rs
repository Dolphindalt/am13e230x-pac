#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `RXDONE` writer - Receive Data Interrupt Signals that a byte has been received"]
pub type RxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` writer - Transmit Transaction completed Interrupt"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRG` writer - Receive FIFO Trigger"]
pub type RxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRG` writer - Transmit FIFO Trigger"]
pub type TxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - RXFIFO full event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit FIFO Empty interrupt mask"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Address/Data NACK Interrupt"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` writer - START Detection Interrupt"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - STOP Detection Interrupt"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC_RX_ERR` writer - RX Pec Error Interrupt"]
pub type PecRxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTA` writer - Timeout A interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` writer - DMA Done on Event Channel RX"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` writer - DMA Done on Event Channel TX"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn rxdone(&mut self) -> RxdoneW<'_, IclrSpec> {
        RxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TxdoneW<'_, IclrSpec> {
        TxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrg(&mut self) -> RxtrgW<'_, IclrSpec> {
        RxtrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn txtrg(&mut self) -> TxtrgW<'_, IclrSpec> {
        TxtrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IclrSpec> {
        RxfullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, IclrSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IclrSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IclrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, IclrSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IclrSpec> {
        ArblostW::new(self, 10)
    }
    #[doc = "Bit 11 - RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&mut self) -> PecRxErrW<'_, IclrSpec> {
        PecRxErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout A interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, IclrSpec> {
        TimeoutaW::new(self, 12)
    }
    #[doc = "Bit 13 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, IclrSpec> {
        TimeoutbW::new(self, 13)
    }
    #[doc = "Bit 15 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, IclrSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, IclrSpec> {
        DmaDoneTxW::new(self, 16)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
