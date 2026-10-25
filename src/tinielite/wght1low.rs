#[doc = "Register `WGHT1LOW` reader"]
pub type R = crate::R<Wght1lowSpec>;
#[doc = "Register `WGHT1LOW` writer"]
pub type W = crate::W<Wght1lowSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Weight register 1, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght1low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght1low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wght1lowSpec;
impl crate::RegisterSpec for Wght1lowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wght1low::R`](R) reader structure"]
impl crate::Readable for Wght1lowSpec {}
#[doc = "`write(|w| ..)` method takes [`wght1low::W`](W) writer structure"]
impl crate::Writable for Wght1lowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WGHT1LOW to value 0"]
impl crate::Resettable for Wght1lowSpec {}
