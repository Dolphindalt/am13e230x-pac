#[doc = "Register `QPOSLAT` reader"]
pub type R = crate::R<QposlatSpec>;
#[doc = "Register `QPOSLAT` writer"]
pub type W = crate::W<QposlatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Position Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qposlat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposlat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposlatSpec;
impl crate::RegisterSpec for QposlatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposlat::R`](R) reader structure"]
impl crate::Readable for QposlatSpec {}
#[doc = "`write(|w| ..)` method takes [`qposlat::W`](W) writer structure"]
impl crate::Writable for QposlatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSLAT to value 0"]
impl crate::Resettable for QposlatSpec {}
