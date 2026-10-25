#[doc = "Register `ADCSEQ2FIFORESULT` reader"]
pub type R = crate::R<Adcseq2fiforesultSpec>;
#[doc = "Register `ADCSEQ2FIFORESULT` writer"]
pub type W = crate::W<Adcseq2fiforesultSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Sequence 2 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq2fiforesult::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq2fiforesult::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq2fiforesultSpec;
impl crate::RegisterSpec for Adcseq2fiforesultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq2fiforesult::R`](R) reader structure"]
impl crate::Readable for Adcseq2fiforesultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq2fiforesult::W`](W) writer structure"]
impl crate::Writable for Adcseq2fiforesultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ2FIFORESULT to value 0"]
impl crate::Resettable for Adcseq2fiforesultSpec {}
