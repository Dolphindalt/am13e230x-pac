#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Receive Done Flag"]
    Rxdonefg = 1,
    #[doc = "2: Transmit Done Flag"]
    Txdonefg = 2,
    #[doc = "3: receive FIFO Trigger Level"]
    Rxtrg = 3,
    #[doc = "4: transmit FIFO Trigger level"]
    Txtrg = 4,
    #[doc = "5: RX FIFO FULL Event/interrupt pending"]
    Rxfull = 5,
    #[doc = "6: Transmit FIFO/Buffer Empty Event/interrupt pending"]
    Txempty = 6,
    #[doc = "7: Target TX FIFO underflow"]
    TxUnfl = 7,
    #[doc = "8: Target RX FIFO overflow event"]
    RxOvfl = 8,
    #[doc = "9: General Call Event"]
    Gencall = 9,
    #[doc = "10: Start Event"]
    Startfg = 10,
    #[doc = "11: Stop Event"]
    Stopfg = 11,
    #[doc = "12: PEC receive error event"]
    PecRxErr = 12,
    #[doc = "13: Timeout A Event"]
    Timeouta = 13,
    #[doc = "14: Timeout B Event"]
    Timeoutb = 14,
    #[doc = "16: DMA DONE on Channel RX"]
    DmaDoneRx = 16,
    #[doc = "17: DMA DONE on Channel TX"]
    DmaDoneTx = 17,
    #[doc = "18: Arbitration Lost"]
    Arblostfg = 18,
    #[doc = "19: DMA PRE IRQ INTERRUPT"]
    DmaPreirqRx = 19,
    #[doc = "20: DMA PRE IRQ INTERRUPT"]
    DmaPreirqTx = 20,
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
#[doc = "Field `STAT` reader - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Rxdonefg),
            2 => Some(Stat::Txdonefg),
            3 => Some(Stat::Rxtrg),
            4 => Some(Stat::Txtrg),
            5 => Some(Stat::Rxfull),
            6 => Some(Stat::Txempty),
            7 => Some(Stat::TxUnfl),
            8 => Some(Stat::RxOvfl),
            9 => Some(Stat::Gencall),
            10 => Some(Stat::Startfg),
            11 => Some(Stat::Stopfg),
            12 => Some(Stat::PecRxErr),
            13 => Some(Stat::Timeouta),
            14 => Some(Stat::Timeoutb),
            16 => Some(Stat::DmaDoneRx),
            17 => Some(Stat::DmaDoneTx),
            18 => Some(Stat::Arblostfg),
            19 => Some(Stat::DmaPreirqRx),
            20 => Some(Stat::DmaPreirqTx),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Receive Done Flag"]
    #[inline(always)]
    pub fn is_rxdonefg(&self) -> bool {
        *self == Stat::Rxdonefg
    }
    #[doc = "Transmit Done Flag"]
    #[inline(always)]
    pub fn is_txdonefg(&self) -> bool {
        *self == Stat::Txdonefg
    }
    #[doc = "receive FIFO Trigger Level"]
    #[inline(always)]
    pub fn is_rxtrg(&self) -> bool {
        *self == Stat::Rxtrg
    }
    #[doc = "transmit FIFO Trigger level"]
    #[inline(always)]
    pub fn is_txtrg(&self) -> bool {
        *self == Stat::Txtrg
    }
    #[doc = "RX FIFO FULL Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rxfull(&self) -> bool {
        *self == Stat::Rxfull
    }
    #[doc = "Transmit FIFO/Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_txempty(&self) -> bool {
        *self == Stat::Txempty
    }
    #[doc = "Target TX FIFO underflow"]
    #[inline(always)]
    pub fn is_tx_unfl(&self) -> bool {
        *self == Stat::TxUnfl
    }
    #[doc = "Target RX FIFO overflow event"]
    #[inline(always)]
    pub fn is_rx_ovfl(&self) -> bool {
        *self == Stat::RxOvfl
    }
    #[doc = "General Call Event"]
    #[inline(always)]
    pub fn is_gencall(&self) -> bool {
        *self == Stat::Gencall
    }
    #[doc = "Start Event"]
    #[inline(always)]
    pub fn is_startfg(&self) -> bool {
        *self == Stat::Startfg
    }
    #[doc = "Stop Event"]
    #[inline(always)]
    pub fn is_stopfg(&self) -> bool {
        *self == Stat::Stopfg
    }
    #[doc = "PEC receive error event"]
    #[inline(always)]
    pub fn is_pec_rx_err(&self) -> bool {
        *self == Stat::PecRxErr
    }
    #[doc = "Timeout A Event"]
    #[inline(always)]
    pub fn is_timeouta(&self) -> bool {
        *self == Stat::Timeouta
    }
    #[doc = "Timeout B Event"]
    #[inline(always)]
    pub fn is_timeoutb(&self) -> bool {
        *self == Stat::Timeoutb
    }
    #[doc = "DMA DONE on Channel RX"]
    #[inline(always)]
    pub fn is_dma_done_rx(&self) -> bool {
        *self == Stat::DmaDoneRx
    }
    #[doc = "DMA DONE on Channel TX"]
    #[inline(always)]
    pub fn is_dma_done_tx(&self) -> bool {
        *self == Stat::DmaDoneTx
    }
    #[doc = "Arbitration Lost"]
    #[inline(always)]
    pub fn is_arblostfg(&self) -> bool {
        *self == Stat::Arblostfg
    }
    #[doc = "DMA PRE IRQ INTERRUPT"]
    #[inline(always)]
    pub fn is_dma_preirq_rx(&self) -> bool {
        *self == Stat::DmaPreirqRx
    }
    #[doc = "DMA PRE IRQ INTERRUPT"]
    #[inline(always)]
    pub fn is_dma_preirq_tx(&self) -> bool {
        *self == Stat::DmaPreirqTx
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
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
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
