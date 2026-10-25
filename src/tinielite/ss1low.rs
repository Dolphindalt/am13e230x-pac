#[doc = "Register `SS1LOW` reader"]
pub type R = crate::R<Ss1lowSpec>;
#[doc = "Register `SS1LOW` writer"]
pub type W = crate::W<Ss1lowSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scale and shift register 1, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss1low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss1low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ss1lowSpec;
impl crate::RegisterSpec for Ss1lowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss1low::R`](R) reader structure"]
impl crate::Readable for Ss1lowSpec {}
#[doc = "`write(|w| ..)` method takes [`ss1low::W`](W) writer structure"]
impl crate::Writable for Ss1lowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SS1LOW to value 0"]
impl crate::Resettable for Ss1lowSpec {}
