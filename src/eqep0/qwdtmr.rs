#[doc = "Register `QWDTMR` reader"]
pub type R = crate::R<QwdtmrSpec>;
#[doc = "Register `QWDTMR` writer"]
pub type W = crate::W<QwdtmrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Watchdog Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qwdtmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwdtmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QwdtmrSpec;
impl crate::RegisterSpec for QwdtmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qwdtmr::R`](R) reader structure"]
impl crate::Readable for QwdtmrSpec {}
#[doc = "`write(|w| ..)` method takes [`qwdtmr::W`](W) writer structure"]
impl crate::Writable for QwdtmrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QWDTMR to value 0"]
impl crate::Resettable for QwdtmrSpec {}
