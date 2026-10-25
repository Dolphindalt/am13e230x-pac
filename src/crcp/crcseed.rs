#[doc = "Register `CRCSEED` writer"]
pub type W = crate::W<CrcseedSpec>;
#[doc = "Field `SEED` writer - Seed Data"]
pub type SeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CrcseedSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Seed Data"]
    #[inline(always)]
    pub fn seed(&mut self) -> SeedW<'_, CrcseedSpec> {
        SeedW::new(self, 0)
    }
}
#[doc = "CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcseed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcseedSpec;
impl crate::RegisterSpec for CrcseedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcseed::W`](W) writer structure"]
impl crate::Writable for CrcseedSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCSEED to value 0"]
impl crate::Resettable for CrcseedSpec {}
