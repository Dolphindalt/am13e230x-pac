#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `DIO0` reader - DIO0 event mask"]
pub type Dio0R = crate::BitReader;
#[doc = "Field `DIO0` writer - DIO0 event mask"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO1` reader - DIO1 event mask"]
pub type Dio1R = crate::BitReader;
#[doc = "Field `DIO1` writer - DIO1 event mask"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO2` reader - DIO2 event mask"]
pub type Dio2R = crate::BitReader;
#[doc = "Field `DIO2` writer - DIO2 event mask"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO3` reader - DIO3 event mask"]
pub type Dio3R = crate::BitReader;
#[doc = "Field `DIO3` writer - DIO3 event mask"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO4` reader - DIO4 event mask"]
pub type Dio4R = crate::BitReader;
#[doc = "Field `DIO4` writer - DIO4 event mask"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO5` reader - DIO5 event mask"]
pub type Dio5R = crate::BitReader;
#[doc = "Field `DIO5` writer - DIO5 event mask"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO6` reader - DIO6 event mask"]
pub type Dio6R = crate::BitReader;
#[doc = "Field `DIO6` writer - DIO6 event mask"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO7` reader - DIO7 event mask"]
pub type Dio7R = crate::BitReader;
#[doc = "Field `DIO7` writer - DIO7 event mask"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO8` reader - DIO8 event mask"]
pub type Dio8R = crate::BitReader;
#[doc = "Field `DIO8` writer - DIO8 event mask"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO9` reader - DIO9 event mask"]
pub type Dio9R = crate::BitReader;
#[doc = "Field `DIO9` writer - DIO9 event mask"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO10` reader - DIO10 event mask"]
pub type Dio10R = crate::BitReader;
#[doc = "Field `DIO10` writer - DIO10 event mask"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO11` reader - DIO11 event mask"]
pub type Dio11R = crate::BitReader;
#[doc = "Field `DIO11` writer - DIO11 event mask"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO12` reader - DIO12 event mask"]
pub type Dio12R = crate::BitReader;
#[doc = "Field `DIO12` writer - DIO12 event mask"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO13` reader - DIO13 event mask"]
pub type Dio13R = crate::BitReader;
#[doc = "Field `DIO13` writer - DIO13 event mask"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO14` reader - DIO14 event mask"]
pub type Dio14R = crate::BitReader;
#[doc = "Field `DIO14` writer - DIO14 event mask"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO15` reader - DIO15 event mask"]
pub type Dio15R = crate::BitReader;
#[doc = "Field `DIO15` writer - DIO15 event mask"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
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
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, ImaskSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, ImaskSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, ImaskSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, ImaskSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, ImaskSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, ImaskSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, ImaskSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, ImaskSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, ImaskSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, ImaskSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, ImaskSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, ImaskSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, ImaskSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, ImaskSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, ImaskSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, ImaskSpec> {
        Dio15W::new(self, 15)
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
