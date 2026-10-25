#[doc = "Register `ADCINTOVF` reader"]
pub type R = crate::R<AdcintovfSpec>;
#[doc = "Register `ADCINTOVF` writer"]
pub type W = crate::W<AdcintovfSpec>;
#[doc = "Field `ADCINT1OVF` reader - ADC Interrupt 1 Overflow Flags"]
pub type Adcint1ovfR = crate::BitReader;
#[doc = "Field `ADCINT2OVF` reader - ADC Interrupt 2 Overflow Flags"]
pub type Adcint2ovfR = crate::BitReader;
#[doc = "Field `ADCINT3OVF` reader - ADC Interrupt 3 Overflow Flags"]
pub type Adcint3ovfR = crate::BitReader;
#[doc = "Field `ADCINT4OVF` reader - ADC Interrupt 4 Overflow Flags"]
pub type Adcint4ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT1OVF` reader - ADC DMA Interrupt 1 Overflow Flags"]
pub type Adcdmaint1ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT2OVF` reader - ADC DMA Interrupt 2 Overflow Flags"]
pub type Adcdmaint2ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT3OVF` reader - ADC DMA Interrupt 3 Overflow Flags"]
pub type Adcdmaint3ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT4OVF` reader - ADC DMA Interrupt 4 Overflow Flags"]
pub type Adcdmaint4ovfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Interrupt 1 Overflow Flags"]
    #[inline(always)]
    pub fn adcint1ovf(&self) -> Adcint1ovfR {
        Adcint1ovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Overflow Flags"]
    #[inline(always)]
    pub fn adcint2ovf(&self) -> Adcint2ovfR {
        Adcint2ovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Overflow Flags"]
    #[inline(always)]
    pub fn adcint3ovf(&self) -> Adcint3ovfR {
        Adcint3ovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Overflow Flags"]
    #[inline(always)]
    pub fn adcint4ovf(&self) -> Adcint4ovfR {
        Adcint4ovfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Overflow Flags"]
    #[inline(always)]
    pub fn adcdmaint1ovf(&self) -> Adcdmaint1ovfR {
        Adcdmaint1ovfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Overflow Flags"]
    #[inline(always)]
    pub fn adcdmaint2ovf(&self) -> Adcdmaint2ovfR {
        Adcdmaint2ovfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Overflow Flags"]
    #[inline(always)]
    pub fn adcdmaint3ovf(&self) -> Adcdmaint3ovfR {
        Adcdmaint3ovfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Overflow Flags"]
    #[inline(always)]
    pub fn adcdmaint4ovf(&self) -> Adcdmaint4ovfR {
        Adcdmaint4ovfR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTOVF")
            .field("adcint1ovf", &self.adcint1ovf())
            .field("adcint2ovf", &self.adcint2ovf())
            .field("adcint3ovf", &self.adcint3ovf())
            .field("adcint4ovf", &self.adcint4ovf())
            .field("adcdmaint1ovf", &self.adcdmaint1ovf())
            .field("adcdmaint2ovf", &self.adcdmaint2ovf())
            .field("adcdmaint3ovf", &self.adcdmaint3ovf())
            .field("adcdmaint4ovf", &self.adcdmaint4ovf())
            .finish()
    }
}
impl W {}
#[doc = "ADC Interrupt Overflow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintovf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintovf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintovfSpec;
impl crate::RegisterSpec for AdcintovfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintovf::R`](R) reader structure"]
impl crate::Readable for AdcintovfSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintovf::W`](W) writer structure"]
impl crate::Writable for AdcintovfSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTOVF to value 0"]
impl crate::Resettable for AdcintovfSpec {}
