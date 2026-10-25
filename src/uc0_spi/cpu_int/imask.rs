#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `PER` reader - Parity error event"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Parity error event"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` reader - RXFIFO overflow event"]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` writer - RXFIFO overflow event"]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` reader - Receive event. If FIFO is present, this bit is set when the programmed threshold is reached. When there is no FIFO, this bit is set when a frame is received"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Receive event. If FIFO is present, this bit is set when the programmed threshold is reached. When there is no FIFO, this bit is set when a frame is received"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO Full Interrupt Mask"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO Full Interrupt Mask"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_UNF` reader - TX FIFO underflow interrupt mask"]
pub type TxfifoUnfR = crate::BitReader;
#[doc = "Field `TXFIFO_UNF` writer - TX FIFO underflow interrupt mask"]
pub type TxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Transmit event. When FIFO is present, this bit is set when FIFO threshold is reached. When FIFO is not used, this bit is set when transmit buffer is empty."]
pub type TxR = crate::BitReader;
#[doc = "Field `TX` writer - Transmit event. When FIFO is present, this bit is set when FIFO threshold is reached. When FIFO is not used, this bit is set when transmit buffer is empty."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty interrupt mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXEMPTY` writer - Transmit FIFO Empty interrupt mask"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - SPI in Idle mode"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - SPI in Idle mode"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` reader - Enable DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` writer - Enable DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` reader - Enable DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` writer - Enable DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Receive event. If FIFO is present, this bit is set when the programmed threshold is reached. When there is no FIFO, this bit is set when a frame is received"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn txfifo_unf(&self) -> TxfifoUnfR {
        TxfifoUnfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit event. When FIFO is present, this bit is set when FIFO threshold is reached. When FIFO is not used, this bit is set when transmit buffer is empty."]
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
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("rx", &self.rx())
            .field("tx", &self.tx())
            .field("txempty", &self.txempty())
            .field("per", &self.per())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("idle", &self.idle())
            .field("rxfull", &self.rxfull())
            .field("txfifo_unf", &self.txfifo_unf())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("dma_done_tx", &self.dma_done_tx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parity error event"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, ImaskSpec> {
        PerW::new(self, 0)
    }
    #[doc = "Bit 1 - RXFIFO overflow event"]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, ImaskSpec> {
        RxfifoOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive event. If FIFO is present, this bit is set when the programmed threshold is reached. When there is no FIFO, this bit is set when a frame is received"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, ImaskSpec> {
        RxW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, ImaskSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn txfifo_unf(&mut self) -> TxfifoUnfW<'_, ImaskSpec> {
        TxfifoUnfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit event. When FIFO is present, this bit is set when FIFO threshold is reached. When FIFO is not used, this bit is set when transmit buffer is empty."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, ImaskSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty interrupt mask"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, ImaskSpec> {
        TxemptyW::new(self, 6)
    }
    #[doc = "Bit 8 - SPI in Idle mode"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, ImaskSpec> {
        IdleW::new(self, 8)
    }
    #[doc = "Bit 15 - Enable DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<'_, ImaskSpec> {
        DmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable DMA Done on TX Event Channel Interrupt"]
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
