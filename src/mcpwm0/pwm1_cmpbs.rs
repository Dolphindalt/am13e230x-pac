#[doc = "Register `PWM1_CMPBS` reader"]
pub type R = crate::R<Pwm1CmpbsSpec>;
#[doc = "Register `PWM1_CMPBS` writer"]
pub type W = crate::W<Pwm1CmpbsSpec>;
#[doc = "Field `PWM1_CMPBS` reader - PWM1 Compare B shadow register"]
pub type Pwm1CmpbsR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_CMPBS` writer - PWM1 Compare B shadow register"]
pub type Pwm1CmpbsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM1 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm1_cmpbs(&self) -> Pwm1CmpbsR {
        Pwm1CmpbsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_CMPBS")
            .field("pwm1_cmpbs", &self.pwm1_cmpbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM1 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm1_cmpbs(&mut self) -> Pwm1CmpbsW<'_, Pwm1CmpbsSpec> {
        Pwm1CmpbsW::new(self, 0)
    }
}
#[doc = "PWM1 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1CmpbsSpec;
impl crate::RegisterSpec for Pwm1CmpbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_cmpbs::R`](R) reader structure"]
impl crate::Readable for Pwm1CmpbsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_cmpbs::W`](W) writer structure"]
impl crate::Writable for Pwm1CmpbsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_CMPBS to value 0"]
impl crate::Resettable for Pwm1CmpbsSpec {}
