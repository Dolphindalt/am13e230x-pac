#[doc = "Register `DIN19_16` reader"]
pub type R = crate::R<Din19_16Spec>;
#[doc = "Field `DIO16` reader - This bit reads the data input value of DIO16."]
pub type Dio16R = crate::BitReader;
#[doc = "Field `DIO17` reader - This bit reads the data input value of DIO17."]
pub type Dio17R = crate::BitReader;
#[doc = "Field `DIO18` reader - This bit reads the data input value of DIO18."]
pub type Dio18R = crate::BitReader;
#[doc = "Field `DIO19` reader - This bit reads the data input value of DIO19."]
pub type Dio19R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO16."]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO17."]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO18."]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO19."]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN19_16")
            .field("dio16", &self.dio16())
            .field("dio17", &self.dio17())
            .field("dio18", &self.dio18())
            .field("dio19", &self.dio19())
            .finish()
    }
}
#[doc = "Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`din19_16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din19_16Spec;
impl crate::RegisterSpec for Din19_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din19_16::R`](R) reader structure"]
impl crate::Readable for Din19_16Spec {}
#[doc = "`reset()` method sets DIN19_16 to value 0"]
impl crate::Resettable for Din19_16Spec {}
