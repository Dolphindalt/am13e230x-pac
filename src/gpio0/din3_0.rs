#[doc = "Register `DIN3_0` reader"]
pub type R = crate::R<Din3_0Spec>;
#[doc = "Field `DIO0` reader - This bit reads the data input value of DIO0."]
pub type Dio0R = crate::BitReader;
#[doc = "Field `DIO1` reader - This bit reads the data input value of DIO1."]
pub type Dio1R = crate::BitReader;
#[doc = "Field `DIO2` reader - This bit reads the data input value of DIO2."]
pub type Dio2R = crate::BitReader;
#[doc = "Field `DIO3` reader - This bit reads the data input value of DIO3."]
pub type Dio3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO0."]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO1."]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO2."]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO3."]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN3_0")
            .field("dio0", &self.dio0())
            .field("dio1", &self.dio1())
            .field("dio2", &self.dio2())
            .field("dio3", &self.dio3())
            .finish()
    }
}
#[doc = "Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din3_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din3_0Spec;
impl crate::RegisterSpec for Din3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din3_0::R`](R) reader structure"]
impl crate::Readable for Din3_0Spec {}
#[doc = "`reset()` method sets DIN3_0 to value 0"]
impl crate::Resettable for Din3_0Spec {}
