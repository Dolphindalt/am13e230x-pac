#[doc = "Register `ADCSEQ4FIFORESULT` reader"]
pub type R = crate::R<Adcseq4fiforesultSpec>;
#[doc = "Register `ADCSEQ4FIFORESULT` writer"]
pub type W = crate::W<Adcseq4fiforesultSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Sequence 4 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq4fiforesult::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq4fiforesult::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq4fiforesultSpec;
impl crate::RegisterSpec for Adcseq4fiforesultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq4fiforesult::R`](R) reader structure"]
impl crate::Readable for Adcseq4fiforesultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq4fiforesult::W`](W) writer structure"]
impl crate::Writable for Adcseq4fiforesultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ4FIFORESULT to value 0"]
impl crate::Resettable for Adcseq4fiforesultSpec {}
