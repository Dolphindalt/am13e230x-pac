#[doc = "Register `HSCLKEN` reader"]
pub type R = crate::R<HsclkenSpec>;
#[doc = "Register `HSCLKEN` writer"]
pub type W = crate::W<HsclkenSpec>;
#[doc = "SYSPLLEN enables or disables the system phase-lock loop (SYSPLL).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllen {
    #[doc = "0: Disable the SYSPLL"]
    Disable = 0,
    #[doc = "1: Enable the SYSPLL"]
    Enable = 1,
}
impl From<Syspllen> for bool {
    #[inline(always)]
    fn from(variant: Syspllen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLEN` reader - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
pub type SyspllenR = crate::BitReader<Syspllen>;
impl SyspllenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllen {
        match self.bits {
            false => Syspllen::Disable,
            true => Syspllen::Enable,
        }
    }
    #[doc = "Disable the SYSPLL"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Syspllen::Disable
    }
    #[doc = "Enable the SYSPLL"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Syspllen::Enable
    }
}
#[doc = "Field `SYSPLLEN` writer - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
pub type SyspllenW<'a, REG> = crate::BitWriter<'a, REG, Syspllen>;
impl<'a, REG> SyspllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the SYSPLL"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllen::Disable)
    }
    #[doc = "Enable the SYSPLL"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllen::Enable)
    }
}
#[doc = "USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useexthfclk {
    #[doc = "0: Use XTAL as the HFCLK source"]
    Disable = 0,
    #[doc = "1: Use the HFCLK_IN digital clock input as the HFCLK source"]
    Enable = 1,
}
impl From<Useexthfclk> for bool {
    #[inline(always)]
    fn from(variant: Useexthfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEEXTHFCLK` reader - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK."]
pub type UseexthfclkR = crate::BitReader<Useexthfclk>;
impl UseexthfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Useexthfclk {
        match self.bits {
            false => Useexthfclk::Disable,
            true => Useexthfclk::Enable,
        }
    }
    #[doc = "Use XTAL as the HFCLK source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Useexthfclk::Disable
    }
    #[doc = "Use the HFCLK_IN digital clock input as the HFCLK source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Useexthfclk::Enable
    }
}
#[doc = "Field `USEEXTHFCLK` writer - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK."]
pub type UseexthfclkW<'a, REG> = crate::BitWriter<'a, REG, Useexthfclk>;
impl<'a, REG> UseexthfclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use XTAL as the HFCLK source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Useexthfclk::Disable)
    }
    #[doc = "Use the HFCLK_IN digital clock input as the HFCLK source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Useexthfclk::Enable)
    }
}
impl R {
    #[doc = "Bit 8 - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
    #[inline(always)]
    pub fn syspllen(&self) -> SyspllenR {
        SyspllenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK."]
    #[inline(always)]
    pub fn useexthfclk(&self) -> UseexthfclkR {
        UseexthfclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCLKEN")
            .field("useexthfclk", &self.useexthfclk())
            .field("syspllen", &self.syspllen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
    #[inline(always)]
    pub fn syspllen(&mut self) -> SyspllenW<'_, HsclkenSpec> {
        SyspllenW::new(self, 8)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK."]
    #[inline(always)]
    pub fn useexthfclk(&mut self) -> UseexthfclkW<'_, HsclkenSpec> {
        UseexthfclkW::new(self, 16)
    }
}
#[doc = "High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsclkenSpec;
impl crate::RegisterSpec for HsclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsclken::R`](R) reader structure"]
impl crate::Readable for HsclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`hsclken::W`](W) writer structure"]
impl crate::Writable for HsclkenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HSCLKEN to value 0"]
impl crate::Resettable for HsclkenSpec {}
