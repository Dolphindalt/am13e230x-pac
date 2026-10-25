#[doc = "Register `RC` reader"]
pub type R = crate::R<RcSpec>;
#[doc = "Field `RC` reader - Repeat Counter Value"]
pub type RcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter Value"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC").field("rc", &self.rc()).finish()
    }
}
#[doc = "Repeat counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcSpec;
impl crate::RegisterSpec for RcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc::R`](R) reader structure"]
impl crate::Readable for RcSpec {}
#[doc = "`reset()` method sets RC to value 0"]
impl crate::Resettable for RcSpec {}
