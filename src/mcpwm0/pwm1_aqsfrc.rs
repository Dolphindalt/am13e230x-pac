#[doc = "Register `PWM1_AQSFRC` reader"]
pub type R = crate::R<Pwm1AqsfrcSpec>;
#[doc = "Register `PWM1_AQSFRC` writer"]
pub type W = crate::W<Pwm1AqsfrcSpec>;
#[doc = "Field `PWMA` reader - Action qualifier software force on PWMA"]
pub type PwmaR = crate::FieldReader;
#[doc = "Field `PWMA` writer - Action qualifier software force on PWMA"]
pub type PwmaW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `PWMB` reader - Action qualifier software force on PWMB"]
pub type PwmbR = crate::FieldReader;
#[doc = "Field `PWMB` writer - Action qualifier software force on PWMB"]
pub type PwmbW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - Action qualifier software force on PWMA"]
    #[inline(always)]
    pub fn pwma(&self) -> PwmaR {
        PwmaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Action qualifier software force on PWMB"]
    #[inline(always)]
    pub fn pwmb(&self) -> PwmbR {
        PwmbR::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_AQSFRC")
            .field("pwma", &self.pwma())
            .field("pwmb", &self.pwmb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Action qualifier software force on PWMA"]
    #[inline(always)]
    pub fn pwma(&mut self) -> PwmaW<'_, Pwm1AqsfrcSpec> {
        PwmaW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Action qualifier software force on PWMB"]
    #[inline(always)]
    pub fn pwmb(&mut self) -> PwmbW<'_, Pwm1AqsfrcSpec> {
        PwmbW::new(self, 4)
    }
}
#[doc = "PWM1 action qualifier software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqsfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqsfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1AqsfrcSpec;
impl crate::RegisterSpec for Pwm1AqsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_aqsfrc::R`](R) reader structure"]
impl crate::Readable for Pwm1AqsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_aqsfrc::W`](W) writer structure"]
impl crate::Writable for Pwm1AqsfrcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_AQSFRC to value 0"]
impl crate::Resettable for Pwm1AqsfrcSpec {}
