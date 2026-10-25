#[doc = "Register `RFDATA[%s]` reader"]
pub type R = crate::R<RfdataSpec>;
#[doc = "Register `RFDATA[%s]` writer"]
pub type W = crate::W<RfdataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Register file data.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfdataSpec;
impl crate::RegisterSpec for RfdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdata::R`](R) reader structure"]
impl crate::Readable for RfdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rfdata::W`](W) writer structure"]
impl crate::Writable for RfdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFDATA[%s] to value 0"]
impl crate::Resettable for RfdataSpec {}
