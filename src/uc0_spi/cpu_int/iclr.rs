#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `PER` writer - Parity error event"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - RXFIFO overflow event"]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` writer - Receive FIFO event"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Clear RX FIFO underflow event"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_UNF` writer - Clear TXFIFO underflow event"]
pub type TxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` writer - Transmit FIFO event"]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit FIFO Empty interrupt mask"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` writer - SPI in Idle mode"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` writer - Clear DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` writer - Clear DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Parity error event"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, IclrSpec> {
        PerW::new(self, 0)
    }
    #[doc = "Bit 1 - RXFIFO overflow event"]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, IclrSpec> {
        RxfifoOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO event"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, IclrSpec> {
        RxW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RX FIFO underflow event"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IclrSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear TXFIFO underflow event"]
    #[inline(always)]
    pub fn txfifo_unf(&mut self) -> TxfifoUnfW<'_, IclrSpec> {
        TxfifoUnfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO event"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, IclrSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, IclrSpec> {
        TxemptyW::new(self, 6)
    }
    #[doc = "Bit 8 - SPI in Idle mode"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, IclrSpec> {
        IdleW::new(self, 8)
    }
    #[doc = "Bit 15 - Clear DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, IclrSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear DMA Done on TX Event Channel Interrupt"]
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
