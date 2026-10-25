#[doc = "Register `ADCSOCOVF1` reader"]
pub type R = crate::R<Adcsocovf1Spec>;
#[doc = "Register `ADCSOCOVF1` writer"]
pub type W = crate::W<Adcsocovf1Spec>;
#[doc = "Field `SOC0OVF` reader - SOC0 Start of Conversion Overflow Flag"]
pub type Soc0ovfR = crate::BitReader;
#[doc = "Field `SOC1OVF` reader - SOC1 Start of Conversion Overflow Flag"]
pub type Soc1ovfR = crate::BitReader;
#[doc = "Field `SOC2OVF` reader - SOC2 Start of Conversion Overflow Flag"]
pub type Soc2ovfR = crate::BitReader;
#[doc = "Field `SOC3OVF` reader - SOC3 Start of Conversion Overflow Flag"]
pub type Soc3ovfR = crate::BitReader;
#[doc = "Field `SOC4OVF` reader - SOC4 Start of Conversion Overflow Flag"]
pub type Soc4ovfR = crate::BitReader;
#[doc = "Field `SOC5OVF` reader - SOC5 Start of Conversion Overflow Flag"]
pub type Soc5ovfR = crate::BitReader;
#[doc = "Field `SOC6OVF` reader - SOC6 Start of Conversion Overflow Flag"]
pub type Soc6ovfR = crate::BitReader;
#[doc = "Field `SOC7OVF` reader - SOC7 Start of Conversion Overflow Flag"]
pub type Soc7ovfR = crate::BitReader;
#[doc = "Field `SOC8OVF` reader - SOC8 Start of Conversion Overflow Flag"]
pub type Soc8ovfR = crate::BitReader;
#[doc = "Field `SOC9OVF` reader - SOC9 Start of Conversion Overflow Flag"]
pub type Soc9ovfR = crate::BitReader;
#[doc = "Field `SOC10OVF` reader - SOC10 Start of Conversion Overflow Flag"]
pub type Soc10ovfR = crate::BitReader;
#[doc = "Field `SOC11OVF` reader - SOC11 Start of Conversion Overflow Flag"]
pub type Soc11ovfR = crate::BitReader;
#[doc = "Field `SOC12OVF` reader - SOC12 Start of Conversion Overflow Flag"]
pub type Soc12ovfR = crate::BitReader;
#[doc = "Field `SOC13OVF` reader - SOC13 Start of Conversion Overflow Flag"]
pub type Soc13ovfR = crate::BitReader;
#[doc = "Field `SOC14OVF` reader - SOC14 Start of Conversion Overflow Flag"]
pub type Soc14ovfR = crate::BitReader;
#[doc = "Field `SOC15OVF` reader - SOC15 Start of Conversion Overflow Flag"]
pub type Soc15ovfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOC0 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc0ovf(&self) -> Soc0ovfR {
        Soc0ovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOC1 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc1ovf(&self) -> Soc1ovfR {
        Soc1ovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOC2 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc2ovf(&self) -> Soc2ovfR {
        Soc2ovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOC3 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc3ovf(&self) -> Soc3ovfR {
        Soc3ovfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOC4 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc4ovf(&self) -> Soc4ovfR {
        Soc4ovfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SOC5 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc5ovf(&self) -> Soc5ovfR {
        Soc5ovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SOC6 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc6ovf(&self) -> Soc6ovfR {
        Soc6ovfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SOC7 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc7ovf(&self) -> Soc7ovfR {
        Soc7ovfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SOC8 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc8ovf(&self) -> Soc8ovfR {
        Soc8ovfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOC9 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc9ovf(&self) -> Soc9ovfR {
        Soc9ovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOC10 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc10ovf(&self) -> Soc10ovfR {
        Soc10ovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SOC11 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc11ovf(&self) -> Soc11ovfR {
        Soc11ovfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SOC12 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc12ovf(&self) -> Soc12ovfR {
        Soc12ovfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SOC13 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc13ovf(&self) -> Soc13ovfR {
        Soc13ovfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SOC14 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc14ovf(&self) -> Soc14ovfR {
        Soc14ovfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOC15 Start of Conversion Overflow Flag"]
    #[inline(always)]
    pub fn soc15ovf(&self) -> Soc15ovfR {
        Soc15ovfR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOCOVF1")
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
impl W {}
#[doc = "ADC SOC Overflow 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocovf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocovf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcsocovf1Spec;
impl crate::RegisterSpec for Adcsocovf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsocovf1::R`](R) reader structure"]
impl crate::Readable for Adcsocovf1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcsocovf1::W`](W) writer structure"]
impl crate::Writable for Adcsocovf1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOCOVF1 to value 0"]
impl crate::Resettable for Adcsocovf1Spec {}
