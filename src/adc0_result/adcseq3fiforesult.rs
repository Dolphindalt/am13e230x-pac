#[doc = "Register `ADCSEQ3FIFORESULT` reader"]
pub type R = crate::R<Adcseq3fiforesultSpec>;
#[doc = "Register `ADCSEQ3FIFORESULT` writer"]
pub type W = crate::W<Adcseq3fiforesultSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Sequence 3 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq3fiforesult::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq3fiforesult::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq3fiforesultSpec;
impl crate::RegisterSpec for Adcseq3fiforesultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq3fiforesult::R`](R) reader structure"]
impl crate::Readable for Adcseq3fiforesultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq3fiforesult::W`](W) writer structure"]
impl crate::Writable for Adcseq3fiforesultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ3FIFORESULT to value 0"]
impl crate::Resettable for Adcseq3fiforesultSpec {}
