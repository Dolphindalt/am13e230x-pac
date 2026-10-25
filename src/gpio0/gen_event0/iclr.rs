#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `DIO0` writer - DIO0 event"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO1` writer - DIO1 event"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO2` writer - DIO2 event"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO3` writer - DIO3 event"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO4` writer - DIO4 event"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO5` writer - DIO5 event"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO6` writer - DIO6 event"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO7` writer - DIO7 event"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO8` writer - DIO8 event"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO9` writer - DIO9 event"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO10` writer - DIO10 event"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO11` writer - DIO11 event"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO12` writer - DIO12 event"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO13` writer - DIO13 event"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO14` writer - DIO14 event"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO15` writer - DIO15 event"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, IclrSpec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, IclrSpec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, IclrSpec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, IclrSpec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, IclrSpec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, IclrSpec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, IclrSpec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, IclrSpec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, IclrSpec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, IclrSpec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, IclrSpec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, IclrSpec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, IclrSpec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, IclrSpec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, IclrSpec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, IclrSpec> {
        Dio15W::new(self, 15)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
