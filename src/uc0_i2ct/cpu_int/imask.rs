#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `RXDONE` reader - Receive Data Interrupt Signals that a byte has been received"]
pub type RxdoneR = crate::BitReader;
#[doc = "Field `RXDONE` writer - Receive Data Interrupt Signals that a byte has been received"]
pub type RxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` reader - Transmit Transaction completed Interrupt"]
pub type TxdoneR = crate::BitReader;
#[doc = "Field `TXDONE` writer - Transmit Transaction completed Interrupt"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRG` reader - Receive FIFO Trigger"]
pub type RxtrgR = crate::BitReader;
#[doc = "Field `RXTRG` writer - Receive FIFO Trigger"]
pub type RxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRG` reader - Transmit FIFO Trigger"]
pub type TxtrgR = crate::BitReader;
#[doc = "Field `TXTRG` writer - Transmit FIFO Trigger"]
pub type TxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RXFIFO full event"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RXFIFO full event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` reader - Target Transmit FIFO Empty interrupt mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXEMPTY` writer - Target Transmit FIFO Empty interrupt mask"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_UNFL` reader - Target TX FIFO underflow"]
pub type TxUnflR = crate::BitReader;
#[doc = "Field `TX_UNFL` writer - Target TX FIFO underflow"]
pub type TxUnflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVFL` reader - Target RX FIFO overflow"]
pub type RxOvflR = crate::BitReader;
#[doc = "Field `RX_OVFL` writer - Target RX FIFO overflow"]
pub type RxOvflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALL` reader - General Call Interrupt"]
pub type GencallR = crate::BitReader;
#[doc = "Field `GENCALL` writer - General Call Interrupt"]
pub type GencallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC_RX_ERR` reader - RX Pec Error Interrupt"]
pub type PecRxErrR = crate::BitReader;
#[doc = "Field `PEC_RX_ERR` writer - RX Pec Error Interrupt"]
pub type PecRxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTA` reader - Timeout A Interrupt"]
pub type TimeoutaR = crate::BitReader;
#[doc = "Field `TIMEOUTA` writer - Timeout A Interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - Timeout B Interrupt"]
pub type TimeoutbR = crate::BitReader;
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` reader - DMA done for channel 3"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` writer - DMA done for channel 3"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` reader - DMA done for channel 2"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` writer - DMA done for channel 2"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - Target Transmit FIFO Empty interrupt mask"]
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
    #[doc = "Bit 9 - START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Pec Error Interrupt"]
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
    #[doc = "Bit 15 - DMA done for channel 3"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA done for channel 2"]
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
        f.debug_struct("IMASK")
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
impl W {
    #[doc = "Bit 0 - Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn rxdone(&mut self) -> RxdoneW<'_, ImaskSpec> {
        RxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TxdoneW<'_, ImaskSpec> {
        TxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrg(&mut self) -> RxtrgW<'_, ImaskSpec> {
        RxtrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn txtrg(&mut self) -> TxtrgW<'_, ImaskSpec> {
        TxtrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, ImaskSpec> {
        RxfullW::new(self, 4)
    }
    #[doc = "Bit 5 - Target Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, ImaskSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Target TX FIFO underflow"]
    #[inline(always)]
    pub fn tx_unfl(&mut self) -> TxUnflW<'_, ImaskSpec> {
        TxUnflW::new(self, 6)
    }
    #[doc = "Bit 7 - Target RX FIFO overflow"]
    #[inline(always)]
    pub fn rx_ovfl(&mut self) -> RxOvflW<'_, ImaskSpec> {
        RxOvflW::new(self, 7)
    }
    #[doc = "Bit 8 - General Call Interrupt"]
    #[inline(always)]
    pub fn gencall(&mut self) -> GencallW<'_, ImaskSpec> {
        GencallW::new(self, 8)
    }
    #[doc = "Bit 9 - START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, ImaskSpec> {
        StartW::new(self, 9)
    }
    #[doc = "Bit 10 - STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, ImaskSpec> {
        StopW::new(self, 10)
    }
    #[doc = "Bit 11 - RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&mut self) -> PecRxErrW<'_, ImaskSpec> {
        PecRxErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, ImaskSpec> {
        TimeoutaW::new(self, 12)
    }
    #[doc = "Bit 13 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, ImaskSpec> {
        TimeoutbW::new(self, 13)
    }
    #[doc = "Bit 15 - DMA done for channel 3"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, ImaskSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMA done for channel 2"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, ImaskSpec> {
        DmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, ImaskSpec> {
        ArblostW::new(self, 17)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
