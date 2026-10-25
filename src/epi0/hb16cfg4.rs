#[doc = "Register `HB16CFG4` reader"]
pub type R = crate::R<Hb16cfg4Spec>;
#[doc = "Register `HB16CFG4` writer"]
pub type W = crate::W<Hb16cfg4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EPI Host-Bus 16 Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb16cfg4Spec;
impl crate::RegisterSpec for Hb16cfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb16cfg4::R`](R) reader structure"]
impl crate::Readable for Hb16cfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`hb16cfg4::W`](W) writer structure"]
impl crate::Writable for Hb16cfg4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB16CFG4 to value 0x0008_0000"]
impl crate::Resettable for Hb16cfg4Spec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
