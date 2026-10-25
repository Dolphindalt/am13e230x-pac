#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TscvSpec>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TscvSpec>;
#[doc = "Field `TSC` reader - Timestamp Counter"]
pub type TscR = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp Counter"]
pub type TscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TscR {
        TscR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCV").field("tsc", &self.tsc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&mut self) -> TscW<'_, TscvSpec> {
        TscW::new(self, 0)
    }
}
#[doc = "MCAN Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscvSpec;
impl crate::RegisterSpec for TscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TscvSpec {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TscvSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TscvSpec {}
