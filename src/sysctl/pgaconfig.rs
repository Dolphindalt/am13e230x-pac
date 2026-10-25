#[doc = "Register `PGACONFIG` reader"]
pub type R = crate::R<PgaconfigSpec>;
#[doc = "Register `PGACONFIG` writer"]
pub type W = crate::W<PgaconfigSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PGA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgaconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgaconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgaconfigSpec;
impl crate::RegisterSpec for PgaconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgaconfig::R`](R) reader structure"]
impl crate::Readable for PgaconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pgaconfig::W`](W) writer structure"]
impl crate::Writable for PgaconfigSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PGACONFIG to value 0"]
impl crate::Resettable for PgaconfigSpec {}
