#[doc = "Register `QPOSCNT` reader"]
pub type R = crate::R<QposcntSpec>;
#[doc = "Register `QPOSCNT` writer"]
pub type W = crate::W<QposcntSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Position Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`qposcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposcntSpec;
impl crate::RegisterSpec for QposcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposcnt::R`](R) reader structure"]
impl crate::Readable for QposcntSpec {}
#[doc = "`write(|w| ..)` method takes [`qposcnt::W`](W) writer structure"]
impl crate::Writable for QposcntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSCNT to value 0"]
impl crate::Resettable for QposcntSpec {}
