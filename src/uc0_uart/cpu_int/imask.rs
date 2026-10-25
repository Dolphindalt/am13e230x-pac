#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `RTOUT` reader - Enable UARTOUT Receive Time-Out Interrupt."]
pub type RtoutR = crate::BitReader;
#[doc = "Field `RTOUT` writer - Enable UARTOUT Receive Time-Out Interrupt."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMERR` reader - Enable UART Framing Error Interrupt."]
pub type FrmerrR = crate::BitReader;
#[doc = "Field `FRMERR` writer - Enable UART Framing Error Interrupt."]
pub type FrmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERR` reader - Enable UART Parity Error Interrupt."]
pub type ParerrR = crate::BitReader;
#[doc = "Field `PARERR` writer - Enable UART Parity Error Interrupt."]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKERR` reader - Enable UART Break Error Interrupt."]
pub type BrkerrR = crate::BitReader;
#[doc = "Field `BRKERR` writer - Enable UART Break Error Interrupt."]
pub type BrkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRERR` reader - Enable UART Receive Overrun Error Interrupt."]
pub type OvrerrR = crate::BitReader;
#[doc = "Field `OVRERR` writer - Enable UART Receive Overrun Error Interrupt."]
pub type OvrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - Enable Negative Edge on UARTxRXD Interrupt."]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - Enable Negative Edge on UARTxRXD Interrupt."]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPE` reader - Enable Positive Edge on UARTxRXD Interrupt."]
pub type RxpeR = crate::BitReader;
#[doc = "Field `RXPE` writer - Enable Positive Edge on UARTxRXD Interrupt."]
pub type RxpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINC0` reader - Enable LIN Capture 0 / Match Interrupt ."]
pub type Linc0R = crate::BitReader;
#[doc = "Field `LINC0` writer - Enable LIN Capture 0 / Match Interrupt ."]
pub type Linc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINC1` reader - Enable LIN Capture 1 Interrupt."]
pub type Linc1R = crate::BitReader;
#[doc = "Field `LINC1` writer - Enable LIN Capture 1 Interrupt."]
pub type Linc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINOVF` reader - Enable LIN Hardware Counter Overflow Interrupt."]
pub type LinovfR = crate::BitReader;
#[doc = "Field `LINOVF` writer - Enable LIN Hardware Counter Overflow Interrupt."]
pub type LinovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINT` reader - Enable UART Receive Interrupt."]
pub type RxintR = crate::BitReader;
#[doc = "Field `RXINT` writer - Enable UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINT` reader - Enable UART Transmit Interrupt."]
pub type TxintR = crate::BitReader;
#[doc = "Field `TXINT` writer - Enable UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` reader - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_MATCH` reader - Enable Address Match Interrupt."]
pub type AddrMatchR = crate::BitReader;
#[doc = "Field `ADDR_MATCH` writer - Enable Address Match Interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - Enable UART Clear to Send Modem Interrupt."]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - Enable UART Clear to Send Modem Interrupt."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` reader - Enable DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` writer - Enable DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` reader - Enable DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` writer - Enable DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrR = crate::BitReader;
#[doc = "Field `NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTOUT` reader - Enable UARTOUT Line Time-Out Interrupt."]
pub type LtoutR = crate::BitReader;
#[doc = "Field `LTOUT` writer - Enable UARTOUT Line Time-Out Interrupt."]
pub type LtoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&self) -> BrkerrR {
        BrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&self) -> OvrerrR {
        OvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&self) -> RxpeR {
        RxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&self) -> Linc0R {
        Linc0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&self) -> Linc1R {
        Linc1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&self) -> LinovfR {
        LinovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable UARTOUT Line Time-Out Interrupt."]
    #[inline(always)]
    pub fn ltout(&self) -> LtoutR {
        LtoutR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("frmerr", &self.frmerr())
            .field("parerr", &self.parerr())
            .field("brkerr", &self.brkerr())
            .field("ovrerr", &self.ovrerr())
            .field("rxne", &self.rxne())
            .field("rxpe", &self.rxpe())
            .field("linovf", &self.linovf())
            .field("rxint", &self.rxint())
            .field("txint", &self.txint())
            .field("eot", &self.eot())
            .field("addr_match", &self.addr_match())
            .field("cts", &self.cts())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("dma_done_tx", &self.dma_done_tx())
            .field("rtout", &self.rtout())
            .field("linc0", &self.linc0())
            .field("linc1", &self.linc1())
            .field("nerr", &self.nerr())
            .field("ltout", &self.ltout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, ImaskSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FrmerrW<'_, ImaskSpec> {
        FrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<'_, ImaskSpec> {
        ParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&mut self) -> BrkerrW<'_, ImaskSpec> {
        BrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&mut self) -> OvrerrW<'_, ImaskSpec> {
        OvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, ImaskSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&mut self) -> RxpeW<'_, ImaskSpec> {
        RxpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&mut self) -> Linc0W<'_, ImaskSpec> {
        Linc0W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&mut self) -> Linc1W<'_, ImaskSpec> {
        Linc1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&mut self) -> LinovfW<'_, ImaskSpec> {
        LinovfW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, ImaskSpec> {
        RxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, ImaskSpec> {
        TxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<'_, ImaskSpec> {
        EotW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, ImaskSpec> {
        AddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, ImaskSpec> {
        CtsW::new(self, 14)
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
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&mut self) -> NerrW<'_, ImaskSpec> {
        NerrW::new(self, 17)
    }
    #[doc = "Bit 20 - Enable UARTOUT Line Time-Out Interrupt."]
    #[inline(always)]
    pub fn ltout(&mut self) -> LtoutW<'_, ImaskSpec> {
        LtoutW::new(self, 20)
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
