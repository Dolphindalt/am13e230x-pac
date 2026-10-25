#[doc = "Register `QPOSILAT` reader"]
pub type R = crate::R<QposilatSpec>;
#[doc = "Register `QPOSILAT` writer"]
pub type W = crate::W<QposilatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Index Position Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qposilat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposilat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposilatSpec;
impl crate::RegisterSpec for QposilatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposilat::R`](R) reader structure"]
impl crate::Readable for QposilatSpec {}
#[doc = "`write(|w| ..)` method takes [`qposilat::W`](W) writer structure"]
impl crate::Writable for QposilatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSILAT to value 0"]
impl crate::Resettable for QposilatSpec {}
