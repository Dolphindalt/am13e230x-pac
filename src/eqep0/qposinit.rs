#[doc = "Register `QPOSINIT` reader"]
pub type R = crate::R<QposinitSpec>;
#[doc = "Register `QPOSINIT` writer"]
pub type W = crate::W<QposinitSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Position Counter Init\n\nYou can [`read`](crate::Reg::read) this register and get [`qposinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposinitSpec;
impl crate::RegisterSpec for QposinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposinit::R`](R) reader structure"]
impl crate::Readable for QposinitSpec {}
#[doc = "`write(|w| ..)` method takes [`qposinit::W`](W) writer structure"]
impl crate::Writable for QposinitSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSINIT to value 0"]
impl crate::Resettable for QposinitSpec {}
