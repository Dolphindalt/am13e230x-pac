#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RTOUT` reader - UARTOUT Receive Time-Out Interrupt. Interupt is raised when a time-out has lapsed and there are still items in the FIFO"]
pub type RtoutR = crate::BitReader;
#[doc = "Field `FRMERR` reader - UART Framing Error Interrupt. Framing error interrupt is raised when a &quot;low&quot; stop bit is detected."]
pub type FrmerrR = crate::BitReader;
#[doc = "Field `PARERR` reader - UART Parity Error Interrupt."]
pub type ParerrR = crate::BitReader;
#[doc = "Field `BRKERR` reader - UART Break Error Interrupt. Break Error Interrupt is raised when all data bits, parity bit and stop bits are '0'"]
pub type BrkerrR = crate::BitReader;
#[doc = "Field `OVRERR` reader - UART Receive Overrun Error Interrupt. UART Receive Overrun Error Interrupt is raised when the receive FIFO is flow and a new frame is received"]
pub type OvrerrR = crate::BitReader;
#[doc = "Field `RXNE` reader - Negative Edge on UARTxRXD Interrupt."]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXPE` reader - Positive Edge on UARTxRXD Interrupt."]
pub type RxpeR = crate::BitReader;
#[doc = "Field `LINC0` reader - LIN Capture 0 / Match Interrupt ."]
pub type Linc0R = crate::BitReader;
#[doc = "Field `LINC1` reader - LIN Capture 1 Interrupt."]
pub type Linc1R = crate::BitReader;
#[doc = "Field `LINOVF` reader - LIN Hardware Counter Overflow Interrupt."]
pub type LinovfR = crate::BitReader;
#[doc = "Field `RXINT` reader - UART Receive Interrupt."]
pub type RxintR = crate::BitReader;
#[doc = "Field `TXINT` reader - UART Transmit Interrupt."]
pub type TxintR = crate::BitReader;
#[doc = "Field `EOT` reader - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type EotR = crate::BitReader;
#[doc = "Field `ADDR_MATCH` reader - Address Match Interrupt."]
pub type AddrMatchR = crate::BitReader;
#[doc = "Field `CTS` reader - UART Clear to Send Modem Interrupt. This interrupt is raised when CTS toggles, either from '0' to '1' or from '1' to '0'."]
pub type CtsR = crate::BitReader;
#[doc = "Field `DMA_DONE_RX` reader - DMA Done on RX Event Channel Interrupt"]
pub type DmaDoneRxR = crate::BitReader;
#[doc = "Field `DMA_DONE_TX` reader - DMA Done on TX Event Channel Interrupt"]
pub type DmaDoneTxR = crate::BitReader;
#[doc = "Field `NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type NerrR = crate::BitReader;
#[doc = "Field `LTOUT` reader - UARTOUT Line Time-Out Interrupt. Set: no start edge has been detected for an additional character period after reception of last character. This is irrespective of FIFO contents Clear: timeout has not occured"]
pub type LtoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UARTOUT Receive Time-Out Interrupt. Interupt is raised when a time-out has lapsed and there are still items in the FIFO"]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Framing Error Interrupt. Framing error interrupt is raised when a &quot;low&quot; stop bit is detected."]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Break Error Interrupt. Break Error Interrupt is raised when all data bits, parity bit and stop bits are '0'"]
    #[inline(always)]
    pub fn brkerr(&self) -> BrkerrR {
        BrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Overrun Error Interrupt. UART Receive Overrun Error Interrupt is raised when the receive FIFO is flow and a new frame is received"]
    #[inline(always)]
    pub fn ovrerr(&self) -> OvrerrR {
        OvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn rxpe(&self) -> RxpeR {
        RxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn linc0(&self) -> Linc0R {
        Linc0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn linc1(&self) -> Linc1R {
        Linc1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn linovf(&self) -> LinovfR {
        LinovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART Clear to Send Modem Interrupt. This interrupt is raised when CTS toggles, either from '0' to '1' or from '1' to '0'."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - UARTOUT Line Time-Out Interrupt. Set: no start edge has been detected for an additional character period after reception of last character. This is irrespective of FIFO contents Clear: timeout has not occured"]
    #[inline(always)]
    pub fn ltout(&self) -> LtoutR {
        LtoutR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("rtout", &self.rtout())
            .field("frmerr", &self.frmerr())
            .field("parerr", &self.parerr())
            .field("brkerr", &self.brkerr())
            .field("ovrerr", &self.ovrerr())
            .field("rxne", &self.rxne())
            .field("rxpe", &self.rxpe())
            .field("linc0", &self.linc0())
            .field("linc1", &self.linc1())
            .field("linovf", &self.linovf())
            .field("rxint", &self.rxint())
            .field("txint", &self.txint())
            .field("eot", &self.eot())
            .field("addr_match", &self.addr_match())
            .field("cts", &self.cts())
            .field("dma_done_rx", &self.dma_done_rx())
            .field("dma_done_tx", &self.dma_done_tx())
            .field("nerr", &self.nerr())
            .field("ltout", &self.ltout())
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
