#[doc = "Register `DOUTTGL31_0` writer"]
pub type W = crate::W<Douttgl31_0Spec>;
#[doc = "Field `DIO0` writer - This bit is used to toggle DIO0 output."]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO1` writer - This bit is used to toggle DIO1 output."]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO2` writer - This bit is used to toggle DIO2 output."]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO3` writer - This bit is used to toggle DIO3 output."]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO4` writer - This bit is used to toggle DIO4 output."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO5` writer - This bit is used to toggle DIO5 output."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO6` writer - This bit is used to toggle DIO6 output."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO7` writer - This bit is used to toggle DIO7 output."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO8` writer - This bit is used to toggle DIO8 output."]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO9` writer - This bit is used to toggle DIO9 output."]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO10` writer - This bit is used to toggle DIO10 output."]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO11` writer - This bit is used to toggle DIO11 output."]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO12` writer - This bit is used to toggle DIO12 output."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO13` writer - This bit is used to toggle DIO13 output."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO14` writer - This bit is used to toggle DIO14 output."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO15` writer - This bit is used to toggle DIO15 output."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO16` writer - This bit is used to toggle DIO16 output."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO17` writer - This bit is used to toggle DIO17 output."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO18` writer - This bit is used to toggle DIO18 output."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO19` writer - This bit is used to toggle DIO19 output."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO20` writer - This bit is used to toggle DIO20 output."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO21` writer - This bit is used to toggle DIO21 output."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO22` writer - This bit is used to toggle DIO22 output."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO23` writer - This bit is used to toggle DIO23 output."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO24` writer - This bit is used to toggle DIO24 output."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO25` writer - This bit is used to toggle DIO25 output."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO26` writer - This bit is used to toggle DIO26 output."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO27` writer - This bit is used to toggle DIO27 output."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO28` writer - This bit is used to toggle DIO28 output."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO29` writer - This bit is used to toggle DIO29 output."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO30` writer - This bit is used to toggle DIO30 output."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO31` writer - This bit is used to toggle DIO31 output."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Douttgl31_0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to toggle DIO0 output."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, Douttgl31_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to toggle DIO1 output."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, Douttgl31_0Spec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to toggle DIO2 output."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, Douttgl31_0Spec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to toggle DIO3 output."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, Douttgl31_0Spec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is used to toggle DIO4 output."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, Douttgl31_0Spec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is used to toggle DIO5 output."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, Douttgl31_0Spec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is used to toggle DIO6 output."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, Douttgl31_0Spec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is used to toggle DIO7 output."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, Douttgl31_0Spec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is used to toggle DIO8 output."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, Douttgl31_0Spec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is used to toggle DIO9 output."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, Douttgl31_0Spec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit is used to toggle DIO10 output."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, Douttgl31_0Spec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is used to toggle DIO11 output."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, Douttgl31_0Spec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is used to toggle DIO12 output."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, Douttgl31_0Spec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit is used to toggle DIO13 output."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, Douttgl31_0Spec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit is used to toggle DIO14 output."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, Douttgl31_0Spec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - This bit is used to toggle DIO15 output."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, Douttgl31_0Spec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - This bit is used to toggle DIO16 output."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, Douttgl31_0Spec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used to toggle DIO17 output."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, Douttgl31_0Spec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit is used to toggle DIO18 output."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, Douttgl31_0Spec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit is used to toggle DIO19 output."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, Douttgl31_0Spec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit is used to toggle DIO20 output."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, Douttgl31_0Spec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - This bit is used to toggle DIO21 output."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, Douttgl31_0Spec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit is used to toggle DIO22 output."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, Douttgl31_0Spec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit is used to toggle DIO23 output."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, Douttgl31_0Spec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to toggle DIO24 output."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, Douttgl31_0Spec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is used to toggle DIO25 output."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, Douttgl31_0Spec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - This bit is used to toggle DIO26 output."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, Douttgl31_0Spec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit is used to toggle DIO27 output."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, Douttgl31_0Spec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit is used to toggle DIO28 output."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, Douttgl31_0Spec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to toggle DIO29 output."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, Douttgl31_0Spec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit is used to toggle DIO30 output."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, Douttgl31_0Spec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is used to toggle DIO31 output."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, Douttgl31_0Spec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`douttgl31_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Douttgl31_0Spec;
impl crate::RegisterSpec for Douttgl31_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`douttgl31_0::W`](W) writer structure"]
impl crate::Writable for Douttgl31_0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUTTGL31_0 to value 0"]
impl crate::Resettable for Douttgl31_0Spec {}
