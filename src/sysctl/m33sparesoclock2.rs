#[doc = "Register `M33SPARESOCLOCK2` reader"]
pub type R = crate::R<M33sparesoclock2Spec>;
#[doc = "Register `M33SPARESOCLOCK2` writer"]
pub type W = crate::W<M33sparesoclock2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M33C1 Spare SOC LOCK Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`m33sparesoclock2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33sparesoclock2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M33sparesoclock2Spec;
impl crate::RegisterSpec for M33sparesoclock2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m33sparesoclock2::R`](R) reader structure"]
impl crate::Readable for M33sparesoclock2Spec {}
#[doc = "`write(|w| ..)` method takes [`m33sparesoclock2::W`](W) writer structure"]
impl crate::Writable for M33sparesoclock2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets M33SPARESOCLOCK2 to value 0"]
impl crate::Resettable for M33sparesoclock2Spec {}
