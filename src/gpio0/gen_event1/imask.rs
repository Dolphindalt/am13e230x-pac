#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `DIO16` reader - DIO16 event mask"]
pub type Dio16R = crate::BitReader;
#[doc = "Field `DIO16` writer - DIO16 event mask"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO17` reader - DIO17 event mask"]
pub type Dio17R = crate::BitReader;
#[doc = "Field `DIO17` writer - DIO17 event mask"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO18` reader - DIO18 event mask"]
pub type Dio18R = crate::BitReader;
#[doc = "Field `DIO18` writer - DIO18 event mask"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO19` reader - DIO19 event mask"]
pub type Dio19R = crate::BitReader;
#[doc = "Field `DIO19` writer - DIO19 event mask"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO20` reader - DIO20 event mask"]
pub type Dio20R = crate::BitReader;
#[doc = "Field `DIO20` writer - DIO20 event mask"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO21` reader - DIO21 event mask"]
pub type Dio21R = crate::BitReader;
#[doc = "Field `DIO21` writer - DIO21 event mask"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO22` reader - DIO22 event mask"]
pub type Dio22R = crate::BitReader;
#[doc = "Field `DIO22` writer - DIO22 event mask"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO23` reader - DIO23 event mask"]
pub type Dio23R = crate::BitReader;
#[doc = "Field `DIO23` writer - DIO23 event mask"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO24` reader - DIO24 event mask"]
pub type Dio24R = crate::BitReader;
#[doc = "Field `DIO24` writer - DIO24 event mask"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO25` reader - DIO25 event mask"]
pub type Dio25R = crate::BitReader;
#[doc = "Field `DIO25` writer - DIO25 event mask"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO26` reader - DIO26 event mask"]
pub type Dio26R = crate::BitReader;
#[doc = "Field `DIO26` writer - DIO26 event mask"]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO27` reader - DIO27 event mask"]
pub type Dio27R = crate::BitReader;
#[doc = "Field `DIO27` writer - DIO27 event mask"]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO28` reader - DIO28 event mask"]
pub type Dio28R = crate::BitReader;
#[doc = "Field `DIO28` writer - DIO28 event mask"]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO29` reader - DIO29 event mask"]
pub type Dio29R = crate::BitReader;
#[doc = "Field `DIO29` writer - DIO29 event mask"]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO30` reader - DIO30 event mask"]
pub type Dio30R = crate::BitReader;
#[doc = "Field `DIO30` writer - DIO30 event mask"]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO31` reader - DIO31 event mask"]
pub type Dio31R = crate::BitReader;
#[doc = "Field `DIO31` writer - DIO31 event mask"]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
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
impl W {
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, ImaskSpec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, ImaskSpec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, ImaskSpec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, ImaskSpec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, ImaskSpec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, ImaskSpec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, ImaskSpec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, ImaskSpec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, ImaskSpec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, ImaskSpec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, ImaskSpec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, ImaskSpec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, ImaskSpec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, ImaskSpec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, ImaskSpec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, ImaskSpec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
