#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Transmit Parity Event/interrupt pending"]
    PerEvt = 1,
    #[doc = "2: RX FIFO Overflow Event/interrupt pending"]
    RxfifoOfvEvt = 2,
    #[doc = "3: Receive interrupt"]
    RxEvt = 3,
    #[doc = "4: RX FIFO Full Interrupt"]
    RxfullEvt = 4,
    #[doc = "5: TX FIFO underflow interrupt"]
    TxfifoUnfEvt = 5,
    #[doc = "6: Transmit Event"]
    TxEvt = 6,
    #[doc = "7: Transmit Buffer Empty Event/interrupt pending"]
    TxEmpty = 7,
    #[doc = "9: End of Transmit Event/interrupt pending"]
    IdleEvt = 9,
    #[doc = "10: SPI receive time-out interrupt"]
    RtoutEvt = 10,
    #[doc = "16: DMA DONE on RX"]
    DmaDoneRx = 16,
    #[doc = "17: DMA DONE on TX"]
    DmaDoneTx = 17,
    #[doc = "19: DMA PRE IRQ RX INTERRUPT"]
    DmaPreirqRx = 19,
    #[doc = "20: DMA PRE IRQ TX INTERRUPT"]
    DmaPreirqTx = 20,
    #[doc = "21: SPI line time-out interrupt"]
    Ltout = 21,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::PerEvt),
            2 => Some(Stat::RxfifoOfvEvt),
            3 => Some(Stat::RxEvt),
            4 => Some(Stat::RxfullEvt),
            5 => Some(Stat::TxfifoUnfEvt),
            6 => Some(Stat::TxEvt),
            7 => Some(Stat::TxEmpty),
            9 => Some(Stat::IdleEvt),
            10 => Some(Stat::RtoutEvt),
            16 => Some(Stat::DmaDoneRx),
            17 => Some(Stat::DmaDoneTx),
            19 => Some(Stat::DmaPreirqRx),
            20 => Some(Stat::DmaPreirqTx),
            21 => Some(Stat::Ltout),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Transmit Parity Event/interrupt pending"]
    #[inline(always)]
    pub fn is_per_evt(&self) -> bool {
        *self == Stat::PerEvt
    }
    #[doc = "RX FIFO Overflow Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rxfifo_ofv_evt(&self) -> bool {
        *self == Stat::RxfifoOfvEvt
    }
    #[doc = "Receive interrupt"]
    #[inline(always)]
    pub fn is_rx_evt(&self) -> bool {
        *self == Stat::RxEvt
    }
    #[doc = "RX FIFO Full Interrupt"]
    #[inline(always)]
    pub fn is_rxfull_evt(&self) -> bool {
        *self == Stat::RxfullEvt
    }
    #[doc = "TX FIFO underflow interrupt"]
    #[inline(always)]
    pub fn is_txfifo_unf_evt(&self) -> bool {
        *self == Stat::TxfifoUnfEvt
    }
    #[doc = "Transmit Event"]
    #[inline(always)]
    pub fn is_tx_evt(&self) -> bool {
        *self == Stat::TxEvt
    }
    #[doc = "Transmit Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_empty(&self) -> bool {
        *self == Stat::TxEmpty
    }
    #[doc = "End of Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_idle_evt(&self) -> bool {
        *self == Stat::IdleEvt
    }
    #[doc = "SPI receive time-out interrupt"]
    #[inline(always)]
    pub fn is_rtout_evt(&self) -> bool {
        *self == Stat::RtoutEvt
    }
    #[doc = "DMA DONE on RX"]
    #[inline(always)]
    pub fn is_dma_done_rx(&self) -> bool {
        *self == Stat::DmaDoneRx
    }
    #[doc = "DMA DONE on TX"]
    #[inline(always)]
    pub fn is_dma_done_tx(&self) -> bool {
        *self == Stat::DmaDoneTx
    }
    #[doc = "DMA PRE IRQ RX INTERRUPT"]
    #[inline(always)]
    pub fn is_dma_preirq_rx(&self) -> bool {
        *self == Stat::DmaPreirqRx
    }
    #[doc = "DMA PRE IRQ TX INTERRUPT"]
    #[inline(always)]
    pub fn is_dma_preirq_tx(&self) -> bool {
        *self == Stat::DmaPreirqTx
    }
    #[doc = "SPI line time-out interrupt"]
    #[inline(always)]
    pub fn is_ltout(&self) -> bool {
        *self == Stat::Ltout
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDX").field("stat", &self.stat()).finish()
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
