#[doc = "Register `CPSV` reader"]
pub type R = crate::R<CpsvSpec>;
#[doc = "Field `CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsval(&self) -> CpsvalR {
        CpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSV")
            .field("cpsval", &self.cpsval())
            .finish()
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsvSpec;
impl crate::RegisterSpec for CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsv::R`](R) reader structure"]
impl crate::Readable for CpsvSpec {}
#[doc = "`reset()` method sets CPSV to value 0"]
impl crate::Resettable for CpsvSpec {}
