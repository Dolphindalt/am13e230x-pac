#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `RF0NE` reader - Rx FIFO 0 New Message Enable"]
pub type Rf0neR = crate::BitReader;
#[doc = "Field `RF0NE` writer - Rx FIFO 0 New Message Enable"]
pub type Rf0neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0WE` reader - Rx FIFO 0 Watermark Reached Enable"]
pub type Rf0weR = crate::BitReader;
#[doc = "Field `RF0WE` writer - Rx FIFO 0 Watermark Reached Enable"]
pub type Rf0weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 Full Enable"]
pub type Rf0feR = crate::BitReader;
#[doc = "Field `RF0FE` writer - Rx FIFO 0 Full Enable"]
pub type Rf0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 Message Lost Enable"]
pub type Rf0leR = crate::BitReader;
#[doc = "Field `RF0LE` writer - Rx FIFO 0 Message Lost Enable"]
pub type Rf0leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 New Message Enable"]
pub type Rf1neR = crate::BitReader;
#[doc = "Field `RF1NE` writer - Rx FIFO 1 New Message Enable"]
pub type Rf1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type Rf1weR = crate::BitReader;
#[doc = "Field `RF1WE` writer - Rx FIFO 1 Watermark Reached Enable"]
pub type Rf1weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 Full Enable"]
pub type Rf1feR = crate::BitReader;
#[doc = "Field `RF1FE` writer - Rx FIFO 1 Full Enable"]
pub type Rf1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 Message Lost Enable"]
pub type Rf1leR = crate::BitReader;
#[doc = "Field `RF1LE` writer - Rx FIFO 1 Message Lost Enable"]
pub type Rf1leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPME` reader - High Priority Message Enable"]
pub type HpmeR = crate::BitReader;
#[doc = "Field `HPME` writer - High Priority Message Enable"]
pub type HpmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - Transmission Completed Enable"]
pub type TceR = crate::BitReader;
#[doc = "Field `TCE` writer - Transmission Completed Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFE` reader - Transmission Cancellation Finished Enable"]
pub type TcfeR = crate::BitReader;
#[doc = "Field `TCFE` writer - Transmission Cancellation Finished Enable"]
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEE` reader - Tx FIFO Empty Enable"]
pub type TfeeR = crate::BitReader;
#[doc = "Field `TFEE` writer - Tx FIFO Empty Enable"]
pub type TfeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNE` reader - Tx Event FIFO New Entry Enable"]
pub type TefneR = crate::BitReader;
#[doc = "Field `TEFNE` writer - Tx Event FIFO New Entry Enable"]
pub type TefneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFWE` reader - Tx Event FIFO Watermark Reached Enable"]
pub type TefweR = crate::BitReader;
#[doc = "Field `TEFWE` writer - Tx Event FIFO Watermark Reached Enable"]
pub type TefweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFE` reader - Tx Event FIFO Full Enable"]
pub type TeffeR = crate::BitReader;
#[doc = "Field `TEFFE` writer - Tx Event FIFO Full Enable"]
pub type TeffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLE` reader - Tx Event FIFO Element Lost Enable"]
pub type TefleR = crate::BitReader;
#[doc = "Field `TEFLE` writer - Tx Event FIFO Element Lost Enable"]
pub type TefleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWE` reader - Timestamp Wraparound Enable"]
pub type TsweR = crate::BitReader;
#[doc = "Field `TSWE` writer - Timestamp Wraparound Enable"]
pub type TsweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFE` reader - Message RAM Access Failure Enable"]
pub type MrafeR = crate::BitReader;
#[doc = "Field `MRAFE` writer - Message RAM Access Failure Enable"]
pub type MrafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOE` reader - Timeout Occurred Enable"]
pub type TooeR = crate::BitReader;
#[doc = "Field `TOOE` writer - Timeout Occurred Enable"]
pub type TooeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRXE` reader - Message Stored to Dedicated Rx Buffer Enable"]
pub type DrxeR = crate::BitReader;
#[doc = "Field `DRXE` writer - Message Stored to Dedicated Rx Buffer Enable"]
pub type DrxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECE` reader - Bit Error Corrected Enable"]
pub type BeceR = crate::BitReader;
#[doc = "Field `BECE` writer - Bit Error Corrected Enable"]
pub type BeceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEUE` reader - Bit Error Uncorrected Enable"]
pub type BeueR = crate::BitReader;
#[doc = "Field `BEUE` writer - Bit Error Uncorrected Enable"]
pub type BeueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOE` reader - Error Logging Overflow Enable"]
pub type EloeR = crate::BitReader;
#[doc = "Field `ELOE` writer - Error Logging Overflow Enable"]
pub type EloeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - Error Passive Enable"]
pub type EpeR = crate::BitReader;
#[doc = "Field `EPE` writer - Error Passive Enable"]
pub type EpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWE` reader - Warning Status Enable"]
pub type EweR = crate::BitReader;
#[doc = "Field `EWE` writer - Warning Status Enable"]
pub type EweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOE` reader - Bus_Off Status Enable"]
pub type BoeR = crate::BitReader;
#[doc = "Field `BOE` writer - Bus_Off Status Enable"]
pub type BoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - Watchdog Interrupt Enable"]
pub type WdieR = crate::BitReader;
#[doc = "Field `WDIE` writer - Watchdog Interrupt Enable"]
pub type WdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub type PeaeR = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol Error in Arbitration Phase Enable"]
pub type PeaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - Protocol Error in Data Phase Enable"]
pub type PedeR = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol Error in Data Phase Enable"]
pub type PedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - Access to Reserved Address Enable"]
pub type AraeR = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to Reserved Address Enable"]
pub type AraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> Rf0neR {
        Rf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> Rf0weR {
        Rf0weR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> Rf0feR {
        Rf0feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> Rf0leR {
        Rf0leR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> Rf1neR {
        Rf1neR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> Rf1weR {
        Rf1weR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> Rf1feR {
        Rf1feR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> Rf1leR {
        Rf1leR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HpmeR {
        HpmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TfeeR {
        TfeeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TefneR {
        TefneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TefweR {
        TefweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TeffeR {
        TeffeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TefleR {
        TefleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TsweR {
        TsweR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MrafeR {
        MrafeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TooeR {
        TooeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DrxeR {
        DrxeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BeceR {
        BeceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BeueR {
        BeueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> EloeR {
        EloeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EweR {
        EweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BoeR {
        BoeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WdieR {
        WdieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PeaeR {
        PeaeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PedeR {
        PedeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> AraeR {
        AraeR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0we", &self.rf0we())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1we", &self.rf1we())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("tefwe", &self.tefwe())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("drxe", &self.drxe())
            .field("bece", &self.bece())
            .field("beue", &self.beue())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> Rf0neW<'_, IeSpec> {
        Rf0neW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf0we(&mut self) -> Rf0weW<'_, IeSpec> {
        Rf0weW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> Rf0feW<'_, IeSpec> {
        Rf0feW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn rf0le(&mut self) -> Rf0leW<'_, IeSpec> {
        Rf0leW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> Rf1neW<'_, IeSpec> {
        Rf1neW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1we(&mut self) -> Rf1weW<'_, IeSpec> {
        Rf1weW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Enable"]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> Rf1feW<'_, IeSpec> {
        Rf1feW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn rf1le(&mut self) -> Rf1leW<'_, IeSpec> {
        Rf1leW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn hpme(&mut self) -> HpmeW<'_, IeSpec> {
        HpmeW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TceW<'_, IeSpec> {
        TceW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TcfeW<'_, IeSpec> {
        TcfeW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn tfee(&mut self) -> TfeeW<'_, IeSpec> {
        TfeeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn tefne(&mut self) -> TefneW<'_, IeSpec> {
        TefneW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn tefwe(&mut self) -> TefweW<'_, IeSpec> {
        TefweW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn teffe(&mut self) -> TeffeW<'_, IeSpec> {
        TeffeW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn tefle(&mut self) -> TefleW<'_, IeSpec> {
        TefleW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn tswe(&mut self) -> TsweW<'_, IeSpec> {
        TsweW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mrafe(&mut self) -> MrafeW<'_, IeSpec> {
        MrafeW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn tooe(&mut self) -> TooeW<'_, IeSpec> {
        TooeW::new(self, 18)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn drxe(&mut self) -> DrxeW<'_, IeSpec> {
        DrxeW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected Enable"]
    #[inline(always)]
    pub fn bece(&mut self) -> BeceW<'_, IeSpec> {
        BeceW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Enable"]
    #[inline(always)]
    pub fn beue(&mut self) -> BeueW<'_, IeSpec> {
        BeueW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn eloe(&mut self) -> EloeW<'_, IeSpec> {
        EloeW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn epe(&mut self) -> EpeW<'_, IeSpec> {
        EpeW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn ewe(&mut self) -> EweW<'_, IeSpec> {
        EweW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn boe(&mut self) -> BoeW<'_, IeSpec> {
        BoeW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&mut self) -> WdieW<'_, IeSpec> {
        WdieW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&mut self) -> PeaeW<'_, IeSpec> {
        PeaeW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&mut self) -> PedeW<'_, IeSpec> {
        PedeW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&mut self) -> AraeW<'_, IeSpec> {
        AraeW::new(self, 29)
    }
}
#[doc = "MCAN Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {}
