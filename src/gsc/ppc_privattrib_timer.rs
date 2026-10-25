#[doc = "Register `PPC_PRIVATTRIB_TIMER` reader"]
pub type R = crate::R<PpcPrivattribTimerSpec>;
#[doc = "Register `PPC_PRIVATTRIB_TIMER` writer"]
pub type W = crate::W<PpcPrivattribTimerSpec>;
#[doc = "Field `PRIV_PWM0` reader - Privilege access for PWM0"]
pub type PrivPwm0R = crate::BitReader;
#[doc = "Field `PRIV_PWM0` writer - Privilege access for PWM0"]
pub type PrivPwm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PWM1` reader - Privilege access for PWM1"]
pub type PrivPwm1R = crate::BitReader;
#[doc = "Field `PRIV_PWM1` writer - Privilege access for PWM1"]
pub type PrivPwm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PWM2` reader - Privilege access for PWM2"]
pub type PrivPwm2R = crate::BitReader;
#[doc = "Field `PRIV_PWM2` writer - Privilege access for PWM2"]
pub type PrivPwm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PWM3` reader - Privilege access for PWM3"]
pub type PrivPwm3R = crate::BitReader;
#[doc = "Field `PRIV_PWM3` writer - Privilege access for PWM3"]
pub type PrivPwm3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PWM4` reader - Privilege access for PWM4"]
pub type PrivPwm4R = crate::BitReader;
#[doc = "Field `PRIV_PWM4` writer - Privilege access for PWM4"]
pub type PrivPwm4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ECAP0` reader - Privilege access for ECAP0"]
pub type PrivEcap0R = crate::BitReader;
#[doc = "Field `PRIV_ECAP0` writer - Privilege access for ECAP0"]
pub type PrivEcap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ECAP1` reader - Privilege access for ECAP1"]
pub type PrivEcap1R = crate::BitReader;
#[doc = "Field `PRIV_ECAP1` writer - Privilege access for ECAP1"]
pub type PrivEcap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_EQEP0` reader - Privilege access for EQEP0"]
pub type PrivEqep0R = crate::BitReader;
#[doc = "Field `PRIV_EQEP0` writer - Privilege access for EQEP0"]
pub type PrivEqep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_EQEP1` reader - Privilege access for EQEP1"]
pub type PrivEqep1R = crate::BitReader;
#[doc = "Field `PRIV_EQEP1` writer - Privilege access for EQEP1"]
pub type PrivEqep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_EQEP2` reader - Privilege access for EQEP2"]
pub type PrivEqep2R = crate::BitReader;
#[doc = "Field `PRIV_EQEP2` writer - Privilege access for EQEP2"]
pub type PrivEqep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_TIMG4_0` reader - Privilege access for TIMG4_0"]
pub type PrivTimg4_0R = crate::BitReader;
#[doc = "Field `PRIV_TIMG4_0` writer - Privilege access for TIMG4_0"]
pub type PrivTimg4_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_TIMG12_0` reader - Privilege access for TIMG12_0"]
pub type PrivTimg12_0R = crate::BitReader;
#[doc = "Field `PRIV_TIMG12_0` writer - Privilege access for TIMG12_0"]
pub type PrivTimg12_0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for PWM0"]
    #[inline(always)]
    pub fn priv_pwm0(&self) -> PrivPwm0R {
        PrivPwm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for PWM1"]
    #[inline(always)]
    pub fn priv_pwm1(&self) -> PrivPwm1R {
        PrivPwm1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for PWM2"]
    #[inline(always)]
    pub fn priv_pwm2(&self) -> PrivPwm2R {
        PrivPwm2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privilege access for PWM3"]
    #[inline(always)]
    pub fn priv_pwm3(&self) -> PrivPwm3R {
        PrivPwm3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privilege access for PWM4"]
    #[inline(always)]
    pub fn priv_pwm4(&self) -> PrivPwm4R {
        PrivPwm4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Privilege access for ECAP0"]
    #[inline(always)]
    pub fn priv_ecap0(&self) -> PrivEcap0R {
        PrivEcap0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privilege access for ECAP1"]
    #[inline(always)]
    pub fn priv_ecap1(&self) -> PrivEcap1R {
        PrivEcap1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Privilege access for EQEP0"]
    #[inline(always)]
    pub fn priv_eqep0(&self) -> PrivEqep0R {
        PrivEqep0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Privilege access for EQEP1"]
    #[inline(always)]
    pub fn priv_eqep1(&self) -> PrivEqep1R {
        PrivEqep1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privilege access for EQEP2"]
    #[inline(always)]
    pub fn priv_eqep2(&self) -> PrivEqep2R {
        PrivEqep2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Privilege access for TIMG4_0"]
    #[inline(always)]
    pub fn priv_timg4_0(&self) -> PrivTimg4_0R {
        PrivTimg4_0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Privilege access for TIMG12_0"]
    #[inline(always)]
    pub fn priv_timg12_0(&self) -> PrivTimg12_0R {
        PrivTimg12_0R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_TIMER")
            .field("priv_pwm0", &self.priv_pwm0())
            .field("priv_pwm1", &self.priv_pwm1())
            .field("priv_pwm2", &self.priv_pwm2())
            .field("priv_pwm3", &self.priv_pwm3())
            .field("priv_pwm4", &self.priv_pwm4())
            .field("priv_ecap0", &self.priv_ecap0())
            .field("priv_ecap1", &self.priv_ecap1())
            .field("priv_eqep0", &self.priv_eqep0())
            .field("priv_eqep1", &self.priv_eqep1())
            .field("priv_eqep2", &self.priv_eqep2())
            .field("priv_timg4_0", &self.priv_timg4_0())
            .field("priv_timg12_0", &self.priv_timg12_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for PWM0"]
    #[inline(always)]
    pub fn priv_pwm0(&mut self) -> PrivPwm0W<'_, PpcPrivattribTimerSpec> {
        PrivPwm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for PWM1"]
    #[inline(always)]
    pub fn priv_pwm1(&mut self) -> PrivPwm1W<'_, PpcPrivattribTimerSpec> {
        PrivPwm1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for PWM2"]
    #[inline(always)]
    pub fn priv_pwm2(&mut self) -> PrivPwm2W<'_, PpcPrivattribTimerSpec> {
        PrivPwm2W::new(self, 2)
    }
    #[doc = "Bit 3 - Privilege access for PWM3"]
    #[inline(always)]
    pub fn priv_pwm3(&mut self) -> PrivPwm3W<'_, PpcPrivattribTimerSpec> {
        PrivPwm3W::new(self, 3)
    }
    #[doc = "Bit 4 - Privilege access for PWM4"]
    #[inline(always)]
    pub fn priv_pwm4(&mut self) -> PrivPwm4W<'_, PpcPrivattribTimerSpec> {
        PrivPwm4W::new(self, 4)
    }
    #[doc = "Bit 8 - Privilege access for ECAP0"]
    #[inline(always)]
    pub fn priv_ecap0(&mut self) -> PrivEcap0W<'_, PpcPrivattribTimerSpec> {
        PrivEcap0W::new(self, 8)
    }
    #[doc = "Bit 9 - Privilege access for ECAP1"]
    #[inline(always)]
    pub fn priv_ecap1(&mut self) -> PrivEcap1W<'_, PpcPrivattribTimerSpec> {
        PrivEcap1W::new(self, 9)
    }
    #[doc = "Bit 16 - Privilege access for EQEP0"]
    #[inline(always)]
    pub fn priv_eqep0(&mut self) -> PrivEqep0W<'_, PpcPrivattribTimerSpec> {
        PrivEqep0W::new(self, 16)
    }
    #[doc = "Bit 17 - Privilege access for EQEP1"]
    #[inline(always)]
    pub fn priv_eqep1(&mut self) -> PrivEqep1W<'_, PpcPrivattribTimerSpec> {
        PrivEqep1W::new(self, 17)
    }
    #[doc = "Bit 18 - Privilege access for EQEP2"]
    #[inline(always)]
    pub fn priv_eqep2(&mut self) -> PrivEqep2W<'_, PpcPrivattribTimerSpec> {
        PrivEqep2W::new(self, 18)
    }
    #[doc = "Bit 24 - Privilege access for TIMG4_0"]
    #[inline(always)]
    pub fn priv_timg4_0(&mut self) -> PrivTimg4_0W<'_, PpcPrivattribTimerSpec> {
        PrivTimg4_0W::new(self, 24)
    }
    #[doc = "Bit 25 - Privilege access for TIMG12_0"]
    #[inline(always)]
    pub fn priv_timg12_0(&mut self) -> PrivTimg12_0W<'_, PpcPrivattribTimerSpec> {
        PrivTimg12_0W::new(self, 25)
    }
}
#[doc = "Peripheral Privilege Access Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribTimerSpec;
impl crate::RegisterSpec for PpcPrivattribTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_timer::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_timer::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_TIMER to value 0"]
impl crate::Resettable for PpcPrivattribTimerSpec {}
