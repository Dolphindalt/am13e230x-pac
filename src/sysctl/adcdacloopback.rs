#[doc = "Register `ADCDACLOOPBACK` reader"]
pub type R = crate::R<AdcdacloopbackSpec>;
#[doc = "Register `ADCDACLOOPBACK` writer"]
pub type W = crate::W<AdcdacloopbackSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Not used in AM13\n\nYou can [`read`](crate::Reg::read) this register and get [`adcdacloopback::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcdacloopback::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcdacloopbackSpec;
impl crate::RegisterSpec for AdcdacloopbackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcdacloopback::R`](R) reader structure"]
impl crate::Readable for AdcdacloopbackSpec {}
#[doc = "`write(|w| ..)` method takes [`adcdacloopback::W`](W) writer structure"]
impl crate::Writable for AdcdacloopbackSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCDACLOOPBACK to value 0"]
impl crate::Resettable for AdcdacloopbackSpec {}
