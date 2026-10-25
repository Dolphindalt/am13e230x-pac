#[doc = "Register `DIN23_20` reader"]
pub type R = crate::R<Din23_20Spec>;
#[doc = "Field `DIO20` reader - This bit reads the data input value of DIO20."]
pub type Dio20R = crate::BitReader;
#[doc = "Field `DIO21` reader - This bit reads the data input value of DIO21."]
pub type Dio21R = crate::BitReader;
#[doc = "Field `DIO22` reader - This bit reads the data input value of DIO22."]
pub type Dio22R = crate::BitReader;
#[doc = "Field `DIO23` reader - This bit reads the data input value of DIO23."]
pub type Dio23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO20."]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO21."]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO22."]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO23."]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN23_20")
            .field("dio20", &self.dio20())
            .field("dio21", &self.dio21())
            .field("dio22", &self.dio22())
            .field("dio23", &self.dio23())
            .finish()
    }
}
#[doc = "Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`din23_20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din23_20Spec;
impl crate::RegisterSpec for Din23_20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din23_20::R`](R) reader structure"]
impl crate::Readable for Din23_20Spec {}
#[doc = "`reset()` method sets DIN23_20 to value 0"]
impl crate::Resettable for Din23_20Spec {}
