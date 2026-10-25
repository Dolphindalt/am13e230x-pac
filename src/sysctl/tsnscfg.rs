#[doc = "Register `TSNSCFG` reader"]
pub type R = crate::R<TsnscfgSpec>;
#[doc = "Register `TSNSCFG` writer"]
pub type W = crate::W<TsnscfgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Temperature Sensor Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsnscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsnscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsnscfgSpec;
impl crate::RegisterSpec for TsnscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsnscfg::R`](R) reader structure"]
impl crate::Readable for TsnscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tsnscfg::W`](W) writer structure"]
impl crate::Writable for TsnscfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSNSCFG to value 0"]
impl crate::Resettable for TsnscfgSpec {}
