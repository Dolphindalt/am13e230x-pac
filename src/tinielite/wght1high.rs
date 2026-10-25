#[doc = "Register `WGHT1HIGH` reader"]
pub type R = crate::R<Wght1highSpec>;
#[doc = "Register `WGHT1HIGH` writer"]
pub type W = crate::W<Wght1highSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Weight register 1, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght1high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght1high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wght1highSpec;
impl crate::RegisterSpec for Wght1highSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wght1high::R`](R) reader structure"]
impl crate::Readable for Wght1highSpec {}
#[doc = "`write(|w| ..)` method takes [`wght1high::W`](W) writer structure"]
impl crate::Writable for Wght1highSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WGHT1HIGH to value 0"]
impl crate::Resettable for Wght1highSpec {}
