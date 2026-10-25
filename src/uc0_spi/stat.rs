#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Receive FIFO empty.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfe {
    #[doc = "0: Receive FIFO is not empty."]
    NotEmpty = 0,
    #[doc = "1: Receive FIFO is empty."]
    Empty = 1,
}
impl From<Rxfe> for bool {
    #[inline(always)]
    fn from(variant: Rxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO empty."]
pub type RxfeR = crate::BitReader<Rxfe>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfe {
        match self.bits {
            false => Rxfe::NotEmpty,
            true => Rxfe::Empty,
        }
    }
    #[doc = "Receive FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rxfe::NotEmpty
    }
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rxfe::Empty
    }
}
#[doc = "Field `RXFF` reader - Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXCLR` reader - RX FIFO Clear Status"]
pub type RxclrR = crate::BitReader;
#[doc = "Transmit FIFO empty.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfe {
    #[doc = "0: Transmit FIFO is not empty."]
    NotEmpty = 0,
    #[doc = "1: Transmit FIFO is empty."]
    Empty = 1,
}
impl From<Txfe> for bool {
    #[inline(always)]
    fn from(variant: Txfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO empty."]
pub type TxfeR = crate::BitReader<Txfe>;
impl TxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfe {
        match self.bits {
            false => Txfe::NotEmpty,
            true => Txfe::Empty,
        }
    }
    #[doc = "Transmit FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Txfe::NotEmpty
    }
    #[doc = "Transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txfe::Empty
    }
}
#[doc = "Field `TXFF` reader - Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxffR = crate::BitReader;
#[doc = "Field `TXCLR` reader - TX FIFO Clear Status"]
pub type TxclrR = crate::BitReader;
#[doc = "Busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: SPI is in idle mode."]
    Idle = 0,
    #[doc = "1: SPI is currently transmitting and/or receiving data"]
    Active = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Active,
        }
    }
    #[doc = "SPI is in idle mode."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "SPI is currently transmitting and/or receiving data"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
}
impl R {
    #[doc = "Bit 2 - Receive FIFO empty."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Clear Status"]
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Clear Status"]
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("txfe", &self.txfe())
            .field("rxfe", &self.rxfe())
            .field("busy", &self.busy())
            .field("rxclr", &self.rxclr())
            .field("txclr", &self.txclr())
            .field("rxff", &self.rxff())
            .field("txff", &self.txff())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
