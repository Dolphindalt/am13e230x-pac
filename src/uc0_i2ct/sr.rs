#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RREQ` reader - Receive Request"]
pub type RreqR = crate::BitReader;
#[doc = "Field `TREQ` reader - Transmit Request"]
pub type TreqR = crate::BitReader;
#[doc = "Field `RXMODE` reader - Target FSM is in Rx MODE"]
pub type RxmodeR = crate::BitReader;
#[doc = "Field `OAR2SEL` reader - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
pub type Oar2selR = crate::BitReader;
#[doc = "Field `QCMDST` reader - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
pub type QcmdstR = crate::BitReader;
#[doc = "Field `QCMDRW` reader - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
pub type QcmdrwR = crate::BitReader;
#[doc = "Field `BUSBSY` reader - I2C bus is busy"]
pub type BusbsyR = crate::BitReader;
#[doc = "Field `TXMODE` reader - FSM is in TX MODE"]
pub type TxmodeR = crate::BitReader;
#[doc = "Field `STALE_TXFIFO` reader - Stale Tx FIFO"]
pub type StaleTxfifoR = crate::BitReader;
#[doc = "Field `RXCLR` reader - RX FIFO Clear Status"]
pub type RxclrR = crate::BitReader;
#[doc = "Field `TXCLR` reader - TX FIFO Clear Status"]
pub type TxclrR = crate::BitReader;
#[doc = "Field `RXFE` reader - Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type RxfeR = crate::BitReader;
#[doc = "Field `RXFF` reader - Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type RxffR = crate::BitReader;
#[doc = "Field `TXFE` reader - Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFF` reader - Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
pub type TxffR = crate::BitReader;
#[doc = "Status of ACK Override Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackoen {
    #[doc = "0: ACK override is disabled"]
    Disable = 0,
    #[doc = "1: ACK override was enabled in design"]
    Enable = 1,
}
impl From<Ackoen> for bool {
    #[inline(always)]
    fn from(variant: Ackoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN` reader - Status of ACK Override Enable"]
pub type AckoenR = crate::BitReader<Ackoen>;
impl AckoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackoen {
        match self.bits {
            false => Ackoen::Disable,
            true => Ackoen::Enable,
        }
    }
    #[doc = "ACK override is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ackoen::Disable
    }
    #[doc = "ACK override was enabled in design"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ackoen::Enable
    }
}
#[doc = "Field `ADDRMATCH` reader - Indicates the address for which Target address match happened"]
pub type AddrmatchR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RreqR {
        RreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn treq(&self) -> TreqR {
        TreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Target FSM is in Rx MODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RxmodeR {
        RxmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
    #[inline(always)]
    pub fn oar2sel(&self) -> Oar2selR {
        Oar2selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
    #[inline(always)]
    pub fn qcmdst(&self) -> QcmdstR {
        QcmdstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
    #[inline(always)]
    pub fn qcmdrw(&self) -> QcmdrwR {
        QcmdrwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C bus is busy"]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FSM is in TX MODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TxmodeR {
        TxmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stale Tx FIFO"]
    #[inline(always)]
    pub fn stale_txfifo(&self) -> StaleTxfifoR {
        StaleTxfifoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO Clear Status"]
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Clear Status"]
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Empty The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO Full The meaning of this bit depends on the state of the FEN bit in the CTL0 register."]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status of ACK Override Enable"]
    #[inline(always)]
    pub fn ackoen(&self) -> AckoenR {
        AckoenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Indicates the address for which Target address match happened"]
    #[inline(always)]
    pub fn addrmatch(&self) -> AddrmatchR {
        AddrmatchR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rreq", &self.rreq())
            .field("treq", &self.treq())
            .field("oar2sel", &self.oar2sel())
            .field("qcmdst", &self.qcmdst())
            .field("qcmdrw", &self.qcmdrw())
            .field("rxmode", &self.rxmode())
            .field("busbsy", &self.busbsy())
            .field("txmode", &self.txmode())
            .field("stale_txfifo", &self.stale_txfifo())
            .field("addrmatch", &self.addrmatch())
            .field("rxclr", &self.rxclr())
            .field("txclr", &self.txclr())
            .field("rxfe", &self.rxfe())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("txff", &self.txff())
            .field("ackoen", &self.ackoen())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
