#[doc = "Register `QCPRDLAT` reader"]
pub type R = crate::R<QcprdlatSpec>;
#[doc = "Register `QCPRDLAT` writer"]
pub type W = crate::W<QcprdlatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Capture Period Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qcprdlat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcprdlat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcprdlatSpec;
impl crate::RegisterSpec for QcprdlatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qcprdlat::R`](R) reader structure"]
impl crate::Readable for QcprdlatSpec {}
#[doc = "`write(|w| ..)` method takes [`qcprdlat::W`](W) writer structure"]
impl crate::Writable for QcprdlatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCPRDLAT to value 0"]
impl crate::Resettable for QcprdlatSpec {}
