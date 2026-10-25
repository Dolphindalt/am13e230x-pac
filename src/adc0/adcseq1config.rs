#[doc = "Register `ADCSEQ1CONFIG` reader"]
pub type R = crate::R<Adcseq1configSpec>;
#[doc = "Register `ADCSEQ1CONFIG` writer"]
pub type W = crate::W<Adcseq1configSpec>;
#[doc = "Field `ACQPS` reader - SEQ1 Acquisition Prescale"]
pub type AcqpsR = crate::FieldReader;
#[doc = "Field `ACQPS` writer - SEQ1 Acquisition Prescale"]
pub type AcqpsW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `SAMPCAPRESETDISABLE` reader - SEQ1 Sample Cap Reset Enable"]
pub type SampcapresetdisableR = crate::BitReader;
#[doc = "Field `SAMPCAPRESETDISABLE` writer - SEQ1 Sample Cap Reset Enable"]
pub type SampcapresetdisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPCAPRESETSEL` reader - SEQ1 Sample Cap Reset Select"]
pub type SampcapresetselR = crate::BitReader;
#[doc = "Field `SAMPCAPRESETSEL` writer - SEQ1 Sample Cap Reset Select"]
pub type SampcapresetselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQSTART` reader - Beginning SOC of SEQ1"]
pub type SeqstartR = crate::FieldReader;
#[doc = "Field `SEQSTART` writer - Beginning SOC of SEQ1"]
pub type SeqstartW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `TRIGSEL` reader - SEQ1 Trigger Source Select"]
pub type TrigselR = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - SEQ1 Trigger Source Select"]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SEQSWFRC` reader - SEQ1 Software Force"]
pub type SeqswfrcR = crate::BitReader;
#[doc = "Field `SEQSWFRC` writer - SEQ1 Software Force"]
pub type SeqswfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQENABLE` reader - SEQ1Enable"]
pub type SeqenableR = crate::BitReader;
#[doc = "Field `SEQENABLE` writer - SEQ1Enable"]
pub type SeqenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - SEQ1 Acquisition Prescale"]
    #[inline(always)]
    pub fn acqps(&self) -> AcqpsR {
        AcqpsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - SEQ1 Sample Cap Reset Enable"]
    #[inline(always)]
    pub fn sampcapresetdisable(&self) -> SampcapresetdisableR {
        SampcapresetdisableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEQ1 Sample Cap Reset Select"]
    #[inline(always)]
    pub fn sampcapresetsel(&self) -> SampcapresetselR {
        SampcapresetselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 15:18 - Beginning SOC of SEQ1"]
    #[inline(always)]
    pub fn seqstart(&self) -> SeqstartR {
        SeqstartR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - SEQ1 Trigger Source Select"]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - SEQ1 Software Force"]
    #[inline(always)]
    pub fn seqswfrc(&self) -> SeqswfrcR {
        SeqswfrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SEQ1Enable"]
    #[inline(always)]
    pub fn seqenable(&self) -> SeqenableR {
        SeqenableR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQ1CONFIG")
            .field("acqps", &self.acqps())
            .field("sampcapresetdisable", &self.sampcapresetdisable())
            .field("sampcapresetsel", &self.sampcapresetsel())
            .field("seqstart", &self.seqstart())
            .field("trigsel", &self.trigsel())
            .field("seqswfrc", &self.seqswfrc())
            .field("seqenable", &self.seqenable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SEQ1 Acquisition Prescale"]
    #[inline(always)]
    pub fn acqps(&mut self) -> AcqpsW<'_, Adcseq1configSpec> {
        AcqpsW::new(self, 0)
    }
    #[doc = "Bit 9 - SEQ1 Sample Cap Reset Enable"]
    #[inline(always)]
    pub fn sampcapresetdisable(&mut self) -> SampcapresetdisableW<'_, Adcseq1configSpec> {
        SampcapresetdisableW::new(self, 9)
    }
    #[doc = "Bit 10 - SEQ1 Sample Cap Reset Select"]
    #[inline(always)]
    pub fn sampcapresetsel(&mut self) -> SampcapresetselW<'_, Adcseq1configSpec> {
        SampcapresetselW::new(self, 10)
    }
    #[doc = "Bits 15:18 - Beginning SOC of SEQ1"]
    #[inline(always)]
    pub fn seqstart(&mut self) -> SeqstartW<'_, Adcseq1configSpec> {
        SeqstartW::new(self, 15)
    }
    #[doc = "Bits 20:24 - SEQ1 Trigger Source Select"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<'_, Adcseq1configSpec> {
        TrigselW::new(self, 20)
    }
    #[doc = "Bit 30 - SEQ1 Software Force"]
    #[inline(always)]
    pub fn seqswfrc(&mut self) -> SeqswfrcW<'_, Adcseq1configSpec> {
        SeqswfrcW::new(self, 30)
    }
    #[doc = "Bit 31 - SEQ1Enable"]
    #[inline(always)]
    pub fn seqenable(&mut self) -> SeqenableW<'_, Adcseq1configSpec> {
        SeqenableW::new(self, 31)
    }
}
#[doc = "ADC Sequencer 1 Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq1config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq1config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq1configSpec;
impl crate::RegisterSpec for Adcseq1configSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq1config::R`](R) reader structure"]
impl crate::Readable for Adcseq1configSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq1config::W`](W) writer structure"]
impl crate::Writable for Adcseq1configSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ1CONFIG to value 0x0200"]
impl crate::Resettable for Adcseq1configSpec {
    const RESET_VALUE: u32 = 0x0200;
}
