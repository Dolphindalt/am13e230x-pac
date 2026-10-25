#[doc = "Register `QCTMRLAT` reader"]
pub type R = crate::R<QctmrlatSpec>;
#[doc = "Register `QCTMRLAT` writer"]
pub type W = crate::W<QctmrlatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Capture Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qctmrlat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qctmrlat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QctmrlatSpec;
impl crate::RegisterSpec for QctmrlatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qctmrlat::R`](R) reader structure"]
impl crate::Readable for QctmrlatSpec {}
#[doc = "`write(|w| ..)` method takes [`qctmrlat::W`](W) writer structure"]
impl crate::Writable for QctmrlatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCTMRLAT to value 0"]
impl crate::Resettable for QctmrlatSpec {}
