#[doc = "Register `HB16TIME` reader"]
pub type R = crate::R<Hb16timeSpec>;
#[doc = "Register `HB16TIME` writer"]
pub type W = crate::W<Hb16timeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EPI Host-Bus 16 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb16timeSpec;
impl crate::RegisterSpec for Hb16timeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb16time::R`](R) reader structure"]
impl crate::Readable for Hb16timeSpec {}
#[doc = "`write(|w| ..)` method takes [`hb16time::W`](W) writer structure"]
impl crate::Writable for Hb16timeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB16TIME to value 0x0002_2000"]
impl crate::Resettable for Hb16timeSpec {
    const RESET_VALUE: u32 = 0x0002_2000;
}
