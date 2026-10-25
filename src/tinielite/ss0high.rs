#[doc = "Register `SS0HIGH` reader"]
pub type R = crate::R<Ss0highSpec>;
#[doc = "Register `SS0HIGH` writer"]
pub type W = crate::W<Ss0highSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scale and shift register 0, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss0high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss0high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ss0highSpec;
impl crate::RegisterSpec for Ss0highSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss0high::R`](R) reader structure"]
impl crate::Readable for Ss0highSpec {}
#[doc = "`write(|w| ..)` method takes [`ss0high::W`](W) writer structure"]
impl crate::Writable for Ss0highSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SS0HIGH to value 0"]
impl crate::Resettable for Ss0highSpec {}
