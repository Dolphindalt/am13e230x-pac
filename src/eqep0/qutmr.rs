#[doc = "Register `QUTMR` reader"]
pub type R = crate::R<QutmrSpec>;
#[doc = "Register `QUTMR` writer"]
pub type W = crate::W<QutmrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Unit Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qutmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qutmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QutmrSpec;
impl crate::RegisterSpec for QutmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qutmr::R`](R) reader structure"]
impl crate::Readable for QutmrSpec {}
#[doc = "`write(|w| ..)` method takes [`qutmr::W`](W) writer structure"]
impl crate::Writable for QutmrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QUTMR to value 0"]
impl crate::Resettable for QutmrSpec {}
