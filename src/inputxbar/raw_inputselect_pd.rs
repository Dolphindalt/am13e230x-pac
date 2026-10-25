#[doc = "Register `RAW_INPUTSELECT_PD` reader"]
pub type R = crate::R<RawInputselectPdSpec>;
#[doc = "Register `RAW_INPUTSELECT_PD` writer"]
pub type W = crate::W<RawInputselectPdSpec>;
#[doc = "Field `SEL0` reader - Raw input select bit for PD0"]
pub type Sel0R = crate::BitReader;
#[doc = "Field `SEL0` writer - Raw input select bit for PD0"]
pub type Sel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL1` reader - Raw input select bit for PD1"]
pub type Sel1R = crate::BitReader;
#[doc = "Field `SEL1` writer - Raw input select bit for PD1"]
pub type Sel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL2` reader - Raw input select bit for PD2"]
pub type Sel2R = crate::BitReader;
#[doc = "Field `SEL2` writer - Raw input select bit for PD2"]
pub type Sel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL3` reader - Raw input select bit for PD3"]
pub type Sel3R = crate::BitReader;
#[doc = "Field `SEL3` writer - Raw input select bit for PD3"]
pub type Sel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL4` reader - Raw input select bit for PD4"]
pub type Sel4R = crate::BitReader;
#[doc = "Field `SEL4` writer - Raw input select bit for PD4"]
pub type Sel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL5` reader - Raw input select bit for PD5"]
pub type Sel5R = crate::BitReader;
#[doc = "Field `SEL5` writer - Raw input select bit for PD5"]
pub type Sel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL6` reader - Raw input select bit for PD6"]
pub type Sel6R = crate::BitReader;
#[doc = "Field `SEL6` writer - Raw input select bit for PD6"]
pub type Sel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL7` reader - Raw input select bit for PD7"]
pub type Sel7R = crate::BitReader;
#[doc = "Field `SEL7` writer - Raw input select bit for PD7"]
pub type Sel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL8` reader - Raw input select bit for PD8"]
pub type Sel8R = crate::BitReader;
#[doc = "Field `SEL8` writer - Raw input select bit for PD8"]
pub type Sel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL9` reader - Raw input select bit for PD9"]
pub type Sel9R = crate::BitReader;
#[doc = "Field `SEL9` writer - Raw input select bit for PD9"]
pub type Sel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL11` reader - Raw input select bit for PD11"]
pub type Sel11R = crate::BitReader;
#[doc = "Field `SEL11` writer - Raw input select bit for PD11"]
pub type Sel11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw input select bit for PD0"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw input select bit for PD1"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw input select bit for PD2"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw input select bit for PD3"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw input select bit for PD4"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw input select bit for PD5"]
    #[inline(always)]
    pub fn sel5(&self) -> Sel5R {
        Sel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw input select bit for PD6"]
    #[inline(always)]
    pub fn sel6(&self) -> Sel6R {
        Sel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw input select bit for PD7"]
    #[inline(always)]
    pub fn sel7(&self) -> Sel7R {
        Sel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw input select bit for PD8"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw input select bit for PD9"]
    #[inline(always)]
    pub fn sel9(&self) -> Sel9R {
        Sel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw input select bit for PD11"]
    #[inline(always)]
    pub fn sel11(&self) -> Sel11R {
        Sel11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW_INPUTSELECT_PD")
            .field("sel0", &self.sel0())
            .field("sel1", &self.sel1())
            .field("sel2", &self.sel2())
            .field("sel3", &self.sel3())
            .field("sel4", &self.sel4())
            .field("sel5", &self.sel5())
            .field("sel6", &self.sel6())
            .field("sel7", &self.sel7())
            .field("sel8", &self.sel8())
            .field("sel9", &self.sel9())
            .field("sel11", &self.sel11())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Raw input select bit for PD0"]
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<'_, RawInputselectPdSpec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw input select bit for PD1"]
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<'_, RawInputselectPdSpec> {
        Sel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw input select bit for PD2"]
    #[inline(always)]
    pub fn sel2(&mut self) -> Sel2W<'_, RawInputselectPdSpec> {
        Sel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw input select bit for PD3"]
    #[inline(always)]
    pub fn sel3(&mut self) -> Sel3W<'_, RawInputselectPdSpec> {
        Sel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Raw input select bit for PD4"]
    #[inline(always)]
    pub fn sel4(&mut self) -> Sel4W<'_, RawInputselectPdSpec> {
        Sel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Raw input select bit for PD5"]
    #[inline(always)]
    pub fn sel5(&mut self) -> Sel5W<'_, RawInputselectPdSpec> {
        Sel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Raw input select bit for PD6"]
    #[inline(always)]
    pub fn sel6(&mut self) -> Sel6W<'_, RawInputselectPdSpec> {
        Sel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Raw input select bit for PD7"]
    #[inline(always)]
    pub fn sel7(&mut self) -> Sel7W<'_, RawInputselectPdSpec> {
        Sel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Raw input select bit for PD8"]
    #[inline(always)]
    pub fn sel8(&mut self) -> Sel8W<'_, RawInputselectPdSpec> {
        Sel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw input select bit for PD9"]
    #[inline(always)]
    pub fn sel9(&mut self) -> Sel9W<'_, RawInputselectPdSpec> {
        Sel9W::new(self, 9)
    }
    #[doc = "Bit 11 - Raw input select bit for PD11"]
    #[inline(always)]
    pub fn sel11(&mut self) -> Sel11W<'_, RawInputselectPdSpec> {
        Sel11W::new(self, 11)
    }
}
#[doc = "Raw Input Select Register for Port-D GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_inputselect_pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_inputselect_pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawInputselectPdSpec;
impl crate::RegisterSpec for RawInputselectPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_inputselect_pd::R`](R) reader structure"]
impl crate::Readable for RawInputselectPdSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_inputselect_pd::W`](W) writer structure"]
impl crate::Writable for RawInputselectPdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RAW_INPUTSELECT_PD to value 0"]
impl crate::Resettable for RawInputselectPdSpec {}
