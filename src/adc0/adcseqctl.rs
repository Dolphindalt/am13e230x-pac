#[doc = "Register `ADCSEQCTL` reader"]
pub type R = crate::R<AdcseqctlSpec>;
#[doc = "Register `ADCSEQCTL` writer"]
pub type W = crate::W<AdcseqctlSpec>;
#[doc = "Field `SEQEND` reader - END SOC of SEQ4"]
pub type SeqendR = crate::FieldReader;
#[doc = "Field `SEQEND` writer - END SOC of SEQ4"]
pub type SeqendW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `SEQPREEMPT` reader - SOC Sequnece Preempt and restart setting"]
pub type SeqpreemptR = crate::FieldReader;
#[doc = "Field `SEQPREEMPT` writer - SOC Sequnece Preempt and restart setting"]
pub type SeqpreemptW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - END SOC of SEQ4"]
    #[inline(always)]
    pub fn seqend(&self) -> SeqendR {
        SeqendR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - SOC Sequnece Preempt and restart setting"]
    #[inline(always)]
    pub fn seqpreempt(&self) -> SeqpreemptR {
        SeqpreemptR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQCTL")
            .field("seqend", &self.seqend())
            .field("seqpreempt", &self.seqpreempt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - END SOC of SEQ4"]
    #[inline(always)]
    pub fn seqend(&mut self) -> SeqendW<'_, AdcseqctlSpec> {
        SeqendW::new(self, 0)
    }
    #[doc = "Bits 16:17 - SOC Sequnece Preempt and restart setting"]
    #[inline(always)]
    pub fn seqpreempt(&mut self) -> SeqpreemptW<'_, AdcseqctlSpec> {
        SeqpreemptW::new(self, 16)
    }
}
#[doc = "ADC Sequencer common control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcseqctlSpec;
impl crate::RegisterSpec for AdcseqctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseqctl::R`](R) reader structure"]
impl crate::Readable for AdcseqctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseqctl::W`](W) writer structure"]
impl crate::Writable for AdcseqctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQCTL to value 0"]
impl crate::Resettable for AdcseqctlSpec {}
