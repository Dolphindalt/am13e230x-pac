#[doc = "Register `ERR_STAT2` reader"]
pub type R = crate::R<ErrStat2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MCAN ECC Error Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat2Spec;
impl crate::RegisterSpec for ErrStat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat2::R`](R) reader structure"]
impl crate::Readable for ErrStat2Spec {}
#[doc = "`reset()` method sets ERR_STAT2 to value 0"]
impl crate::Resettable for ErrStat2Spec {}
