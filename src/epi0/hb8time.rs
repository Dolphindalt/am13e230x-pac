#[doc = "Register `HB8TIME` reader"]
pub type R = crate::R<Hb8timeSpec>;
#[doc = "Register `HB8TIME` writer"]
pub type W = crate::W<Hb8timeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb8timeSpec;
impl crate::RegisterSpec for Hb8timeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb8time::R`](R) reader structure"]
impl crate::Readable for Hb8timeSpec {}
#[doc = "`write(|w| ..)` method takes [`hb8time::W`](W) writer structure"]
impl crate::Writable for Hb8timeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB8TIME to value 0x0002_2000"]
impl crate::Resettable for Hb8timeSpec {
    const RESET_VALUE: u32 = 0x0002_2000;
}
