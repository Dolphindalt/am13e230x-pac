#[doc = "Register `ADCSOCFLG1` reader"]
pub type R = crate::R<Adcsocflg1Spec>;
#[doc = "Register `ADCSOCFLG1` writer"]
pub type W = crate::W<Adcsocflg1Spec>;
#[doc = "Field `SOC0` reader - SOC0 Start of Conversion Flag"]
pub type Soc0R = crate::BitReader;
#[doc = "Field `SOC1` reader - SOC1 Start of Conversion Flag"]
pub type Soc1R = crate::BitReader;
#[doc = "Field `SOC2` reader - SOC2 Start of Conversion Flag"]
pub type Soc2R = crate::BitReader;
#[doc = "Field `SOC3` reader - SOC3 Start of Conversion Flag"]
pub type Soc3R = crate::BitReader;
#[doc = "Field `SOC4` reader - SOC4 Start of Conversion Flag"]
pub type Soc4R = crate::BitReader;
#[doc = "Field `SOC5` reader - SOC5 Start of Conversion Flag"]
pub type Soc5R = crate::BitReader;
#[doc = "Field `SOC6` reader - SOC6 Start of Conversion Flag"]
pub type Soc6R = crate::BitReader;
#[doc = "Field `SOC7` reader - SOC7 Start of Conversion Flag"]
pub type Soc7R = crate::BitReader;
#[doc = "Field `SOC8` reader - SOC8 Start of Conversion Flag"]
pub type Soc8R = crate::BitReader;
#[doc = "Field `SOC9` reader - SOC9 Start of Conversion Flag"]
pub type Soc9R = crate::BitReader;
#[doc = "Field `SOC10` reader - SOC10 Start of Conversion Flag"]
pub type Soc10R = crate::BitReader;
#[doc = "Field `SOC11` reader - SOC11 Start of Conversion Flag"]
pub type Soc11R = crate::BitReader;
#[doc = "Field `SOC12` reader - SOC12 Start of Conversion Flag"]
pub type Soc12R = crate::BitReader;
#[doc = "Field `SOC13` reader - SOC13 Start of Conversion Flag"]
pub type Soc13R = crate::BitReader;
#[doc = "Field `SOC14` reader - SOC14 Start of Conversion Flag"]
pub type Soc14R = crate::BitReader;
#[doc = "Field `SOC15` reader - SOC15 Start of Conversion Flag"]
pub type Soc15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOC0 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc0(&self) -> Soc0R {
        Soc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOC1 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc1(&self) -> Soc1R {
        Soc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOC2 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc2(&self) -> Soc2R {
        Soc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOC3 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc3(&self) -> Soc3R {
        Soc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOC4 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc4(&self) -> Soc4R {
        Soc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SOC5 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc5(&self) -> Soc5R {
        Soc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SOC6 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc6(&self) -> Soc6R {
        Soc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SOC7 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc7(&self) -> Soc7R {
        Soc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SOC8 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc8(&self) -> Soc8R {
        Soc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOC9 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc9(&self) -> Soc9R {
        Soc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOC10 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc10(&self) -> Soc10R {
        Soc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SOC11 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc11(&self) -> Soc11R {
        Soc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SOC12 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc12(&self) -> Soc12R {
        Soc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SOC13 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc13(&self) -> Soc13R {
        Soc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SOC14 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc14(&self) -> Soc14R {
        Soc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOC15 Start of Conversion Flag"]
    #[inline(always)]
    pub fn soc15(&self) -> Soc15R {
        Soc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOCFLG1")
            .field("soc0", &self.soc0())
            .field("soc1", &self.soc1())
            .field("soc2", &self.soc2())
            .field("soc3", &self.soc3())
            .field("soc4", &self.soc4())
            .field("soc5", &self.soc5())
            .field("soc6", &self.soc6())
            .field("soc7", &self.soc7())
            .field("soc8", &self.soc8())
            .field("soc9", &self.soc9())
            .field("soc10", &self.soc10())
            .field("soc11", &self.soc11())
            .field("soc12", &self.soc12())
            .field("soc13", &self.soc13())
            .field("soc14", &self.soc14())
            .field("soc15", &self.soc15())
            .finish()
    }
}
impl W {}
#[doc = "ADC SOC Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocflg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocflg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcsocflg1Spec;
impl crate::RegisterSpec for Adcsocflg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsocflg1::R`](R) reader structure"]
impl crate::Readable for Adcsocflg1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcsocflg1::W`](W) writer structure"]
impl crate::Writable for Adcsocflg1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOCFLG1 to value 0"]
impl crate::Resettable for Adcsocflg1Spec {}
