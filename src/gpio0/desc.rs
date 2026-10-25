#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Field `MINREV` reader - Minor rev of the IP"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major rev of the IP"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `FEATUREVER` reader - Feature Set for the module *instance*"]
pub type FeatureverR = crate::FieldReader;
#[doc = "Field `MODULEID` reader - Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type ModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor rev of the IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major rev of the IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature Set for the module *instance*"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESC")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .field("featurever", &self.featurever())
            .field("moduleid", &self.moduleid())
            .finish()
    }
}
#[doc = "Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`reset()` method sets DESC to value 0"]
impl crate::Resettable for DescSpec {}
