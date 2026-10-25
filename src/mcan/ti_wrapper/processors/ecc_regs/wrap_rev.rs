#[doc = "Register `WRAP_REV` reader"]
pub type R = crate::R<WrapRevSpec>;
#[doc = "Field `REVMIN` reader - Minor Revision"]
pub type RevminR = crate::FieldReader;
#[doc = "Field `REVMAJ` reader - Major Revision"]
pub type RevmajR = crate::FieldReader;
#[doc = "Field `MODULE_ID` reader - Module Identification Number"]
pub type ModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `SCHEME` reader - PID Register Scheme"]
pub type SchemeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Minor Revision"]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision"]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 8) & 7) as u8)
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
        f.debug_struct("WRAP_REV")
            .field("revmin", &self.revmin())
            .field("revmaj", &self.revmaj())
            .field("module_id", &self.module_id())
            .field("scheme", &self.scheme())
            .finish()
    }
}
#[doc = "MCAN ECC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_rev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrapRevSpec;
impl crate::RegisterSpec for WrapRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrap_rev::R`](R) reader structure"]
impl crate::Readable for WrapRevSpec {}
#[doc = "`reset()` method sets WRAP_REV to value 0x66a4_6a02"]
impl crate::Resettable for WrapRevSpec {
    const RESET_VALUE: u32 = 0x66a4_6a02;
}
