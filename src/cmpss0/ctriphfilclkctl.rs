#[doc = "Register `CTRIPHFILCLKCTL` reader"]
pub type R = crate::R<CtriphfilclkctlSpec>;
#[doc = "Register `CTRIPHFILCLKCTL` writer"]
pub type W = crate::W<CtriphfilclkctlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CTRIPH Filter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriphfilclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriphfilclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtriphfilclkctlSpec;
impl crate::RegisterSpec for CtriphfilclkctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctriphfilclkctl::R`](R) reader structure"]
impl crate::Readable for CtriphfilclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctriphfilclkctl::W`](W) writer structure"]
impl crate::Writable for CtriphfilclkctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRIPHFILCLKCTL to value 0"]
impl crate::Resettable for CtriphfilclkctlSpec {}
