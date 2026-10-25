#[doc = "Register `XBARFLG1` reader"]
pub type R = crate::R<Xbarflg1Spec>;
#[doc = "Register `XBARFLG1` writer"]
pub type W = crate::W<Xbarflg1Spec>;
#[doc = "Field `CMPSS0_CTRIPH` reader - Input Flag for CMPSS0_CTRIPH Signal"]
pub type Cmpss0CtriphR = crate::BitReader;
#[doc = "Field `CMPSS0_CTRIPL` reader - Input Flag for CMPSS0_CTRIPL Signal"]
pub type Cmpss0CtriplR = crate::BitReader;
#[doc = "Field `CMPSS1_CTRIPH` reader - Input Flag for CMPSS1_CTRIPH Signal"]
pub type Cmpss1CtriphR = crate::BitReader;
#[doc = "Field `CMPSS1_CTRIPL` reader - Input Flag for CMPSS1_CTRIPL Signal"]
pub type Cmpss1CtriplR = crate::BitReader;
#[doc = "Field `CMPSS2_CTRIPH` reader - Input Flag for CMPSS2_CTRIPH Signal"]
pub type Cmpss2CtriphR = crate::BitReader;
#[doc = "Field `CMPSS2_CTRIPL` reader - Input Flag for CMPSS2_CTRIPL Signal"]
pub type Cmpss2CtriplR = crate::BitReader;
#[doc = "Field `CMPSS3_CTRIPH` reader - Input Flag for CMPSS3_CTRIPH Signal"]
pub type Cmpss3CtriphR = crate::BitReader;
#[doc = "Field `CMPSS3_CTRIPL` reader - Input Flag for CMPSS3_CTRIPL Signal"]
pub type Cmpss3CtriplR = crate::BitReader;
#[doc = "Field `ADC0EVT1` reader - Input Flag for ADC0EVT1 Signal"]
pub type Adc0evt1R = crate::BitReader;
#[doc = "Field `ADC0EVT2` reader - Input Flag for ADC0EVT2 Signal"]
pub type Adc0evt2R = crate::BitReader;
#[doc = "Field `ADC0EVT3` reader - Input Flag for ADC0EVT3 Signal"]
pub type Adc0evt3R = crate::BitReader;
#[doc = "Field `ADC0EVT4` reader - Input Flag for ADC0EVT4 Signal"]
pub type Adc0evt4R = crate::BitReader;
#[doc = "Field `ADC1EVT1` reader - Input Flag for ADC1EVT1 Signal"]
pub type Adc1evt1R = crate::BitReader;
#[doc = "Field `ADC1EVT2` reader - Input Flag for ADC1EVT2 Signal"]
pub type Adc1evt2R = crate::BitReader;
#[doc = "Field `ADC1EVT3` reader - Input Flag for ADC1EVT3 Signal"]
pub type Adc1evt3R = crate::BitReader;
#[doc = "Field `ADC1EVT4` reader - Input Flag for ADC1EVT4 Signal"]
pub type Adc1evt4R = crate::BitReader;
#[doc = "Field `ADC2EVT1` reader - Input Flag for ADC2EVT1 Signal"]
pub type Adc2evt1R = crate::BitReader;
#[doc = "Field `ADC2EVT2` reader - Input Flag for ADC2EVT2 Signal"]
pub type Adc2evt2R = crate::BitReader;
#[doc = "Field `ADC2EVT3` reader - Input Flag for ADC2EVT3 Signal"]
pub type Adc2evt3R = crate::BitReader;
#[doc = "Field `ADC2EVT4` reader - Input Flag for ADC2EVT4 Signal"]
pub type Adc2evt4R = crate::BitReader;
#[doc = "Field `INPUTXBAR1` reader - Input Flag for INPUTXBAR1 Signal"]
pub type Inputxbar1R = crate::BitReader;
#[doc = "Field `INPUTXBAR2` reader - Input Flag for INPUTXBAR2 Signal"]
pub type Inputxbar2R = crate::BitReader;
#[doc = "Field `INPUTXBAR3` reader - Input Flag for INPUTXBAR3 Signal"]
pub type Inputxbar3R = crate::BitReader;
#[doc = "Field `INPUTXBAR4` reader - Input Flag for INPUTXBAR4 Signal"]
pub type Inputxbar4R = crate::BitReader;
#[doc = "Field `INPUTXBAR5` reader - Input Flag for INPUTXBAR5 Signal"]
pub type Inputxbar5R = crate::BitReader;
#[doc = "Field `INPUTXBAR6` reader - Input Flag for INPUTXBAR6 Signal"]
pub type Inputxbar6R = crate::BitReader;
#[doc = "Field `INPUTXBAR7` reader - Input Flag for INPUTXBAR7 Signal"]
pub type Inputxbar7R = crate::BitReader;
#[doc = "Field `INPUTXBAR8` reader - Input Flag for INPUTXBAR8 Signal"]
pub type Inputxbar8R = crate::BitReader;
#[doc = "Field `INPUTXBAR9` reader - Input Flag for INPUTXBAR9 Signal"]
pub type Inputxbar9R = crate::BitReader;
#[doc = "Field `INPUTXBAR10` reader - Input Flag for INPUTXBAR10 Signal"]
pub type Inputxbar10R = crate::BitReader;
#[doc = "Field `INPUTXBAR11` reader - Input Flag for INPUTXBAR11 Signal"]
pub type Inputxbar11R = crate::BitReader;
#[doc = "Field `INPUTXBAR12` reader - Input Flag for INPUTXBAR12 Signal"]
pub type Inputxbar12R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input Flag for CMPSS0_CTRIPH Signal"]
    #[inline(always)]
    pub fn cmpss0_ctriph(&self) -> Cmpss0CtriphR {
        Cmpss0CtriphR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Flag for CMPSS0_CTRIPL Signal"]
    #[inline(always)]
    pub fn cmpss0_ctripl(&self) -> Cmpss0CtriplR {
        Cmpss0CtriplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Flag for CMPSS1_CTRIPH Signal"]
    #[inline(always)]
    pub fn cmpss1_ctriph(&self) -> Cmpss1CtriphR {
        Cmpss1CtriphR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Flag for CMPSS1_CTRIPL Signal"]
    #[inline(always)]
    pub fn cmpss1_ctripl(&self) -> Cmpss1CtriplR {
        Cmpss1CtriplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Flag for CMPSS2_CTRIPH Signal"]
    #[inline(always)]
    pub fn cmpss2_ctriph(&self) -> Cmpss2CtriphR {
        Cmpss2CtriphR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Flag for CMPSS2_CTRIPL Signal"]
    #[inline(always)]
    pub fn cmpss2_ctripl(&self) -> Cmpss2CtriplR {
        Cmpss2CtriplR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Flag for CMPSS3_CTRIPH Signal"]
    #[inline(always)]
    pub fn cmpss3_ctriph(&self) -> Cmpss3CtriphR {
        Cmpss3CtriphR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Flag for CMPSS3_CTRIPL Signal"]
    #[inline(always)]
    pub fn cmpss3_ctripl(&self) -> Cmpss3CtriplR {
        Cmpss3CtriplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input Flag for ADC0EVT1 Signal"]
    #[inline(always)]
    pub fn adc0evt1(&self) -> Adc0evt1R {
        Adc0evt1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input Flag for ADC0EVT2 Signal"]
    #[inline(always)]
    pub fn adc0evt2(&self) -> Adc0evt2R {
        Adc0evt2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input Flag for ADC0EVT3 Signal"]
    #[inline(always)]
    pub fn adc0evt3(&self) -> Adc0evt3R {
        Adc0evt3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Flag for ADC0EVT4 Signal"]
    #[inline(always)]
    pub fn adc0evt4(&self) -> Adc0evt4R {
        Adc0evt4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input Flag for ADC1EVT1 Signal"]
    #[inline(always)]
    pub fn adc1evt1(&self) -> Adc1evt1R {
        Adc1evt1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input Flag for ADC1EVT2 Signal"]
    #[inline(always)]
    pub fn adc1evt2(&self) -> Adc1evt2R {
        Adc1evt2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input Flag for ADC1EVT3 Signal"]
    #[inline(always)]
    pub fn adc1evt3(&self) -> Adc1evt3R {
        Adc1evt3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input Flag for ADC1EVT4 Signal"]
    #[inline(always)]
    pub fn adc1evt4(&self) -> Adc1evt4R {
        Adc1evt4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input Flag for ADC2EVT1 Signal"]
    #[inline(always)]
    pub fn adc2evt1(&self) -> Adc2evt1R {
        Adc2evt1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input Flag for ADC2EVT2 Signal"]
    #[inline(always)]
    pub fn adc2evt2(&self) -> Adc2evt2R {
        Adc2evt2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Flag for ADC2EVT3 Signal"]
    #[inline(always)]
    pub fn adc2evt3(&self) -> Adc2evt3R {
        Adc2evt3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input Flag for ADC2EVT4 Signal"]
    #[inline(always)]
    pub fn adc2evt4(&self) -> Adc2evt4R {
        Adc2evt4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input Flag for INPUTXBAR1 Signal"]
    #[inline(always)]
    pub fn inputxbar1(&self) -> Inputxbar1R {
        Inputxbar1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Flag for INPUTXBAR2 Signal"]
    #[inline(always)]
    pub fn inputxbar2(&self) -> Inputxbar2R {
        Inputxbar2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input Flag for INPUTXBAR3 Signal"]
    #[inline(always)]
    pub fn inputxbar3(&self) -> Inputxbar3R {
        Inputxbar3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input Flag for INPUTXBAR4 Signal"]
    #[inline(always)]
    pub fn inputxbar4(&self) -> Inputxbar4R {
        Inputxbar4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input Flag for INPUTXBAR5 Signal"]
    #[inline(always)]
    pub fn inputxbar5(&self) -> Inputxbar5R {
        Inputxbar5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input Flag for INPUTXBAR6 Signal"]
    #[inline(always)]
    pub fn inputxbar6(&self) -> Inputxbar6R {
        Inputxbar6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input Flag for INPUTXBAR7 Signal"]
    #[inline(always)]
    pub fn inputxbar7(&self) -> Inputxbar7R {
        Inputxbar7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input Flag for INPUTXBAR8 Signal"]
    #[inline(always)]
    pub fn inputxbar8(&self) -> Inputxbar8R {
        Inputxbar8R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input Flag for INPUTXBAR9 Signal"]
    #[inline(always)]
    pub fn inputxbar9(&self) -> Inputxbar9R {
        Inputxbar9R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input Flag for INPUTXBAR10 Signal"]
    #[inline(always)]
    pub fn inputxbar10(&self) -> Inputxbar10R {
        Inputxbar10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input Flag for INPUTXBAR11 Signal"]
    #[inline(always)]
    pub fn inputxbar11(&self) -> Inputxbar11R {
        Inputxbar11R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input Flag for INPUTXBAR12 Signal"]
    #[inline(always)]
    pub fn inputxbar12(&self) -> Inputxbar12R {
        Inputxbar12R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XBARFLG1")
            .field("cmpss0_ctriph", &self.cmpss0_ctriph())
            .field("cmpss0_ctripl", &self.cmpss0_ctripl())
            .field("cmpss1_ctriph", &self.cmpss1_ctriph())
            .field("cmpss1_ctripl", &self.cmpss1_ctripl())
            .field("cmpss2_ctriph", &self.cmpss2_ctriph())
            .field("cmpss2_ctripl", &self.cmpss2_ctripl())
            .field("cmpss3_ctriph", &self.cmpss3_ctriph())
            .field("cmpss3_ctripl", &self.cmpss3_ctripl())
            .field("adc0evt1", &self.adc0evt1())
            .field("adc0evt2", &self.adc0evt2())
            .field("adc0evt3", &self.adc0evt3())
            .field("adc0evt4", &self.adc0evt4())
            .field("adc1evt1", &self.adc1evt1())
            .field("adc1evt2", &self.adc1evt2())
            .field("adc1evt3", &self.adc1evt3())
            .field("adc1evt4", &self.adc1evt4())
            .field("adc2evt1", &self.adc2evt1())
            .field("adc2evt2", &self.adc2evt2())
            .field("adc2evt3", &self.adc2evt3())
            .field("adc2evt4", &self.adc2evt4())
            .field("inputxbar1", &self.inputxbar1())
            .field("inputxbar2", &self.inputxbar2())
            .field("inputxbar3", &self.inputxbar3())
            .field("inputxbar4", &self.inputxbar4())
            .field("inputxbar5", &self.inputxbar5())
            .field("inputxbar6", &self.inputxbar6())
            .field("inputxbar7", &self.inputxbar7())
            .field("inputxbar8", &self.inputxbar8())
            .field("inputxbar9", &self.inputxbar9())
            .field("inputxbar10", &self.inputxbar10())
            .field("inputxbar11", &self.inputxbar11())
            .field("inputxbar12", &self.inputxbar12())
            .finish()
    }
}
impl W {}
#[doc = "X-Bar Input Flag Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarflg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarflg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xbarflg1Spec;
impl crate::RegisterSpec for Xbarflg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xbarflg1::R`](R) reader structure"]
impl crate::Readable for Xbarflg1Spec {}
#[doc = "`write(|w| ..)` method takes [`xbarflg1::W`](W) writer structure"]
impl crate::Writable for Xbarflg1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets XBARFLG1 to value 0"]
impl crate::Resettable for Xbarflg1Spec {}
