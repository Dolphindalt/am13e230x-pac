#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `BUSY` reader - FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
pub type BusyR = crate::BitReader;
#[doc = "Field `ERR` reader - Error The error can be from the Target address not being acknowledged or the transmit data not being acknowledged."]
pub type ErrR = crate::BitReader;
#[doc = "Field `ADRACK` reader - Acknowledge Address"]
pub type AdrackR = crate::BitReader;
#[doc = "Field `DATACK` reader - Acknowledge Data"]
pub type DatackR = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub type ArblstR = crate::BitReader;
#[doc = "Field `IDLE` reader - I2C Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `BUSBSY` reader - I2C Bus is Busy Controller State Machine will wait until this bit to be cleared before starting a transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the CTR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
pub type BusbsyR = crate::BitReader;
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
#[doc = "Field `BCNT` reader - transaction count"]
pub type BcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error The error can be from the Target address not being acknowledged or the transmit data not being acknowledged."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn adrack(&self) -> AdrackR {
        AdrackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn datack(&self) -> DatackR {
        DatackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus is Busy Controller State Machine will wait until this bit to be cleared before starting a transaction. When first enabling the Controller in multi Controller environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the CTR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bits 16:27 - transaction count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("err", &self.err())
            .field("adrack", &self.adrack())
            .field("datack", &self.datack())
            .field("arblst", &self.arblst())
            .field("idle", &self.idle())
            .field("busbsy", &self.busbsy())
            .field("bcnt", &self.bcnt())
            .field("rxclr", &self.rxclr())
            .field("txclr", &self.txclr())
            .field("rxfe", &self.rxfe())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("txff", &self.txff())
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
