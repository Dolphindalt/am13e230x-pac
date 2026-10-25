#[doc = "Register `PGALOCK` reader"]
pub type R = crate::R<PgalockSpec>;
#[doc = "Register `PGALOCK` writer"]
pub type W = crate::W<PgalockSpec>;
#[doc = "Field `PGACTL` writer - Lock bit for PGACTL."]
pub type PgactlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXSEL` writer - Lock bit for MUXSEL"]
pub type MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGATMCTL` writer - Lock but for PGATMCTL"]
pub type PgatmctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGALOCK").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock bit for PGACTL."]
    #[inline(always)]
    pub fn pgactl(&mut self) -> PgactlW<'_, PgalockSpec> {
        PgactlW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock bit for MUXSEL"]
    #[inline(always)]
    pub fn muxsel(&mut self) -> MuxselW<'_, PgalockSpec> {
        MuxselW::new(self, 1)
    }
    #[doc = "Bit 3 - Lock but for PGATMCTL"]
    #[inline(always)]
    pub fn pgatmctl(&mut self) -> PgatmctlW<'_, PgalockSpec> {
        PgatmctlW::new(self, 3)
    }
}
#[doc = "PGA Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgalock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgalock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgalockSpec;
impl crate::RegisterSpec for PgalockSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgalock::R`](R) reader structure"]
impl crate::Readable for PgalockSpec {}
#[doc = "`write(|w| ..)` method takes [`pgalock::W`](W) writer structure"]
impl crate::Writable for PgalockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PGALOCK to value 0"]
impl crate::Resettable for PgalockSpec {}
