#[doc = "Register `DATAIN[%s]` reader"]
pub type R = crate::R<DatainSpec>;
#[doc = "Register `DATAIN[%s]` writer"]
pub type W = crate::W<DatainSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Data registers, used to load data from memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatainSpec;
impl crate::RegisterSpec for DatainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datain::R`](R) reader structure"]
impl crate::Readable for DatainSpec {}
#[doc = "`write(|w| ..)` method takes [`datain::W`](W) writer structure"]
impl crate::Writable for DatainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAIN[%s] to value 0"]
impl crate::Resettable for DatainSpec {}
