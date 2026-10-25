#[doc = "Register `QWDPRD` reader"]
pub type R = crate::R<QwdprdSpec>;
#[doc = "Register `QWDPRD` writer"]
pub type W = crate::W<QwdprdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Watchdog Period\n\nYou can [`read`](crate::Reg::read) this register and get [`qwdprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwdprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QwdprdSpec;
impl crate::RegisterSpec for QwdprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qwdprd::R`](R) reader structure"]
impl crate::Readable for QwdprdSpec {}
#[doc = "`write(|w| ..)` method takes [`qwdprd::W`](W) writer structure"]
impl crate::Writable for QwdprdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QWDPRD to value 0"]
impl crate::Resettable for QwdprdSpec {}
