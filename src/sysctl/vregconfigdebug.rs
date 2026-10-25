#[doc = "Register `VREGCONFIGDEBUG` reader"]
pub type R = crate::R<VregconfigdebugSpec>;
#[doc = "Register `VREGCONFIGDEBUG` writer"]
pub type W = crate::W<VregconfigdebugSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "VREG Configuration Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vregconfigdebug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregconfigdebug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregconfigdebugSpec;
impl crate::RegisterSpec for VregconfigdebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vregconfigdebug::R`](R) reader structure"]
impl crate::Readable for VregconfigdebugSpec {}
#[doc = "`write(|w| ..)` method takes [`vregconfigdebug::W`](W) writer structure"]
impl crate::Writable for VregconfigdebugSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets VREGCONFIGDEBUG to value 0"]
impl crate::Resettable for VregconfigdebugSpec {}
