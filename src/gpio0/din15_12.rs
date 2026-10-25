#[doc = "Register `DIN15_12` reader"]
pub type R = crate::R<Din15_12Spec>;
#[doc = "Field `DIO12` reader - This bit reads the data input value of DIO12."]
pub type Dio12R = crate::BitReader;
#[doc = "Field `DIO13` reader - This bit reads the data input value of DIO13."]
pub type Dio13R = crate::BitReader;
#[doc = "Field `DIO14` reader - This bit reads the data input value of DIO14."]
pub type Dio14R = crate::BitReader;
#[doc = "Field `DIO15` reader - This bit reads the data input value of DIO15."]
pub type Dio15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO12."]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO13."]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO14."]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO15."]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN15_12")
            .field("dio12", &self.dio12())
            .field("dio13", &self.dio13())
            .field("dio14", &self.dio14())
            .field("dio15", &self.dio15())
            .finish()
    }
}
#[doc = "Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`din15_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din15_12Spec;
impl crate::RegisterSpec for Din15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din15_12::R`](R) reader structure"]
impl crate::Readable for Din15_12Spec {}
#[doc = "`reset()` method sets DIN15_12 to value 0"]
impl crate::Resettable for Din15_12Spec {}
