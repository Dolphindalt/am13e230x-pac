#[doc = "Register `PID` reader"]
pub type R = crate::R<PidSpec>;
#[doc = "Field `MINOR` reader - Minor Revision"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Major Revision"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MODULE_ID` reader - Module Identification Number"]
pub type ModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `SCHEME` reader - PID Register Scheme"]
pub type SchemeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Minor Revision"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Module Identification Number"]
    #[inline(always)]
    pub fn module_id(&self) -> ModuleIdR {
        ModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - PID Register Scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID")
            .field("minor", &self.minor())
            .field("major", &self.major())
            .field("module_id", &self.module_id())
            .field("scheme", &self.scheme())
            .finish()
    }
}
#[doc = "MCAN Subsystem Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidSpec;
impl crate::RegisterSpec for PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PidSpec {}
#[doc = "`reset()` method sets PID to value 0x68e0_4901"]
impl crate::Resettable for PidSpec {
    const RESET_VALUE: u32 = 0x68e0_4901;
}
