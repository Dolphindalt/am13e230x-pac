#[doc = "Register `DIN31_0` reader"]
pub type R = crate::R<Din31_0Spec>;
#[doc = "Field `DIO0` reader - This bit reads the data input value of DIO0."]
pub type Dio0R = crate::BitReader;
#[doc = "Field `DIO1` reader - This bit reads the data input value of DIO1."]
pub type Dio1R = crate::BitReader;
#[doc = "Field `DIO2` reader - This bit reads the data input value of DIO2."]
pub type Dio2R = crate::BitReader;
#[doc = "Field `DIO3` reader - This bit reads the data input value of DIO3."]
pub type Dio3R = crate::BitReader;
#[doc = "Field `DIO4` reader - This bit reads the data input value of DIO4."]
pub type Dio4R = crate::BitReader;
#[doc = "Field `DIO5` reader - This bit reads the data input value of DIO5."]
pub type Dio5R = crate::BitReader;
#[doc = "Field `DIO6` reader - This bit reads the data input value of DIO6."]
pub type Dio6R = crate::BitReader;
#[doc = "Field `DIO7` reader - This bit reads the data input value of DIO7."]
pub type Dio7R = crate::BitReader;
#[doc = "Field `DIO8` reader - This bit reads the data input value of DIO8."]
pub type Dio8R = crate::BitReader;
#[doc = "Field `DIO9` reader - This bit reads the data input value of DIO9."]
pub type Dio9R = crate::BitReader;
#[doc = "Field `DIO10` reader - This bit reads the data input value of DIO10."]
pub type Dio10R = crate::BitReader;
#[doc = "Field `DIO11` reader - This bit reads the data input value of DIO11."]
pub type Dio11R = crate::BitReader;
#[doc = "Field `DIO12` reader - This bit reads the data input value of DIO12."]
pub type Dio12R = crate::BitReader;
#[doc = "Field `DIO13` reader - This bit reads the data input value of DIO13."]
pub type Dio13R = crate::BitReader;
#[doc = "Field `DIO14` reader - This bit reads the data input value of DIO14."]
pub type Dio14R = crate::BitReader;
#[doc = "Field `DIO15` reader - This bit reads the data input value of DIO15."]
pub type Dio15R = crate::BitReader;
#[doc = "Field `DIO16` reader - This bit reads the data input value of DIO16."]
pub type Dio16R = crate::BitReader;
#[doc = "Field `DIO17` reader - This bit reads the data input value of DIO17."]
pub type Dio17R = crate::BitReader;
#[doc = "Field `DIO18` reader - This bit reads the data input value of DIO18."]
pub type Dio18R = crate::BitReader;
#[doc = "Field `DIO19` reader - This bit reads the data input value of DIO19."]
pub type Dio19R = crate::BitReader;
#[doc = "Field `DIO20` reader - This bit reads the data input value of DIO20."]
pub type Dio20R = crate::BitReader;
#[doc = "Field `DIO21` reader - This bit reads the data input value of DIO21."]
pub type Dio21R = crate::BitReader;
#[doc = "Field `DIO22` reader - This bit reads the data input value of DIO22."]
pub type Dio22R = crate::BitReader;
#[doc = "Field `DIO23` reader - This bit reads the data input value of DIO23."]
pub type Dio23R = crate::BitReader;
#[doc = "Field `DIO24` reader - This bit reads the data input value of DIO24."]
pub type Dio24R = crate::BitReader;
#[doc = "Field `DIO25` reader - This bit reads the data input value of DIO25."]
pub type Dio25R = crate::BitReader;
#[doc = "Field `DIO26` reader - This bit reads the data input value of DIO26."]
pub type Dio26R = crate::BitReader;
#[doc = "Field `DIO27` reader - This bit reads the data input value of DIO27."]
pub type Dio27R = crate::BitReader;
#[doc = "Field `DIO28` reader - This bit reads the data input value of DIO28."]
pub type Dio28R = crate::BitReader;
#[doc = "Field `DIO29` reader - This bit reads the data input value of DIO29."]
pub type Dio29R = crate::BitReader;
#[doc = "Field `DIO30` reader - This bit reads the data input value of DIO30."]
pub type Dio30R = crate::BitReader;
#[doc = "Field `DIO31` reader - This bit reads the data input value of DIO31."]
pub type Dio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO0."]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit reads the data input value of DIO1."]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit reads the data input value of DIO2."]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit reads the data input value of DIO3."]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit reads the data input value of DIO4."]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit reads the data input value of DIO5."]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit reads the data input value of DIO6."]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit reads the data input value of DIO7."]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO8."]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit reads the data input value of DIO9."]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit reads the data input value of DIO10."]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit reads the data input value of DIO11."]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit reads the data input value of DIO12."]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit reads the data input value of DIO13."]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit reads the data input value of DIO14."]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit reads the data input value of DIO15."]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO16."]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit reads the data input value of DIO17."]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit reads the data input value of DIO18."]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit reads the data input value of DIO19."]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit reads the data input value of DIO20."]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit reads the data input value of DIO21."]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit reads the data input value of DIO22."]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit reads the data input value of DIO23."]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO24."]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit reads the data input value of DIO25."]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit reads the data input value of DIO26."]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit reads the data input value of DIO27."]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit reads the data input value of DIO28."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit reads the data input value of DIO29."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit reads the data input value of DIO30."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit reads the data input value of DIO31."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN31_0")
            .field("dio0", &self.dio0())
            .field("dio1", &self.dio1())
            .field("dio2", &self.dio2())
            .field("dio3", &self.dio3())
            .field("dio4", &self.dio4())
            .field("dio5", &self.dio5())
            .field("dio6", &self.dio6())
            .field("dio7", &self.dio7())
            .field("dio8", &self.dio8())
            .field("dio9", &self.dio9())
            .field("dio10", &self.dio10())
            .field("dio11", &self.dio11())
            .field("dio12", &self.dio12())
            .field("dio13", &self.dio13())
            .field("dio14", &self.dio14())
            .field("dio15", &self.dio15())
            .field("dio16", &self.dio16())
            .field("dio17", &self.dio17())
            .field("dio18", &self.dio18())
            .field("dio19", &self.dio19())
            .field("dio20", &self.dio20())
            .field("dio21", &self.dio21())
            .field("dio22", &self.dio22())
            .field("dio23", &self.dio23())
            .field("dio24", &self.dio24())
            .field("dio25", &self.dio25())
            .field("dio26", &self.dio26())
            .field("dio27", &self.dio27())
            .field("dio28", &self.dio28())
            .field("dio29", &self.dio29())
            .field("dio30", &self.dio30())
            .field("dio31", &self.dio31())
            .finish()
    }
}
#[doc = "Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din31_0Spec;
impl crate::RegisterSpec for Din31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din31_0::R`](R) reader structure"]
impl crate::Readable for Din31_0Spec {}
#[doc = "`reset()` method sets DIN31_0 to value 0"]
impl crate::Resettable for Din31_0Spec {}
