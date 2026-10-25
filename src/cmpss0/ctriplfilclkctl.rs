#[doc = "Register `CTRIPLFILCLKCTL` reader"]
pub type R = crate::R<CtriplfilclkctlSpec>;
#[doc = "Register `CTRIPLFILCLKCTL` writer"]
pub type W = crate::W<CtriplfilclkctlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CTRIPL Filter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriplfilclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriplfilclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtriplfilclkctlSpec;
impl crate::RegisterSpec for CtriplfilclkctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctriplfilclkctl::R`](R) reader structure"]
impl crate::Readable for CtriplfilclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctriplfilclkctl::W`](W) writer structure"]
impl crate::Writable for CtriplfilclkctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRIPLFILCLKCTL to value 0"]
impl crate::Resettable for CtriplfilclkctlSpec {}
