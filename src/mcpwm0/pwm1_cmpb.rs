#[doc = "Register `PWM1_CMPB` reader"]
pub type R = crate::R<Pwm1CmpbSpec>;
#[doc = "Register `PWM1_CMPB` writer"]
pub type W = crate::W<Pwm1CmpbSpec>;
#[doc = "Field `PWM1_CMPB` reader - PWM1 Compare B register"]
pub type Pwm1CmpbR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_CMPB` writer - PWM1 Compare B register"]
pub type Pwm1CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM1 Compare B register"]
    #[inline(always)]
    pub fn pwm1_cmpb(&self) -> Pwm1CmpbR {
        Pwm1CmpbR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_CMPB")
            .field("pwm1_cmpb", &self.pwm1_cmpb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM1 Compare B register"]
    #[inline(always)]
    pub fn pwm1_cmpb(&mut self) -> Pwm1CmpbW<'_, Pwm1CmpbSpec> {
        Pwm1CmpbW::new(self, 0)
    }
}
#[doc = "PWM1 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1CmpbSpec;
impl crate::RegisterSpec for Pwm1CmpbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_cmpb::R`](R) reader structure"]
impl crate::Readable for Pwm1CmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_cmpb::W`](W) writer structure"]
impl crate::Writable for Pwm1CmpbSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_CMPB to value 0"]
impl crate::Resettable for Pwm1CmpbSpec {}
