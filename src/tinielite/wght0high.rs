#[doc = "Register `WGHT0HIGH` reader"]
pub type R = crate::R<Wght0highSpec>;
#[doc = "Register `WGHT0HIGH` writer"]
pub type W = crate::W<Wght0highSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Weight register 0, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght0high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght0high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wght0highSpec;
impl crate::RegisterSpec for Wght0highSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wght0high::R`](R) reader structure"]
impl crate::Readable for Wght0highSpec {}
#[doc = "`write(|w| ..)` method takes [`wght0high::W`](W) writer structure"]
impl crate::Writable for Wght0highSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WGHT0HIGH to value 0"]
impl crate::Resettable for Wght0highSpec {}
