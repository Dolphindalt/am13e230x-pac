#[doc = "Register `PMODECFG` reader"]
pub type R = crate::R<PmodecfgSpec>;
#[doc = "Register `PMODECFG` writer"]
pub type W = crate::W<PmodecfgSpec>;
#[doc = "DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsleep {
    #[doc = "0: STOP mode is entered"]
    Stop = 0,
    #[doc = "1: STANDBY mode is entered"]
    Standby = 1,
    #[doc = "2: SHUTDOWN mode is entered"]
    Shutdown = 2,
}
impl From<Dsleep> for u8 {
    #[inline(always)]
    fn from(variant: Dsleep) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsleep {
    type Ux = u8;
}
impl crate::IsEnum for Dsleep {}
#[doc = "Field `DSLEEP` reader - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
pub type DsleepR = crate::FieldReader<Dsleep>;
impl DsleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsleep> {
        match self.bits {
            0 => Some(Dsleep::Stop),
            1 => Some(Dsleep::Standby),
            2 => Some(Dsleep::Shutdown),
            _ => None,
        }
    }
    #[doc = "STOP mode is entered"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Dsleep::Stop
    }
    #[doc = "STANDBY mode is entered"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Dsleep::Standby
    }
    #[doc = "SHUTDOWN mode is entered"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == Dsleep::Shutdown
    }
}
#[doc = "Field `DSLEEP` writer - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
pub type DsleepW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsleep, crate::Safe>;
impl<'a, REG> DsleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STOP mode is entered"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Dsleep::Stop)
    }
    #[doc = "STANDBY mode is entered"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Dsleep::Standby)
    }
    #[doc = "SHUTDOWN mode is entered"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(Dsleep::Shutdown)
    }
}
impl R {
    #[doc = "Bits 0:1 - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
    #[inline(always)]
    pub fn dsleep(&self) -> DsleepR {
        DsleepR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMODECFG")
            .field("dsleep", &self.dsleep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
    #[inline(always)]
    pub fn dsleep(&mut self) -> DsleepW<'_, PmodecfgSpec> {
        DsleepW::new(self, 0)
    }
}
#[doc = "Power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pmodecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmodecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmodecfgSpec;
impl crate::RegisterSpec for PmodecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmodecfg::R`](R) reader structure"]
impl crate::Readable for PmodecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmodecfg::W`](W) writer structure"]
impl crate::Writable for PmodecfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PMODECFG to value 0"]
impl crate::Resettable for PmodecfgSpec {}
