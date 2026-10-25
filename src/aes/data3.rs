#[doc = "Register `DATA3` reader"]
pub type R = crate::R<Data3Spec>;
#[doc = "Register `DATA3` writer"]
pub type W = crate::W<Data3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data input (LSW) / Data output (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3Spec;
impl crate::RegisterSpec for Data3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3::R`](R) reader structure"]
impl crate::Readable for Data3Spec {}
#[doc = "`write(|w| ..)` method takes [`data3::W`](W) writer structure"]
impl crate::Writable for Data3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for Data3Spec {}
