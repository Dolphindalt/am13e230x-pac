#[doc = "Register `PMULDOSPARECTL` reader"]
pub type R = crate::R<PmuldosparectlSpec>;
#[doc = "Register `PMULDOSPARECTL` writer"]
pub type W = crate::W<PmuldosparectlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LDO Spare Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuldosparectl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuldosparectl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuldosparectlSpec;
impl crate::RegisterSpec for PmuldosparectlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuldosparectl::R`](R) reader structure"]
impl crate::Readable for PmuldosparectlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmuldosparectl::W`](W) writer structure"]
impl crate::Writable for PmuldosparectlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PMULDOSPARECTL to value 0"]
impl crate::Resettable for PmuldosparectlSpec {}
