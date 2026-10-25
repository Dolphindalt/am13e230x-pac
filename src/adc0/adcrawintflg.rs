#[doc = "Register `ADCRAWINTFLG` reader"]
pub type R = crate::R<AdcrawintflgSpec>;
#[doc = "Register `ADCRAWINTFLG` writer"]
pub type W = crate::W<AdcrawintflgSpec>;
#[doc = "Field `ADCRAWINT1` reader - ADC Raw Interrupt 1 Flag"]
pub type Adcrawint1R = crate::BitReader;
#[doc = "Field `ADCRAWINT2` reader - ADC Raw Interrupt 2 Flag"]
pub type Adcrawint2R = crate::BitReader;
#[doc = "Field `ADCRAWINT3` reader - ADC Raw Interrupt 3 Flag"]
pub type Adcrawint3R = crate::BitReader;
#[doc = "Field `ADCRAWINT4` reader - ADC Raw Interrupt 4 Flag"]
pub type Adcrawint4R = crate::BitReader;
#[doc = "Field `ADCDMARAWINT1` reader - ADC DMA Raw Interrupt 1 Flag"]
pub type Adcdmarawint1R = crate::BitReader;
#[doc = "Field `ADCDMARAWINT2` reader - ADC DMA Raw Interrupt 2 Flag"]
pub type Adcdmarawint2R = crate::BitReader;
#[doc = "Field `ADCDMARAWINT3` reader - ADC DMA Raw Interrupt 3 Flag"]
pub type Adcdmarawint3R = crate::BitReader;
#[doc = "Field `ADCDMARAWINT4` reader - ADC DMA Raw Interrupt 4 Flag"]
pub type Adcdmarawint4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Raw Interrupt 1 Flag"]
    #[inline(always)]
    pub fn adcrawint1(&self) -> Adcrawint1R {
        Adcrawint1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Raw Interrupt 2 Flag"]
    #[inline(always)]
    pub fn adcrawint2(&self) -> Adcrawint2R {
        Adcrawint2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Raw Interrupt 3 Flag"]
    #[inline(always)]
    pub fn adcrawint3(&self) -> Adcrawint3R {
        Adcrawint3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Raw Interrupt 4 Flag"]
    #[inline(always)]
    pub fn adcrawint4(&self) -> Adcrawint4R {
        Adcrawint4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC DMA Raw Interrupt 1 Flag"]
    #[inline(always)]
    pub fn adcdmarawint1(&self) -> Adcdmarawint1R {
        Adcdmarawint1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC DMA Raw Interrupt 2 Flag"]
    #[inline(always)]
    pub fn adcdmarawint2(&self) -> Adcdmarawint2R {
        Adcdmarawint2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC DMA Raw Interrupt 3 Flag"]
    #[inline(always)]
    pub fn adcdmarawint3(&self) -> Adcdmarawint3R {
        Adcdmarawint3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC DMA Raw Interrupt 4 Flag"]
    #[inline(always)]
    pub fn adcdmarawint4(&self) -> Adcdmarawint4R {
        Adcdmarawint4R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCRAWINTFLG")
            .field("adcrawint1", &self.adcrawint1())
            .field("adcrawint2", &self.adcrawint2())
            .field("adcrawint3", &self.adcrawint3())
            .field("adcrawint4", &self.adcrawint4())
            .field("adcdmarawint1", &self.adcdmarawint1())
            .field("adcdmarawint2", &self.adcdmarawint2())
            .field("adcdmarawint3", &self.adcdmarawint3())
            .field("adcdmarawint4", &self.adcdmarawint4())
            .finish()
    }
}
impl W {}
#[doc = "ADC Raw Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrawintflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrawintflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcrawintflgSpec;
impl crate::RegisterSpec for AdcrawintflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcrawintflg::R`](R) reader structure"]
impl crate::Readable for AdcrawintflgSpec {}
#[doc = "`write(|w| ..)` method takes [`adcrawintflg::W`](W) writer structure"]
impl crate::Writable for AdcrawintflgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRAWINTFLG to value 0"]
impl crate::Resettable for AdcrawintflgSpec {}
