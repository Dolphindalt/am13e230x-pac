#[doc = "Register `CMPSSDACBUFCONFIG` reader"]
pub type R = crate::R<CmpssdacbufconfigSpec>;
#[doc = "Register `CMPSSDACBUFCONFIG` writer"]
pub type W = crate::W<CmpssdacbufconfigSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Config bits for CMPSS DAC buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpssdacbufconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpssdacbufconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpssdacbufconfigSpec;
impl crate::RegisterSpec for CmpssdacbufconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpssdacbufconfig::R`](R) reader structure"]
impl crate::Readable for CmpssdacbufconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpssdacbufconfig::W`](W) writer structure"]
impl crate::Writable for CmpssdacbufconfigSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPSSDACBUFCONFIG to value 0"]
impl crate::Resettable for CmpssdacbufconfigSpec {}
