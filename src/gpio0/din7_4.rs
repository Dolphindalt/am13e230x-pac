#[doc = "Register `DIN7_4` reader"]
pub type R = crate::R<Din7_4Spec>;
#[doc = "Field `DIO4` reader - This bit reads the data input value of DIO4."]
pub type Dio4R = crate::BitReader;
#[doc = "Field `DIO5` reader - This bit reads the data input value of DIO5."]
pub type Dio5R = crate::BitReader;
#[doc = "Field `DIO6` reader - This bit reads the data input value of DIO6."]
pub type Dio6R = crate::BitReader;
#[doc = "Field `DIO7` reader - This bit reads the data input value of DIO7."]
pub type Dio7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO4."]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO5."]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO6."]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO7."]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN7_4")
            .field("dio4", &self.dio4())
            .field("dio5", &self.dio5())
            .field("dio6", &self.dio6())
            .field("dio7", &self.dio7())
            .finish()
    }
}
#[doc = "Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`din7_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din7_4Spec;
impl crate::RegisterSpec for Din7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din7_4::R`](R) reader structure"]
impl crate::Readable for Din7_4Spec {}
#[doc = "`reset()` method sets DIN7_4 to value 0"]
impl crate::Resettable for Din7_4Spec {}
