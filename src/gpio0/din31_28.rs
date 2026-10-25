#[doc = "Register `DIN31_28` reader"]
pub type R = crate::R<Din31_28Spec>;
#[doc = "Field `DIO28` reader - This bit reads the data input value of DIO28."]
pub type Dio28R = crate::BitReader;
#[doc = "Field `DIO29` reader - This bit reads the data input value of DIO29."]
pub type Dio29R = crate::BitReader;
#[doc = "Field `DIO30` reader - This bit reads the data input value of DIO30."]
pub type Dio30R = crate::BitReader;
#[doc = "Field `DIO31` reader - This bit reads the data input value of DIO31."]
pub type Dio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO28."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO29."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO30."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO31."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN31_28")
            .field("dio28", &self.dio28())
            .field("dio29", &self.dio29())
            .field("dio30", &self.dio30())
            .field("dio31", &self.dio31())
            .finish()
    }
}
#[doc = "Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din31_28Spec;
impl crate::RegisterSpec for Din31_28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din31_28::R`](R) reader structure"]
impl crate::Readable for Din31_28Spec {}
#[doc = "`reset()` method sets DIN31_28 to value 0"]
impl crate::Resettable for Din31_28Spec {}
