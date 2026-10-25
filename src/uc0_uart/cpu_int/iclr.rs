#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMERR` writer - Clear UART Framing Error Interrupt."]
pub type FrmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERR` writer - Clear UART Parity Error Interrupt."]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKERR` writer - Clear UART Break Error Interrupt."]
pub type BrkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRERR` writer - Clear UART Receive Overrun Error Interrupt."]
pub type OvrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` writer - Clear Negative Edge on UARTxRXD Interrupt."]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPE` writer - Clear Positive Edge on UARTxRXD Interrupt."]
pub type RxpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINC0` writer - Clear LIN Capture 0 / Match Interrupt ."]
pub type Linc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINC1` writer - Clear LIN Capture 1 Interrupt."]
pub type Linc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINOVF` writer - Clear LIN Hardware Counter Overflow Interrupt."]
pub type LinovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINT` writer - Clear UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINT` writer - Clear UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` writer - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_MATCH` writer - Clear Address Match Interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` writer - Clear UART Clear to Send Modem Interrupt."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_RX` writer - Clear DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_TX` writer - Clear DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTOUT` writer - Clear UARTOUT Line Time-Out Interrupt."]
pub type LtoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, IclrSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FrmerrW<'_, IclrSpec> {
        FrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<'_, IclrSpec> {
        ParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear UART Break Error Interrupt."]
    #[inline(always)]
    pub fn brkerr(&mut self) -> BrkerrW<'_, IclrSpec> {
        BrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn ovrerr(&mut self) -> OvrerrW<'_, IclrSpec> {
        OvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, IclrSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&mut self) -> RxpeW<'_, IclrSpec> {
        RxpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&mut self) -> Linc0W<'_, IclrSpec> {
        Linc0W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&mut self) -> Linc1W<'_, IclrSpec> {
        Linc1W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&mut self) -> LinovfW<'_, IclrSpec> {
        LinovfW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, IclrSpec> {
        RxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, IclrSpec> {
        TxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<'_, IclrSpec> {
        EotW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, IclrSpec> {
        AddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IclrSpec> {
        CtsW::new(self, 14)
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
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&mut self) -> NerrW<'_, IclrSpec> {
        NerrW::new(self, 17)
    }
    #[doc = "Bit 20 - Clear UARTOUT Line Time-Out Interrupt."]
    #[inline(always)]
    pub fn ltout(&mut self) -> LtoutW<'_, IclrSpec> {
        LtoutW::new(self, 20)
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
