#[doc = "Register `HB8CFG3` reader"]
pub type R = crate::R<Hb8cfg3Spec>;
#[doc = "Register `HB8CFG3` writer"]
pub type W = crate::W<Hb8cfg3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EPI Host-Bus 8 Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb8cfg3Spec;
impl crate::RegisterSpec for Hb8cfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb8cfg3::R`](R) reader structure"]
impl crate::Readable for Hb8cfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hb8cfg3::W`](W) writer structure"]
impl crate::Writable for Hb8cfg3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB8CFG3 to value 0x0008_0000"]
impl crate::Resettable for Hb8cfg3Spec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
