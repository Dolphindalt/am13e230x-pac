#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `RXDONE` writer - Target Receive Data Interrupt Signals that a byte has been received"]
pub type RxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` writer - Target Transmit Transaction completed Interrupt"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRG` writer - Target Receive FIFO Trigger"]
pub type RxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRG` writer - Target Transmit FIFO Trigger"]
pub type TxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - RXFIFO full event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit FIFO Empty interrupt mask"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_UNFL` writer - Target TX FIFO underflow"]
pub type TxUnflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVFL` writer - Target RX FIFO overflow"]
pub type RxOvflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALL` writer - General Call Interrupt"]
pub type GencallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` writer - Start Condition Interrupt"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Stop Condition Interrupt"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC_RX_ERR` writer - Target RX Pec Error Interrupt"]
pub type PecRxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTA` writer - Timeout A interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` writer - DMA Done on Event Channel RX"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` writer - DMA Done on Event Channel TX"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Target Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn rxdone(&mut self) -> RxdoneW<'_, IsetSpec> {
        RxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Target Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TxdoneW<'_, IsetSpec> {
        TxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrg(&mut self) -> RxtrgW<'_, IsetSpec> {
        RxtrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn txtrg(&mut self) -> TxtrgW<'_, IsetSpec> {
        TxtrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IsetSpec> {
        RxfullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, IsetSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn tx_unfl(&mut self) -> TxUnflW<'_, IsetSpec> {
        TxUnflW::new(self, 6)
    }
    #[doc = "Bit 7 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn rx_ovfl(&mut self) -> RxOvflW<'_, IsetSpec> {
        RxOvflW::new(self, 7)
    }
    #[doc = "Bit 8 - General Call Interrupt"]
    #[inline(always)]
    pub fn gencall(&mut self) -> GencallW<'_, IsetSpec> {
        GencallW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IsetSpec> {
        StartW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, IsetSpec> {
        StopW::new(self, 10)
    }
    #[doc = "Bit 11 - Target RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&mut self) -> PecRxErrW<'_, IsetSpec> {
        PecRxErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout A interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, IsetSpec> {
        TimeoutaW::new(self, 12)
    }
    #[doc = "Bit 13 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, IsetSpec> {
        TimeoutbW::new(self, 13)
    }
    #[doc = "Bit 15 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, IsetSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, IsetSpec> {
        DmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IsetSpec> {
        ArblostW::new(self, 17)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {}
