#[doc = "Register `DATAOUT` reader"]
pub type R = crate::R<DataoutSpec>;
#[doc = "Register `DATAOUT` writer"]
pub type W = crate::W<DataoutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Data registers, used to store data to memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSpec;
impl crate::RegisterSpec for DataoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataout::R`](R) reader structure"]
impl crate::Readable for DataoutSpec {}
#[doc = "`write(|w| ..)` method takes [`dataout::W`](W) writer structure"]
impl crate::Writable for DataoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUT to value 0"]
impl crate::Resettable for DataoutSpec {}
