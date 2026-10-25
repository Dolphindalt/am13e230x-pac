#[doc = "Register `RXD` reader"]
pub type R = crate::R<RxdSpec>;
#[doc = "Register `RXD` writer"]
pub type W = crate::W<RxdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdSpec;
impl crate::RegisterSpec for RxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RxdSpec {}
#[doc = "`write(|w| ..)` method takes [`rxd::W`](W) writer structure"]
impl crate::Writable for RxdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RxdSpec {}
