#[doc = "Register `HB8CFG2` reader"]
pub type R = crate::R<Hb8cfg2Spec>;
#[doc = "Register `HB8CFG2` writer"]
pub type W = crate::W<Hb8cfg2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EPI Host-Bus 8 Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb8cfg2Spec;
impl crate::RegisterSpec for Hb8cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb8cfg2::R`](R) reader structure"]
impl crate::Readable for Hb8cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`hb8cfg2::W`](W) writer structure"]
impl crate::Writable for Hb8cfg2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB8CFG2 to value 0"]
impl crate::Resettable for Hb8cfg2Spec {}
