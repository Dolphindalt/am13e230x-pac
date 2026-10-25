#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `DIO16` writer - DIO16 event"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO17` writer - DIO17 event"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO18` writer - DIO18 event"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO19` writer - DIO19 event"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO20` writer - DIO20 event"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO21` writer - DIO21 event"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO22` writer - DIO22 event"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO23` writer - DIO23 event"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO24` writer - DIO24 event"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO25` writer - DIO25 event"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO26` writer - DIO26 event"]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO27` writer - DIO27 event"]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO28` writer - DIO28 event"]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO29` writer - DIO29 event"]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO30` writer - DIO30 event"]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO31` writer - DIO31 event"]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, IsetSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, IsetSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, IsetSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, IsetSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, IsetSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, IsetSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, IsetSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, IsetSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, IsetSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, IsetSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, IsetSpec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, IsetSpec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, IsetSpec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, IsetSpec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, IsetSpec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, IsetSpec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {}
