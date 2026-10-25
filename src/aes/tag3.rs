#[doc = "Register `TAG3` reader"]
pub type R = crate::R<Tag3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag3Spec;
impl crate::RegisterSpec for Tag3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag3::R`](R) reader structure"]
impl crate::Readable for Tag3Spec {}
#[doc = "`reset()` method sets TAG3 to value 0"]
impl crate::Resettable for Tag3Spec {}
