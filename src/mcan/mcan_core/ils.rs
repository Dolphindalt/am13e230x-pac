#[doc = "Register `ILS` reader"]
pub type R = crate::R<IlsSpec>;
#[doc = "Register `ILS` writer"]
pub type W = crate::W<IlsSpec>;
#[doc = "Field `RF0NL` reader - Rx FIFO 0 New Message Line"]
pub type Rf0nlR = crate::BitReader;
#[doc = "Field `RF0NL` writer - Rx FIFO 0 New Message Line"]
pub type Rf0nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Line"]
pub type Rf0wlR = crate::BitReader;
#[doc = "Field `RF0WL` writer - Rx FIFO 0 Watermark Reached Line"]
pub type Rf0wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FL` reader - Rx FIFO 0 Full Line"]
pub type Rf0flR = crate::BitReader;
#[doc = "Field `RF0FL` writer - Rx FIFO 0 Full Line"]
pub type Rf0flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LL` reader - Rx FIFO 0 Message Lost Line"]
pub type Rf0llR = crate::BitReader;
#[doc = "Field `RF0LL` writer - Rx FIFO 0 Message Lost Line"]
pub type Rf0llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NL` reader - Rx FIFO 1 New Message Line"]
pub type Rf1nlR = crate::BitReader;
#[doc = "Field `RF1NL` writer - Rx FIFO 1 New Message Line"]
pub type Rf1nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Line"]
pub type Rf1wlR = crate::BitReader;
#[doc = "Field `RF1WL` writer - Rx FIFO 1 Watermark Reached Line"]
pub type Rf1wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FL` reader - Rx FIFO 1 Full Line"]
pub type Rf1flR = crate::BitReader;
#[doc = "Field `RF1FL` writer - Rx FIFO 1 Full Line"]
pub type Rf1flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LL` reader - Rx FIFO 1 Message Lost Line"]
pub type Rf1llR = crate::BitReader;
#[doc = "Field `RF1LL` writer - Rx FIFO 1 Message Lost Line"]
pub type Rf1llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPML` reader - High Priority Message Line"]
pub type HpmlR = crate::BitReader;
#[doc = "Field `HPML` writer - High Priority Message Line"]
pub type HpmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL` reader - Transmission Completed Line"]
pub type TclR = crate::BitReader;
#[doc = "Field `TCL` writer - Transmission Completed Line"]
pub type TclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFL` reader - Transmission Cancellation Finished Line"]
pub type TcflR = crate::BitReader;
#[doc = "Field `TCFL` writer - Transmission Cancellation Finished Line"]
pub type TcflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEL` reader - Tx FIFO Empty Line"]
pub type TfelR = crate::BitReader;
#[doc = "Field `TFEL` writer - Tx FIFO Empty Line"]
pub type TfelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNL` reader - Tx Event FIFO New Entry Line"]
pub type TefnlR = crate::BitReader;
#[doc = "Field `TEFNL` writer - Tx Event FIFO New Entry Line"]
pub type TefnlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFWL` reader - Tx Event FIFO Watermark Reached Line"]
pub type TefwlR = crate::BitReader;
#[doc = "Field `TEFWL` writer - Tx Event FIFO Watermark Reached Line"]
pub type TefwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFL` reader - Tx Event FIFO Full Line"]
pub type TefflR = crate::BitReader;
#[doc = "Field `TEFFL` writer - Tx Event FIFO Full Line"]
pub type TefflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLL` reader - Tx Event FIFO Element Lost Line"]
pub type TefllR = crate::BitReader;
#[doc = "Field `TEFLL` writer - Tx Event FIFO Element Lost Line"]
pub type TefllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWL` reader - Timestamp Wraparound Line"]
pub type TswlR = crate::BitReader;
#[doc = "Field `TSWL` writer - Timestamp Wraparound Line"]
pub type TswlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFL` reader - Message RAM Access Failure Line"]
pub type MraflR = crate::BitReader;
#[doc = "Field `MRAFL` writer - Message RAM Access Failure Line"]
pub type MraflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOL` reader - Timeout Occurred Line"]
pub type ToolR = crate::BitReader;
#[doc = "Field `TOOL` writer - Timeout Occurred Line"]
pub type ToolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRXL` reader - Message Stored to Dedicated Rx Buffer Line"]
pub type DrxlR = crate::BitReader;
#[doc = "Field `DRXL` writer - Message Stored to Dedicated Rx Buffer Line"]
pub type DrxlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECL` reader - Bit Error Corrected Line"]
pub type BeclR = crate::BitReader;
#[doc = "Field `BECL` writer - Bit Error Corrected Line"]
pub type BeclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEUL` reader - Bit Error Uncorrected Line"]
pub type BeulR = crate::BitReader;
#[doc = "Field `BEUL` writer - Bit Error Uncorrected Line"]
pub type BeulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOL` reader - Error Logging Overflow Line"]
pub type ElolR = crate::BitReader;
#[doc = "Field `ELOL` writer - Error Logging Overflow Line"]
pub type ElolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPL` reader - Error Passive Line"]
pub type EplR = crate::BitReader;
#[doc = "Field `EPL` writer - Error Passive Line"]
pub type EplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWL` reader - Warning Status Line"]
pub type EwlR = crate::BitReader;
#[doc = "Field `EWL` writer - Warning Status Line"]
pub type EwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOL` reader - Bus_Off Status Line"]
pub type BolR = crate::BitReader;
#[doc = "Field `BOL` writer - Bus_Off Status Line"]
pub type BolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIL` reader - Watchdog Interrupt Line"]
pub type WdilR = crate::BitReader;
#[doc = "Field `WDIL` writer - Watchdog Interrupt Line"]
pub type WdilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAL` reader - Protocol Error in Arbitration Phase Line"]
pub type PealR = crate::BitReader;
#[doc = "Field `PEAL` writer - Protocol Error in Arbitration Phase Line"]
pub type PealW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDL` reader - Protocol Error in Data Phase Line"]
pub type PedlR = crate::BitReader;
#[doc = "Field `PEDL` writer - Protocol Error in Data Phase Line"]
pub type PedlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAL` reader - Access to Reserved Address Line"]
pub type AralR = crate::BitReader;
#[doc = "Field `ARAL` writer - Access to Reserved Address Line"]
pub type AralW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Line"]
    #[inline(always)]
    pub fn rf0nl(&self) -> Rf0nlR {
        Rf0nlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Line"]
    #[inline(always)]
    pub fn rf0wl(&self) -> Rf0wlR {
        Rf0wlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Line"]
    #[inline(always)]
    pub fn rf0fl(&self) -> Rf0flR {
        Rf0flR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Line"]
    #[inline(always)]
    pub fn rf0ll(&self) -> Rf0llR {
        Rf0llR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Line"]
    #[inline(always)]
    pub fn rf1nl(&self) -> Rf1nlR {
        Rf1nlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Line"]
    #[inline(always)]
    pub fn rf1wl(&self) -> Rf1wlR {
        Rf1wlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Line"]
    #[inline(always)]
    pub fn rf1fl(&self) -> Rf1flR {
        Rf1flR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Line"]
    #[inline(always)]
    pub fn rf1ll(&self) -> Rf1llR {
        Rf1llR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Line"]
    #[inline(always)]
    pub fn hpml(&self) -> HpmlR {
        HpmlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Line"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Line"]
    #[inline(always)]
    pub fn tcfl(&self) -> TcflR {
        TcflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Line"]
    #[inline(always)]
    pub fn tfel(&self) -> TfelR {
        TfelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Line"]
    #[inline(always)]
    pub fn tefnl(&self) -> TefnlR {
        TefnlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Line"]
    #[inline(always)]
    pub fn tefwl(&self) -> TefwlR {
        TefwlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Line"]
    #[inline(always)]
    pub fn teffl(&self) -> TefflR {
        TefflR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Line"]
    #[inline(always)]
    pub fn tefll(&self) -> TefllR {
        TefllR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Line"]
    #[inline(always)]
    pub fn tswl(&self) -> TswlR {
        TswlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Line"]
    #[inline(always)]
    pub fn mrafl(&self) -> MraflR {
        MraflR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Line"]
    #[inline(always)]
    pub fn tool(&self) -> ToolR {
        ToolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Line"]
    #[inline(always)]
    pub fn drxl(&self) -> DrxlR {
        DrxlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Line"]
    #[inline(always)]
    pub fn becl(&self) -> BeclR {
        BeclR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Line"]
    #[inline(always)]
    pub fn beul(&self) -> BeulR {
        BeulR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Line"]
    #[inline(always)]
    pub fn elol(&self) -> ElolR {
        ElolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Line"]
    #[inline(always)]
    pub fn epl(&self) -> EplR {
        EplR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Line"]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Line"]
    #[inline(always)]
    pub fn bol(&self) -> BolR {
        BolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&self) -> WdilR {
        WdilR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&self) -> PealR {
        PealR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&self) -> PedlR {
        PedlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&self) -> AralR {
        AralR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILS")
            .field("rf0nl", &self.rf0nl())
            .field("rf0wl", &self.rf0wl())
            .field("rf0fl", &self.rf0fl())
            .field("rf0ll", &self.rf0ll())
            .field("rf1nl", &self.rf1nl())
            .field("rf1wl", &self.rf1wl())
            .field("rf1fl", &self.rf1fl())
            .field("rf1ll", &self.rf1ll())
            .field("hpml", &self.hpml())
            .field("tcl", &self.tcl())
            .field("tcfl", &self.tcfl())
            .field("tfel", &self.tfel())
            .field("tefnl", &self.tefnl())
            .field("tefwl", &self.tefwl())
            .field("teffl", &self.teffl())
            .field("tefll", &self.tefll())
            .field("tswl", &self.tswl())
            .field("mrafl", &self.mrafl())
            .field("tool", &self.tool())
            .field("drxl", &self.drxl())
            .field("becl", &self.becl())
            .field("beul", &self.beul())
            .field("elol", &self.elol())
            .field("epl", &self.epl())
            .field("ewl", &self.ewl())
            .field("bol", &self.bol())
            .field("wdil", &self.wdil())
            .field("peal", &self.peal())
            .field("pedl", &self.pedl())
            .field("aral", &self.aral())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Line"]
    #[inline(always)]
    pub fn rf0nl(&mut self) -> Rf0nlW<'_, IlsSpec> {
        Rf0nlW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Line"]
    #[inline(always)]
    pub fn rf0wl(&mut self) -> Rf0wlW<'_, IlsSpec> {
        Rf0wlW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Line"]
    #[inline(always)]
    pub fn rf0fl(&mut self) -> Rf0flW<'_, IlsSpec> {
        Rf0flW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Line"]
    #[inline(always)]
    pub fn rf0ll(&mut self) -> Rf0llW<'_, IlsSpec> {
        Rf0llW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Line"]
    #[inline(always)]
    pub fn rf1nl(&mut self) -> Rf1nlW<'_, IlsSpec> {
        Rf1nlW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Line"]
    #[inline(always)]
    pub fn rf1wl(&mut self) -> Rf1wlW<'_, IlsSpec> {
        Rf1wlW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Line"]
    #[inline(always)]
    pub fn rf1fl(&mut self) -> Rf1flW<'_, IlsSpec> {
        Rf1flW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Line"]
    #[inline(always)]
    pub fn rf1ll(&mut self) -> Rf1llW<'_, IlsSpec> {
        Rf1llW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message Line"]
    #[inline(always)]
    pub fn hpml(&mut self) -> HpmlW<'_, IlsSpec> {
        HpmlW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed Line"]
    #[inline(always)]
    pub fn tcl(&mut self) -> TclW<'_, IlsSpec> {
        TclW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Line"]
    #[inline(always)]
    pub fn tcfl(&mut self) -> TcflW<'_, IlsSpec> {
        TcflW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Line"]
    #[inline(always)]
    pub fn tfel(&mut self) -> TfelW<'_, IlsSpec> {
        TfelW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Line"]
    #[inline(always)]
    pub fn tefnl(&mut self) -> TefnlW<'_, IlsSpec> {
        TefnlW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Line"]
    #[inline(always)]
    pub fn tefwl(&mut self) -> TefwlW<'_, IlsSpec> {
        TefwlW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Line"]
    #[inline(always)]
    pub fn teffl(&mut self) -> TefflW<'_, IlsSpec> {
        TefflW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Line"]
    #[inline(always)]
    pub fn tefll(&mut self) -> TefllW<'_, IlsSpec> {
        TefllW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Line"]
    #[inline(always)]
    pub fn tswl(&mut self) -> TswlW<'_, IlsSpec> {
        TswlW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Line"]
    #[inline(always)]
    pub fn mrafl(&mut self) -> MraflW<'_, IlsSpec> {
        MraflW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred Line"]
    #[inline(always)]
    pub fn tool(&mut self) -> ToolW<'_, IlsSpec> {
        ToolW::new(self, 18)
    }
    #[doc = "Bit 19 - Message Stored to Dedicated Rx Buffer Line"]
    #[inline(always)]
    pub fn drxl(&mut self) -> DrxlW<'_, IlsSpec> {
        DrxlW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected Line"]
    #[inline(always)]
    pub fn becl(&mut self) -> BeclW<'_, IlsSpec> {
        BeclW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Line"]
    #[inline(always)]
    pub fn beul(&mut self) -> BeulW<'_, IlsSpec> {
        BeulW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow Line"]
    #[inline(always)]
    pub fn elol(&mut self) -> ElolW<'_, IlsSpec> {
        ElolW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive Line"]
    #[inline(always)]
    pub fn epl(&mut self) -> EplW<'_, IlsSpec> {
        EplW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status Line"]
    #[inline(always)]
    pub fn ewl(&mut self) -> EwlW<'_, IlsSpec> {
        EwlW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status Line"]
    #[inline(always)]
    pub fn bol(&mut self) -> BolW<'_, IlsSpec> {
        BolW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&mut self) -> WdilW<'_, IlsSpec> {
        WdilW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&mut self) -> PealW<'_, IlsSpec> {
        PealW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&mut self) -> PedlW<'_, IlsSpec> {
        PedlW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&mut self) -> AralW<'_, IlsSpec> {
        AralW::new(self, 29)
    }
}
#[doc = "MCAN Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlsSpec;
impl crate::RegisterSpec for IlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ils::R`](R) reader structure"]
impl crate::Readable for IlsSpec {}
#[doc = "`write(|w| ..)` method takes [`ils::W`](W) writer structure"]
impl crate::Writable for IlsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ILS to value 0"]
impl crate::Resettable for IlsSpec {}
