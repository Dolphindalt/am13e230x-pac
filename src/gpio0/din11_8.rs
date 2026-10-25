#[doc = "Register `DIN11_8` reader"]
pub type R = crate::R<Din11_8Spec>;
#[doc = "Field `DIO8` reader - This bit reads the data input value of DIO8."]
pub type Dio8R = crate::BitReader;
#[doc = "Field `DIO9` reader - This bit reads the data input value of DIO9."]
pub type Dio9R = crate::BitReader;
#[doc = "Field `DIO10` reader - This bit reads the data input value of DIO10."]
pub type Dio10R = crate::BitReader;
#[doc = "Field `DIO11` reader - This bit reads the data input value of DIO11."]
pub type Dio11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO8."]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO9."]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO10."]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO11."]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN11_8")
            .field("dio8", &self.dio8())
            .field("dio9", &self.dio9())
            .field("dio10", &self.dio10())
            .field("dio11", &self.dio11())
            .finish()
    }
}
#[doc = "Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`din11_8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din11_8Spec;
impl crate::RegisterSpec for Din11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din11_8::R`](R) reader structure"]
impl crate::Readable for Din11_8Spec {}
#[doc = "`reset()` method sets DIN11_8 to value 0"]
impl crate::Resettable for Din11_8Spec {}
