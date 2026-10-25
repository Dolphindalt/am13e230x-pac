#[doc = "Register `ADCSOCOVFCLR1` reader"]
pub type R = crate::R<Adcsocovfclr1Spec>;
#[doc = "Register `ADCSOCOVFCLR1` writer"]
pub type W = crate::W<Adcsocovfclr1Spec>;
#[doc = "Field `SOC0OVF` reader - SOC0 Clear Start of Conversion Overflow Bit"]
pub type Soc0ovfR = crate::BitReader;
#[doc = "Field `SOC0OVF` writer - SOC0 Clear Start of Conversion Overflow Bit"]
pub type Soc0ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC1OVF` reader - SOC1 Clear Start of Conversion Overflow Bit"]
pub type Soc1ovfR = crate::BitReader;
#[doc = "Field `SOC1OVF` writer - SOC1 Clear Start of Conversion Overflow Bit"]
pub type Soc1ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC2OVF` reader - SOC2 Clear Start of Conversion Overflow Bit"]
pub type Soc2ovfR = crate::BitReader;
#[doc = "Field `SOC2OVF` writer - SOC2 Clear Start of Conversion Overflow Bit"]
pub type Soc2ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC3OVF` reader - SOC3 Clear Start of Conversion Overflow Bit"]
pub type Soc3ovfR = crate::BitReader;
#[doc = "Field `SOC3OVF` writer - SOC3 Clear Start of Conversion Overflow Bit"]
pub type Soc3ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC4OVF` reader - SOC4 Clear Start of Conversion Overflow Bit"]
pub type Soc4ovfR = crate::BitReader;
#[doc = "Field `SOC4OVF` writer - SOC4 Clear Start of Conversion Overflow Bit"]
pub type Soc4ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC5OVF` reader - SOC5 Clear Start of Conversion Overflow Bit"]
pub type Soc5ovfR = crate::BitReader;
#[doc = "Field `SOC5OVF` writer - SOC5 Clear Start of Conversion Overflow Bit"]
pub type Soc5ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC6OVF` reader - SOC6 Clear Start of Conversion Overflow Bit"]
pub type Soc6ovfR = crate::BitReader;
#[doc = "Field `SOC6OVF` writer - SOC6 Clear Start of Conversion Overflow Bit"]
pub type Soc6ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC7OVF` reader - SOC7 Clear Start of Conversion Overflow Bit"]
pub type Soc7ovfR = crate::BitReader;
#[doc = "Field `SOC7OVF` writer - SOC7 Clear Start of Conversion Overflow Bit"]
pub type Soc7ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC8OVF` reader - SOC8 Clear Start of Conversion Overflow Bit"]
pub type Soc8ovfR = crate::BitReader;
#[doc = "Field `SOC8OVF` writer - SOC8 Clear Start of Conversion Overflow Bit"]
pub type Soc8ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC9OVF` reader - SOC9 Clear Start of Conversion Overflow Bit"]
pub type Soc9ovfR = crate::BitReader;
#[doc = "Field `SOC9OVF` writer - SOC9 Clear Start of Conversion Overflow Bit"]
pub type Soc9ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC10OVF` reader - SOC10 Clear Start of Conversion Overflow Bit"]
pub type Soc10ovfR = crate::BitReader;
#[doc = "Field `SOC10OVF` writer - SOC10 Clear Start of Conversion Overflow Bit"]
pub type Soc10ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC11OVF` reader - SOC11 Clear Start of Conversion Overflow Bit"]
pub type Soc11ovfR = crate::BitReader;
#[doc = "Field `SOC11OVF` writer - SOC11 Clear Start of Conversion Overflow Bit"]
pub type Soc11ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC12OVF` reader - SOC12 Clear Start of Conversion Overflow Bit"]
pub type Soc12ovfR = crate::BitReader;
#[doc = "Field `SOC12OVF` writer - SOC12 Clear Start of Conversion Overflow Bit"]
pub type Soc12ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC13OVF` reader - SOC13 Clear Start of Conversion Overflow Bit"]
pub type Soc13ovfR = crate::BitReader;
#[doc = "Field `SOC13OVF` writer - SOC13 Clear Start of Conversion Overflow Bit"]
pub type Soc13ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC14OVF` reader - SOC14 Clear Start of Conversion Overflow Bit"]
pub type Soc14ovfR = crate::BitReader;
#[doc = "Field `SOC14OVF` writer - SOC14 Clear Start of Conversion Overflow Bit"]
pub type Soc14ovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC15OVF` reader - SOC15 Clear Start of Conversion Overflow Bit"]
pub type Soc15ovfR = crate::BitReader;
#[doc = "Field `SOC15OVF` writer - SOC15 Clear Start of Conversion Overflow Bit"]
pub type Soc15ovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOC0 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc0ovf(&self) -> Soc0ovfR {
        Soc0ovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOC1 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc1ovf(&self) -> Soc1ovfR {
        Soc1ovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOC2 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc2ovf(&self) -> Soc2ovfR {
        Soc2ovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOC3 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc3ovf(&self) -> Soc3ovfR {
        Soc3ovfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOC4 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc4ovf(&self) -> Soc4ovfR {
        Soc4ovfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SOC5 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc5ovf(&self) -> Soc5ovfR {
        Soc5ovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SOC6 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc6ovf(&self) -> Soc6ovfR {
        Soc6ovfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SOC7 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc7ovf(&self) -> Soc7ovfR {
        Soc7ovfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SOC8 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc8ovf(&self) -> Soc8ovfR {
        Soc8ovfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOC9 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc9ovf(&self) -> Soc9ovfR {
        Soc9ovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOC10 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc10ovf(&self) -> Soc10ovfR {
        Soc10ovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SOC11 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc11ovf(&self) -> Soc11ovfR {
        Soc11ovfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SOC12 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc12ovf(&self) -> Soc12ovfR {
        Soc12ovfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SOC13 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc13ovf(&self) -> Soc13ovfR {
        Soc13ovfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SOC14 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc14ovf(&self) -> Soc14ovfR {
        Soc14ovfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOC15 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc15ovf(&self) -> Soc15ovfR {
        Soc15ovfR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOCOVFCLR1")
            .field("soc0ovf", &self.soc0ovf())
            .field("soc1ovf", &self.soc1ovf())
            .field("soc2ovf", &self.soc2ovf())
            .field("soc3ovf", &self.soc3ovf())
            .field("soc4ovf", &self.soc4ovf())
            .field("soc5ovf", &self.soc5ovf())
            .field("soc6ovf", &self.soc6ovf())
            .field("soc7ovf", &self.soc7ovf())
            .field("soc8ovf", &self.soc8ovf())
            .field("soc9ovf", &self.soc9ovf())
            .field("soc10ovf", &self.soc10ovf())
            .field("soc11ovf", &self.soc11ovf())
            .field("soc12ovf", &self.soc12ovf())
            .field("soc13ovf", &self.soc13ovf())
            .field("soc14ovf", &self.soc14ovf())
            .field("soc15ovf", &self.soc15ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SOC0 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc0ovf(&mut self) -> Soc0ovfW<'_, Adcsocovfclr1Spec> {
        Soc0ovfW::new(self, 0)
    }
    #[doc = "Bit 1 - SOC1 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc1ovf(&mut self) -> Soc1ovfW<'_, Adcsocovfclr1Spec> {
        Soc1ovfW::new(self, 1)
    }
    #[doc = "Bit 2 - SOC2 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc2ovf(&mut self) -> Soc2ovfW<'_, Adcsocovfclr1Spec> {
        Soc2ovfW::new(self, 2)
    }
    #[doc = "Bit 3 - SOC3 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc3ovf(&mut self) -> Soc3ovfW<'_, Adcsocovfclr1Spec> {
        Soc3ovfW::new(self, 3)
    }
    #[doc = "Bit 4 - SOC4 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc4ovf(&mut self) -> Soc4ovfW<'_, Adcsocovfclr1Spec> {
        Soc4ovfW::new(self, 4)
    }
    #[doc = "Bit 5 - SOC5 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc5ovf(&mut self) -> Soc5ovfW<'_, Adcsocovfclr1Spec> {
        Soc5ovfW::new(self, 5)
    }
    #[doc = "Bit 6 - SOC6 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc6ovf(&mut self) -> Soc6ovfW<'_, Adcsocovfclr1Spec> {
        Soc6ovfW::new(self, 6)
    }
    #[doc = "Bit 7 - SOC7 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc7ovf(&mut self) -> Soc7ovfW<'_, Adcsocovfclr1Spec> {
        Soc7ovfW::new(self, 7)
    }
    #[doc = "Bit 8 - SOC8 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc8ovf(&mut self) -> Soc8ovfW<'_, Adcsocovfclr1Spec> {
        Soc8ovfW::new(self, 8)
    }
    #[doc = "Bit 9 - SOC9 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc9ovf(&mut self) -> Soc9ovfW<'_, Adcsocovfclr1Spec> {
        Soc9ovfW::new(self, 9)
    }
    #[doc = "Bit 10 - SOC10 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc10ovf(&mut self) -> Soc10ovfW<'_, Adcsocovfclr1Spec> {
        Soc10ovfW::new(self, 10)
    }
    #[doc = "Bit 11 - SOC11 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc11ovf(&mut self) -> Soc11ovfW<'_, Adcsocovfclr1Spec> {
        Soc11ovfW::new(self, 11)
    }
    #[doc = "Bit 12 - SOC12 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc12ovf(&mut self) -> Soc12ovfW<'_, Adcsocovfclr1Spec> {
        Soc12ovfW::new(self, 12)
    }
    #[doc = "Bit 13 - SOC13 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc13ovf(&mut self) -> Soc13ovfW<'_, Adcsocovfclr1Spec> {
        Soc13ovfW::new(self, 13)
    }
    #[doc = "Bit 14 - SOC14 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc14ovf(&mut self) -> Soc14ovfW<'_, Adcsocovfclr1Spec> {
        Soc14ovfW::new(self, 14)
    }
    #[doc = "Bit 15 - SOC15 Clear Start of Conversion Overflow Bit"]
    #[inline(always)]
    pub fn soc15ovf(&mut self) -> Soc15ovfW<'_, Adcsocovfclr1Spec> {
        Soc15ovfW::new(self, 15)
    }
}
#[doc = "ADC SOC Overflow Clear 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocovfclr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocovfclr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcsocovfclr1Spec;
impl crate::RegisterSpec for Adcsocovfclr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsocovfclr1::R`](R) reader structure"]
impl crate::Readable for Adcsocovfclr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcsocovfclr1::W`](W) writer structure"]
impl crate::Writable for Adcsocovfclr1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOCOVFCLR1 to value 0"]
impl crate::Resettable for Adcsocovfclr1Spec {}
