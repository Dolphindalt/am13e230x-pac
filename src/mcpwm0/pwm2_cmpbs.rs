#[doc = "Register `PWM2_CMPBS` reader"]
pub type R = crate::R<Pwm2CmpbsSpec>;
#[doc = "Register `PWM2_CMPBS` writer"]
pub type W = crate::W<Pwm2CmpbsSpec>;
#[doc = "Field `PWM2_CMPBS` reader - PWM2 Compare B shadow register"]
pub type Pwm2CmpbsR = crate::FieldReader<u16>;
#[doc = "Field `PWM2_CMPBS` writer - PWM2 Compare B shadow register"]
pub type Pwm2CmpbsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM2 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm2_cmpbs(&self) -> Pwm2CmpbsR {
        Pwm2CmpbsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2_CMPBS")
            .field("pwm2_cmpbs", &self.pwm2_cmpbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM2 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm2_cmpbs(&mut self) -> Pwm2CmpbsW<'_, Pwm2CmpbsSpec> {
        Pwm2CmpbsW::new(self, 0)
    }
}
#[doc = "PWM2 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2CmpbsSpec;
impl crate::RegisterSpec for Pwm2CmpbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_cmpbs::R`](R) reader structure"]
impl crate::Readable for Pwm2CmpbsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_cmpbs::W`](W) writer structure"]
impl crate::Writable for Pwm2CmpbsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM2_CMPBS to value 0"]
impl crate::Resettable for Pwm2CmpbsSpec {}
