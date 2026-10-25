#[doc = "Register `OUTPUTXBARSTATUS` reader"]
pub type R = crate::R<OutputxbarstatusSpec>;
#[doc = "Register `OUTPUTXBARSTATUS` writer"]
pub type W = crate::W<OutputxbarstatusSpec>;
#[doc = "Field `STS` reader - Output Signal Status"]
pub type StsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output Signal Status"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARSTATUS")
            .field("sts", &self.sts())
            .finish()
    }
}
impl W {}
#[doc = "Output Signal Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarstatusSpec;
impl crate::RegisterSpec for OutputxbarstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarstatus::R`](R) reader structure"]
impl crate::Readable for OutputxbarstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarstatus::W`](W) writer structure"]
impl crate::Writable for OutputxbarstatusSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARSTATUS to value 0"]
impl crate::Resettable for OutputxbarstatusSpec {}
