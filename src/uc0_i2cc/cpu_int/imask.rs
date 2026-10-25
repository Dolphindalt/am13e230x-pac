#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `RXDONE` reader - RXDONE interrupt"]
pub type RxdoneR = crate::BitReader;
#[doc = "Field `RXDONE` writer - RXDONE interrupt"]
pub type RxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` reader - Transmit Transaction completed Interrupt"]
pub type TxdoneR = crate::BitReader;
#[doc = "Field `TXDONE` writer - Transmit Transaction completed Interrupt"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRG` reader - Receive FIFO Trigger"]
pub type RxtrgR = crate::BitReader;
#[doc = "Field `RXTRG` writer - Receive FIFO Trigger"]
pub type RxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRG` reader - Transmit Trigger"]
pub type TxtrgR = crate::BitReader;
#[doc = "Field `TXTRG` writer - Transmit Trigger"]
pub type TxtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RXFIFO full event"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RXFIFO full event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` reader - Interrupt Mask for Transmit FIFO Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXEMPTY` writer - Interrupt Mask for Transmit FIFO Empty"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK interrupt"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - NACK interrupt"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - START interrupt"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START interrupt"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP interrupt"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - STOP interrupt"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost interrupt"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost interrupt"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC_RX_ERR` reader - RX Pec Error Interrupt"]
pub type PecRxErrR = crate::BitReader;
#[doc = "Field `PEC_RX_ERR` writer - RX Pec Error Interrupt"]
pub type PecRxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTA` reader - TIMEOUTA interrupt"]
pub type TimeoutaR = crate::BitReader;
#[doc = "Field `TIMEOUTA` writer - TIMEOUTA interrupt"]
pub type TimeoutaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - Timeout B Interrupt"]
pub type TimeoutbR = crate::BitReader;
#[doc = "Field `TIMEOUTB` writer - Timeout B Interrupt"]
pub type TimeoutbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` reader - DMA Done on Event Channel RX"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` writer - DMA Done on Event Channel RX"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` reader - DMA Done on Event Channel TX"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` writer - DMA Done on Event Channel TX"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXDONE interrupt"]
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
    #[doc = "Bit 3 - Transmit Trigger"]
    #[inline(always)]
    pub fn txtrg(&self) -> TxtrgR {
        TxtrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask for Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - NACK interrupt"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - START interrupt"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP interrupt"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost interrupt"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&self) -> PecRxErrR {
        PecRxErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEOUTA interrupt"]
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
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("rxdone", &self.rxdone())
            .field("timeouta", &self.timeouta())
            .field("nack", &self.nack())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("arblost", &self.arblost())
            .field("txdone", &self.txdone())
            .field("rxtrg", &self.rxtrg())
            .field("txtrg", &self.txtrg())
            .field("dma_done_tx", &self.dma_done_tx())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("pec_rx_err", &self.pec_rx_err())
            .field("timeoutb", &self.timeoutb())
            .field("rxfull", &self.rxfull())
            .field("txempty", &self.txempty())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXDONE interrupt"]
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
    #[doc = "Bit 3 - Transmit Trigger"]
    #[inline(always)]
    pub fn txtrg(&mut self) -> TxtrgW<'_, ImaskSpec> {
        TxtrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, ImaskSpec> {
        RxfullW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask for Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, ImaskSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - NACK interrupt"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, ImaskSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - START interrupt"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, ImaskSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP interrupt"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, ImaskSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost interrupt"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, ImaskSpec> {
        ArblostW::new(self, 10)
    }
    #[doc = "Bit 11 - RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn pec_rx_err(&mut self) -> PecRxErrW<'_, ImaskSpec> {
        PecRxErrW::new(self, 11)
    }
    #[doc = "Bit 12 - TIMEOUTA interrupt"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, ImaskSpec> {
        TimeoutaW::new(self, 12)
    }
    #[doc = "Bit 13 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, ImaskSpec> {
        TimeoutbW::new(self, 13)
    }
    #[doc = "Bit 15 - DMA Done on Event Channel RX"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, ImaskSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMA Done on Event Channel TX"]
    #[inline(always)]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<'_, ImaskSpec> {
        DmaDoneTxW::new(self, 16)
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
