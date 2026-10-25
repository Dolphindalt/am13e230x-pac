#[doc = "Register `ADCINTFLG` reader"]
pub type R = crate::R<AdcintflgSpec>;
#[doc = "Register `ADCINTFLG` writer"]
pub type W = crate::W<AdcintflgSpec>;
#[doc = "Field `ADCINT1` reader - ADC Interrupt 1 Flag"]
pub type Adcint1R = crate::BitReader;
#[doc = "Field `ADCINT2` reader - ADC Interrupt 2 Flag"]
pub type Adcint2R = crate::BitReader;
#[doc = "Field `ADCINT3` reader - ADC Interrupt 3 Flag"]
pub type Adcint3R = crate::BitReader;
#[doc = "Field `ADCINT4` reader - ADC Interrupt 4 Flag"]
pub type Adcint4R = crate::BitReader;
#[doc = "Field `ADCINT1RESULT` reader - ADC Interrupt 1 Results Ready"]
pub type Adcint1resultR = crate::BitReader;
#[doc = "Field `ADCINT2RESULT` reader - ADC Interrupt 2 Results Ready"]
pub type Adcint2resultR = crate::BitReader;
#[doc = "Field `ADCINT3RESULT` reader - ADC Interrupt 3 Results Ready"]
pub type Adcint3resultR = crate::BitReader;
#[doc = "Field `ADCINT4RESULT` reader - ADC Interrupt 4 Results Ready"]
pub type Adcint4resultR = crate::BitReader;
#[doc = "Field `ADCDMAINT1` reader - ADC DMA Interrupt 1 Flag"]
pub type Adcdmaint1R = crate::BitReader;
#[doc = "Field `ADCDMAINT2` reader - ADC DMA Interrupt 2 Flag"]
pub type Adcdmaint2R = crate::BitReader;
#[doc = "Field `ADCDMAINT3` reader - ADC DMA Interrupt 3 Flag"]
pub type Adcdmaint3R = crate::BitReader;
#[doc = "Field `ADCDMAINT4` reader - ADC DMA Interrupt 4 Flag"]
pub type Adcdmaint4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Interrupt 1 Flag"]
    #[inline(always)]
    pub fn adcint1(&self) -> Adcint1R {
        Adcint1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Flag"]
    #[inline(always)]
    pub fn adcint2(&self) -> Adcint2R {
        Adcint2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Flag"]
    #[inline(always)]
    pub fn adcint3(&self) -> Adcint3R {
        Adcint3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Flag"]
    #[inline(always)]
    pub fn adcint4(&self) -> Adcint4R {
        Adcint4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Interrupt 1 Results Ready"]
    #[inline(always)]
    pub fn adcint1result(&self) -> Adcint1resultR {
        Adcint1resultR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC Interrupt 2 Results Ready"]
    #[inline(always)]
    pub fn adcint2result(&self) -> Adcint2resultR {
        Adcint2resultR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC Interrupt 3 Results Ready"]
    #[inline(always)]
    pub fn adcint3result(&self) -> Adcint3resultR {
        Adcint3resultR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Interrupt 4 Results Ready"]
    #[inline(always)]
    pub fn adcint4result(&self) -> Adcint4resultR {
        Adcint4resultR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Flag"]
    #[inline(always)]
    pub fn adcdmaint1(&self) -> Adcdmaint1R {
        Adcdmaint1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Flag"]
    #[inline(always)]
    pub fn adcdmaint2(&self) -> Adcdmaint2R {
        Adcdmaint2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Flag"]
    #[inline(always)]
    pub fn adcdmaint3(&self) -> Adcdmaint3R {
        Adcdmaint3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Flag"]
    #[inline(always)]
    pub fn adcdmaint4(&self) -> Adcdmaint4R {
        Adcdmaint4R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTFLG")
            .field("adcint1", &self.adcint1())
            .field("adcint2", &self.adcint2())
            .field("adcint3", &self.adcint3())
            .field("adcint4", &self.adcint4())
            .field("adcint1result", &self.adcint1result())
            .field("adcint2result", &self.adcint2result())
            .field("adcint3result", &self.adcint3result())
            .field("adcint4result", &self.adcint4result())
            .field("adcdmaint1", &self.adcdmaint1())
            .field("adcdmaint2", &self.adcdmaint2())
            .field("adcdmaint3", &self.adcdmaint3())
            .field("adcdmaint4", &self.adcdmaint4())
            .finish()
    }
}
impl W {}
#[doc = "ADC Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintflgSpec;
impl crate::RegisterSpec for AdcintflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintflg::R`](R) reader structure"]
impl crate::Readable for AdcintflgSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintflg::W`](W) writer structure"]
impl crate::Writable for AdcintflgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTFLG to value 0"]
impl crate::Resettable for AdcintflgSpec {}
