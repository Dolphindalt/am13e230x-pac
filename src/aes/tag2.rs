#[doc = "Register `TAG2` reader"]
pub type R = crate::R<Tag2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`tag2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag2Spec;
impl crate::RegisterSpec for Tag2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag2::R`](R) reader structure"]
impl crate::Readable for Tag2Spec {}
#[doc = "`reset()` method sets TAG2 to value 0"]
impl crate::Resettable for Tag2Spec {}
