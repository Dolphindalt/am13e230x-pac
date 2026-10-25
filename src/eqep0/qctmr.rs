#[doc = "Register `QCTMR` reader"]
pub type R = crate::R<QctmrSpec>;
#[doc = "Register `QCTMR` writer"]
pub type W = crate::W<QctmrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Capture Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qctmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qctmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QctmrSpec;
impl crate::RegisterSpec for QctmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qctmr::R`](R) reader structure"]
impl crate::Readable for QctmrSpec {}
#[doc = "`write(|w| ..)` method takes [`qctmr::W`](W) writer structure"]
impl crate::Writable for QctmrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCTMR to value 0"]
impl crate::Resettable for QctmrSpec {}
