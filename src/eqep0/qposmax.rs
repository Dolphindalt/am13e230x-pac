#[doc = "Register `QPOSMAX` reader"]
pub type R = crate::R<QposmaxSpec>;
#[doc = "Register `QPOSMAX` writer"]
pub type W = crate::W<QposmaxSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Maximum Position Count\n\nYou can [`read`](crate::Reg::read) this register and get [`qposmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposmaxSpec;
impl crate::RegisterSpec for QposmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposmax::R`](R) reader structure"]
impl crate::Readable for QposmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`qposmax::W`](W) writer structure"]
impl crate::Writable for QposmaxSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSMAX to value 0"]
impl crate::Resettable for QposmaxSpec {}
