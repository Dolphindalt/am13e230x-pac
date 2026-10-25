#[doc = "Register `DIN27_24` reader"]
pub type R = crate::R<Din27_24Spec>;
#[doc = "Field `DIO24` reader - This bit reads the data input value of DIO24."]
pub type Dio24R = crate::BitReader;
#[doc = "Field `DIO25` reader - This bit reads the data input value of DIO25."]
pub type Dio25R = crate::BitReader;
#[doc = "Field `DIO26` reader - This bit reads the data input value of DIO26."]
pub type Dio26R = crate::BitReader;
#[doc = "Field `DIO27` reader - This bit reads the data input value of DIO27."]
pub type Dio27R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO24."]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO25."]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO26."]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO27."]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN27_24")
            .field("dio24", &self.dio24())
            .field("dio25", &self.dio25())
            .field("dio26", &self.dio26())
            .field("dio27", &self.dio27())
            .finish()
    }
}
#[doc = "Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`din27_24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din27_24Spec;
impl crate::RegisterSpec for Din27_24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din27_24::R`](R) reader structure"]
impl crate::Readable for Din27_24Spec {}
#[doc = "`reset()` method sets DIN27_24 to value 0"]
impl crate::Resettable for Din27_24Spec {}
