#[doc = "Register `TAG0` reader"]
pub type R = crate::R<Tag0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hash result (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`tag0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag0Spec;
impl crate::RegisterSpec for Tag0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag0::R`](R) reader structure"]
impl crate::Readable for Tag0Spec {}
#[doc = "`reset()` method sets TAG0 to value 0"]
impl crate::Resettable for Tag0Spec {}
