#[doc = "Register `ADCSOCOUTSELECT` reader"]
pub type R = crate::R<AdcsocoutselectSpec>;
#[doc = "Register `ADCSOCOUTSELECT` writer"]
pub type W = crate::W<AdcsocoutselectSpec>;
#[doc = "Field `PWM0SOCAEN` reader - PWM0SOCAEN Enable for ADCSOCAOn"]
pub type Pwm0socaenR = crate::BitReader;
#[doc = "Field `PWM0SOCAEN` writer - PWM0SOCAEN Enable for ADCSOCAOn"]
pub type Pwm0socaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1SOCAEN` reader - PWM1SOCAEN Enable for ADCSOCAOn"]
pub type Pwm1socaenR = crate::BitReader;
#[doc = "Field `PWM1SOCAEN` writer - PWM1SOCAEN Enable for ADCSOCAOn"]
pub type Pwm1socaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2SOCAEN` reader - PWM2SOCAEN Enable for ADCSOCAOn"]
pub type Pwm2socaenR = crate::BitReader;
#[doc = "Field `PWM2SOCAEN` writer - PWM2SOCAEN Enable for ADCSOCAOn"]
pub type Pwm2socaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3SOCAEN` reader - PWM3SOCAEN Enable for ADCSOCAOn"]
pub type Pwm3socaenR = crate::BitReader;
#[doc = "Field `PWM3SOCAEN` writer - PWM3SOCAEN Enable for ADCSOCAOn"]
pub type Pwm3socaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4SOCAEN` reader - PWM4SOCAEN Enable for ADCSOCAOn"]
pub type Pwm4socaenR = crate::BitReader;
#[doc = "Field `PWM4SOCAEN` writer - PWM4SOCAEN Enable for ADCSOCAOn"]
pub type Pwm4socaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0SOCBEN` reader - PWM0SOCBEN Enable for ADCSOCBOn"]
pub type Pwm0socbenR = crate::BitReader;
#[doc = "Field `PWM0SOCBEN` writer - PWM0SOCBEN Enable for ADCSOCBOn"]
pub type Pwm0socbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1SOCBEN` reader - PWM1SOCBEN Enable for ADCSOCBOn"]
pub type Pwm1socbenR = crate::BitReader;
#[doc = "Field `PWM1SOCBEN` writer - PWM1SOCBEN Enable for ADCSOCBOn"]
pub type Pwm1socbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2SOCBEN` reader - PWM2SOCBEN Enable for ADCSOCBOn"]
pub type Pwm2socbenR = crate::BitReader;
#[doc = "Field `PWM2SOCBEN` writer - PWM2SOCBEN Enable for ADCSOCBOn"]
pub type Pwm2socbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3SOCBEN` reader - PWM3SOCBEN Enable for ADCSOCBOn"]
pub type Pwm3socbenR = crate::BitReader;
#[doc = "Field `PWM3SOCBEN` writer - PWM3SOCBEN Enable for ADCSOCBOn"]
pub type Pwm3socbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4SOCBEN` reader - PWM4SOCBEN Enable for ADCSOCBOn"]
pub type Pwm4socbenR = crate::BitReader;
#[doc = "Field `PWM4SOCBEN` writer - PWM4SOCBEN Enable for ADCSOCBOn"]
pub type Pwm4socbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM0SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm0socaen(&self) -> Pwm0socaenR {
        Pwm0socaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm1socaen(&self) -> Pwm1socaenR {
        Pwm1socaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm2socaen(&self) -> Pwm2socaenR {
        Pwm2socaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm3socaen(&self) -> Pwm3socaenR {
        Pwm3socaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm4socaen(&self) -> Pwm4socaenR {
        Pwm4socaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM0SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm0socben(&self) -> Pwm0socbenR {
        Pwm0socbenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM1SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm1socben(&self) -> Pwm1socbenR {
        Pwm1socbenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM2SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm2socben(&self) -> Pwm2socbenR {
        Pwm2socbenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM3SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm3socben(&self) -> Pwm3socbenR {
        Pwm3socbenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM4SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm4socben(&self) -> Pwm4socbenR {
        Pwm4socbenR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOCOUTSELECT")
            .field("pwm0socaen", &self.pwm0socaen())
            .field("pwm1socaen", &self.pwm1socaen())
            .field("pwm2socaen", &self.pwm2socaen())
            .field("pwm3socaen", &self.pwm3socaen())
            .field("pwm4socaen", &self.pwm4socaen())
            .field("pwm0socben", &self.pwm0socben())
            .field("pwm1socben", &self.pwm1socben())
            .field("pwm2socben", &self.pwm2socben())
            .field("pwm3socben", &self.pwm3socben())
            .field("pwm4socben", &self.pwm4socben())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PWM0SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm0socaen(&mut self) -> Pwm0socaenW<'_, AdcsocoutselectSpec> {
        Pwm0socaenW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM1SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm1socaen(&mut self) -> Pwm1socaenW<'_, AdcsocoutselectSpec> {
        Pwm1socaenW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM2SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm2socaen(&mut self) -> Pwm2socaenW<'_, AdcsocoutselectSpec> {
        Pwm2socaenW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM3SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm3socaen(&mut self) -> Pwm3socaenW<'_, AdcsocoutselectSpec> {
        Pwm3socaenW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM4SOCAEN Enable for ADCSOCAOn"]
    #[inline(always)]
    pub fn pwm4socaen(&mut self) -> Pwm4socaenW<'_, AdcsocoutselectSpec> {
        Pwm4socaenW::new(self, 4)
    }
    #[doc = "Bit 16 - PWM0SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm0socben(&mut self) -> Pwm0socbenW<'_, AdcsocoutselectSpec> {
        Pwm0socbenW::new(self, 16)
    }
    #[doc = "Bit 17 - PWM1SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm1socben(&mut self) -> Pwm1socbenW<'_, AdcsocoutselectSpec> {
        Pwm1socbenW::new(self, 17)
    }
    #[doc = "Bit 18 - PWM2SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm2socben(&mut self) -> Pwm2socbenW<'_, AdcsocoutselectSpec> {
        Pwm2socbenW::new(self, 18)
    }
    #[doc = "Bit 19 - PWM3SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm3socben(&mut self) -> Pwm3socbenW<'_, AdcsocoutselectSpec> {
        Pwm3socbenW::new(self, 19)
    }
    #[doc = "Bit 20 - PWM4SOCBEN Enable for ADCSOCBOn"]
    #[inline(always)]
    pub fn pwm4socben(&mut self) -> Pwm4socbenW<'_, AdcsocoutselectSpec> {
        Pwm4socbenW::new(self, 20)
    }
}
#[doc = "External ADCSOC Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocoutselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocoutselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcsocoutselectSpec;
impl crate::RegisterSpec for AdcsocoutselectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsocoutselect::R`](R) reader structure"]
impl crate::Readable for AdcsocoutselectSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsocoutselect::W`](W) writer structure"]
impl crate::Writable for AdcsocoutselectSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOCOUTSELECT to value 0"]
impl crate::Resettable for AdcsocoutselectSpec {}
