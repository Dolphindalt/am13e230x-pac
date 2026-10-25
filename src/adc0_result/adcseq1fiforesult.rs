#[doc = "Register `ADCSEQ1FIFORESULT` reader"]
pub type R = crate::R<Adcseq1fiforesultSpec>;
#[doc = "Register `ADCSEQ1FIFORESULT` writer"]
pub type W = crate::W<Adcseq1fiforesultSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Sequence 1 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq1fiforesult::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq1fiforesult::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq1fiforesultSpec;
impl crate::RegisterSpec for Adcseq1fiforesultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq1fiforesult::R`](R) reader structure"]
impl crate::Readable for Adcseq1fiforesultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq1fiforesult::W`](W) writer structure"]
impl crate::Writable for Adcseq1fiforesultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ1FIFORESULT to value 0"]
impl crate::Resettable for Adcseq1fiforesultSpec {}
