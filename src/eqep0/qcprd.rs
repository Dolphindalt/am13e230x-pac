#[doc = "Register `QCPRD` reader"]
pub type R = crate::R<QcprdSpec>;
#[doc = "Register `QCPRD` writer"]
pub type W = crate::W<QcprdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Capture Period\n\nYou can [`read`](crate::Reg::read) this register and get [`qcprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcprdSpec;
impl crate::RegisterSpec for QcprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qcprd::R`](R) reader structure"]
impl crate::Readable for QcprdSpec {}
#[doc = "`write(|w| ..)` method takes [`qcprd::W`](W) writer structure"]
impl crate::Writable for QcprdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCPRD to value 0"]
impl crate::Resettable for QcprdSpec {}
