#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `BUSY` reader - UART Busy This bit is set as soon as the transmit FIFO/buffer becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
pub type BusyR = crate::BitReader;
#[doc = "Field `RXFE` reader - UART Receive FIFO Empty"]
pub type RxfeR = crate::BitReader;
#[doc = "Field `RXFF` reader - UART Receive FIFO Full"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXCLR` reader - RX FIFO Clear Status"]
pub type RxclrR = crate::BitReader;
#[doc = "Field `TXFE` reader - UART Transmit FIFO Empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFF` reader - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxffR = crate::BitReader;
#[doc = "Field `TXCLR` reader - TX FIFO Clear Status"]
pub type TxclrR = crate::BitReader;
#[doc = "Field `CTS` reader - Clear To Send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `IDLE` reader - IDLE mode has been detected in Idleline-Multiprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
pub type IdleR = crate::BitReader;
#[doc = "Field `SENDIDLE` reader - TX FIFO Clear Status"]
pub type SendidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART Busy This bit is set as soon as the transmit FIFO/buffer becomes non-empty (regardless of whether UART is enabled) or if a receive data is currently ongoing (after the start edge have been detected until a complete byte, including all stop bits, has been received by the shift register). In IDLE_Line mode the Busy signal also stays set during the idle time generation."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Clear Status"]
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Clear Status"]
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDLE mode has been detected in Idleline-Multiprocessor-Mode. The IDLE bit is used as an address tag for each block of characters. In idle-line multiprocessor format, this bit is set when a received character is an address."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Clear Status"]
    #[inline(always)]
    pub fn sendidle(&self) -> SendidleR {
        SendidleR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("busy", &self.busy())
            .field("txff", &self.txff())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("cts", &self.cts())
            .field("idle", &self.idle())
            .field("rxfe", &self.rxfe())
            .field("rxclr", &self.rxclr())
            .field("txclr", &self.txclr())
            .field("sendidle", &self.sendidle())
            .finish()
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
