#[doc = "Register `ADCINTFLGCLR` reader"]
pub type R = crate::R<AdcintflgclrSpec>;
#[doc = "Register `ADCINTFLGCLR` writer"]
pub type W = crate::W<AdcintflgclrSpec>;
#[doc = "Field `ADCINT1` reader - ADC Interrupt 1 Flag Clear"]
pub type Adcint1R = crate::BitReader;
#[doc = "Field `ADCINT1` writer - ADC Interrupt 1 Flag Clear"]
pub type Adcint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT2` reader - ADC Interrupt 2 Flag Clear"]
pub type Adcint2R = crate::BitReader;
#[doc = "Field `ADCINT2` writer - ADC Interrupt 2 Flag Clear"]
pub type Adcint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT3` reader - ADC Interrupt 3 Flag Clear"]
pub type Adcint3R = crate::BitReader;
#[doc = "Field `ADCINT3` writer - ADC Interrupt 3 Flag Clear"]
pub type Adcint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT4` reader - ADC Interrupt 4 Flag Clear"]
pub type Adcint4R = crate::BitReader;
#[doc = "Field `ADCINT4` writer - ADC Interrupt 4 Flag Clear"]
pub type Adcint4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT1` reader - ADC DMA Interrupt 1 Flag Clear"]
pub type Adcdmaint1R = crate::BitReader;
#[doc = "Field `ADCDMAINT1` writer - ADC DMA Interrupt 1 Flag Clear"]
pub type Adcdmaint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT2` reader - ADC DMA Interrupt 2 Flag Clear"]
pub type Adcdmaint2R = crate::BitReader;
#[doc = "Field `ADCDMAINT2` writer - ADC DMA Interrupt 2 Flag Clear"]
pub type Adcdmaint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT3` reader - ADC DMA Interrupt 3 Flag Clear"]
pub type Adcdmaint3R = crate::BitReader;
#[doc = "Field `ADCDMAINT3` writer - ADC DMA Interrupt 3 Flag Clear"]
pub type Adcdmaint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT4` reader - ADC DMA Interrupt 4 Flag Clear"]
pub type Adcdmaint4R = crate::BitReader;
#[doc = "Field `ADCDMAINT4` writer - ADC DMA Interrupt 4 Flag Clear"]
pub type Adcdmaint4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt 1 Flag Clear"]
    #[inline(always)]
    pub fn adcint1(&self) -> Adcint1R {
        Adcint1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Flag Clear"]
    #[inline(always)]
    pub fn adcint2(&self) -> Adcint2R {
        Adcint2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Flag Clear"]
    #[inline(always)]
    pub fn adcint3(&self) -> Adcint3R {
        Adcint3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Flag Clear"]
    #[inline(always)]
    pub fn adcint4(&self) -> Adcint4R {
        Adcint4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint1(&self) -> Adcdmaint1R {
        Adcdmaint1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint2(&self) -> Adcdmaint2R {
        Adcdmaint2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint3(&self) -> Adcdmaint3R {
        Adcdmaint3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint4(&self) -> Adcdmaint4R {
        Adcdmaint4R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTFLGCLR")
            .field("adcint1", &self.adcint1())
            .field("adcint2", &self.adcint2())
            .field("adcint3", &self.adcint3())
            .field("adcint4", &self.adcint4())
            .field("adcdmaint1", &self.adcdmaint1())
            .field("adcdmaint2", &self.adcdmaint2())
            .field("adcdmaint3", &self.adcdmaint3())
            .field("adcdmaint4", &self.adcdmaint4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt 1 Flag Clear"]
    #[inline(always)]
    pub fn adcint1(&mut self) -> Adcint1W<'_, AdcintflgclrSpec> {
        Adcint1W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Flag Clear"]
    #[inline(always)]
    pub fn adcint2(&mut self) -> Adcint2W<'_, AdcintflgclrSpec> {
        Adcint2W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Flag Clear"]
    #[inline(always)]
    pub fn adcint3(&mut self) -> Adcint3W<'_, AdcintflgclrSpec> {
        Adcint3W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Flag Clear"]
    #[inline(always)]
    pub fn adcint4(&mut self) -> Adcint4W<'_, AdcintflgclrSpec> {
        Adcint4W::new(self, 3)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint1(&mut self) -> Adcdmaint1W<'_, AdcintflgclrSpec> {
        Adcdmaint1W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint2(&mut self) -> Adcdmaint2W<'_, AdcintflgclrSpec> {
        Adcdmaint2W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint3(&mut self) -> Adcdmaint3W<'_, AdcintflgclrSpec> {
        Adcdmaint3W::new(self, 18)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Flag Clear"]
    #[inline(always)]
    pub fn adcdmaint4(&mut self) -> Adcdmaint4W<'_, AdcintflgclrSpec> {
        Adcdmaint4W::new(self, 19)
    }
}
#[doc = "ADC Interrupt Flag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintflgclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintflgclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintflgclrSpec;
impl crate::RegisterSpec for AdcintflgclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintflgclr::R`](R) reader structure"]
impl crate::Readable for AdcintflgclrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintflgclr::W`](W) writer structure"]
impl crate::Writable for AdcintflgclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTFLGCLR to value 0"]
impl crate::Resettable for AdcintflgclrSpec {}
