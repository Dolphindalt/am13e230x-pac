#[doc = "Register `WGHT0LOW` reader"]
pub type R = crate::R<Wght0lowSpec>;
#[doc = "Register `WGHT0LOW` writer"]
pub type W = crate::W<Wght0lowSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Weight register 0, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght0low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght0low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wght0lowSpec;
impl crate::RegisterSpec for Wght0lowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wght0low::R`](R) reader structure"]
impl crate::Readable for Wght0lowSpec {}
#[doc = "`write(|w| ..)` method takes [`wght0low::W`](W) writer structure"]
impl crate::Writable for Wght0lowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WGHT0LOW to value 0"]
impl crate::Resettable for Wght0lowSpec {}
