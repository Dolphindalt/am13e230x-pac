#[doc = "Register `TSCTR` reader"]
pub type R = crate::R<TsctrSpec>;
#[doc = "Register `TSCTR` writer"]
pub type W = crate::W<TsctrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Time-Stamp Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsctrSpec;
impl crate::RegisterSpec for TsctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsctr::R`](R) reader structure"]
impl crate::Readable for TsctrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsctr::W`](W) writer structure"]
impl crate::Writable for TsctrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSCTR to value 0"]
impl crate::Resettable for TsctrSpec {}
