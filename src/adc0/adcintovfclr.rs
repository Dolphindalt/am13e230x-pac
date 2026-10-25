#[doc = "Register `ADCINTOVFCLR` reader"]
pub type R = crate::R<AdcintovfclrSpec>;
#[doc = "Register `ADCINTOVFCLR` writer"]
pub type W = crate::W<AdcintovfclrSpec>;
#[doc = "Field `ADCINT1OVF` reader - ADC Interrupt 1 Overflow Clear Bits"]
pub type Adcint1ovfR = crate::BitReader;
#[doc = "Field `ADCINT1OVF` writer - ADC Interrupt 1 Overflow Clear Bits"]
pub type Adcint1ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT2OVF` reader - ADC Interrupt 2 Overflow Clear Bits"]
pub type Adcint2ovfR = crate::BitReader;
#[doc = "Field `ADCINT2OVF` writer - ADC Interrupt 2 Overflow Clear Bits"]
pub type Adcint2ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT3OVF` reader - ADC Interrupt 3 Overflow Clear Bits"]
pub type Adcint3ovfR = crate::BitReader;
#[doc = "Field `ADCINT3OVF` writer - ADC Interrupt 3 Overflow Clear Bits"]
pub type Adcint3ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINT4OVF` reader - ADC Interrupt 4 Overflow Clear Bits"]
pub type Adcint4ovfR = crate::BitReader;
#[doc = "Field `ADCINT4OVF` writer - ADC Interrupt 4 Overflow Clear Bits"]
pub type Adcint4ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT1OVF` reader - ADC DMA Interrupt 1 Overflow Clear Bits"]
pub type Adcdmaint1ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT1OVF` writer - ADC DMA Interrupt 1 Overflow Clear Bits"]
pub type Adcdmaint1ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT2OVF` reader - ADC DMA Interrupt 2 Overflow Clear Bits"]
pub type Adcdmaint2ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT2OVF` writer - ADC DMA Interrupt 2 Overflow Clear Bits"]
pub type Adcdmaint2ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT3OVF` reader - ADC DMA Interrupt 3 Overflow Clear Bits"]
pub type Adcdmaint3ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT3OVF` writer - ADC DMA Interrupt 3 Overflow Clear Bits"]
pub type Adcdmaint3ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDMAINT4OVF` reader - ADC DMA Interrupt 4 Overflow Clear Bits"]
pub type Adcdmaint4ovfR = crate::BitReader;
#[doc = "Field `ADCDMAINT4OVF` writer - ADC DMA Interrupt 4 Overflow Clear Bits"]
pub type Adcdmaint4ovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt 1 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint1ovf(&self) -> Adcint1ovfR {
        Adcint1ovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint2ovf(&self) -> Adcint2ovfR {
        Adcint2ovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint3ovf(&self) -> Adcint3ovfR {
        Adcint3ovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint4ovf(&self) -> Adcint4ovfR {
        Adcint4ovfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint1ovf(&self) -> Adcdmaint1ovfR {
        Adcdmaint1ovfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint2ovf(&self) -> Adcdmaint2ovfR {
        Adcdmaint2ovfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint3ovf(&self) -> Adcdmaint3ovfR {
        Adcdmaint3ovfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint4ovf(&self) -> Adcdmaint4ovfR {
        Adcdmaint4ovfR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTOVFCLR")
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
impl W {
    #[doc = "Bit 0 - ADC Interrupt 1 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint1ovf(&mut self) -> Adcint1ovfW<'_, AdcintovfclrSpec> {
        Adcint1ovfW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt 2 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint2ovf(&mut self) -> Adcint2ovfW<'_, AdcintovfclrSpec> {
        Adcint2ovfW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt 3 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint3ovf(&mut self) -> Adcint3ovfW<'_, AdcintovfclrSpec> {
        Adcint3ovfW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt 4 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcint4ovf(&mut self) -> Adcint4ovfW<'_, AdcintovfclrSpec> {
        Adcint4ovfW::new(self, 3)
    }
    #[doc = "Bit 16 - ADC DMA Interrupt 1 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint1ovf(&mut self) -> Adcdmaint1ovfW<'_, AdcintovfclrSpec> {
        Adcdmaint1ovfW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC DMA Interrupt 2 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint2ovf(&mut self) -> Adcdmaint2ovfW<'_, AdcintovfclrSpec> {
        Adcdmaint2ovfW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC DMA Interrupt 3 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint3ovf(&mut self) -> Adcdmaint3ovfW<'_, AdcintovfclrSpec> {
        Adcdmaint3ovfW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC DMA Interrupt 4 Overflow Clear Bits"]
    #[inline(always)]
    pub fn adcdmaint4ovf(&mut self) -> Adcdmaint4ovfW<'_, AdcintovfclrSpec> {
        Adcdmaint4ovfW::new(self, 19)
    }
}
#[doc = "ADC Interrupt Overflow Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintovfclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintovfclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintovfclrSpec;
impl crate::RegisterSpec for AdcintovfclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintovfclr::R`](R) reader structure"]
impl crate::Readable for AdcintovfclrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintovfclr::W`](W) writer structure"]
impl crate::Writable for AdcintovfclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTOVFCLR to value 0"]
impl crate::Resettable for AdcintovfclrSpec {}
