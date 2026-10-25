#[doc = "Register `SYSOSCCFG` reader"]
pub type R = crate::R<SysosccfgSpec>;
#[doc = "Register `SYSOSCCFG` writer"]
pub type W = crate::W<SysosccfgSpec>;
#[doc = "Field `DISABLE` reader - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
pub type DisableR = crate::BitReader;
#[doc = "BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasyncall {
    #[doc = "0: Asynchronous fast clock requests are controlled by the requesting peripheral"]
    Disable = 0,
    #[doc = "1: All asynchronous fast clock requests are blocked"]
    Enable = 1,
}
impl From<Blockasyncall> for bool {
    #[inline(always)]
    fn from(variant: Blockasyncall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKASYNCALL` reader - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type BlockasyncallR = crate::BitReader<Blockasyncall>;
impl BlockasyncallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockasyncall {
        match self.bits {
            false => Blockasyncall::Disable,
            true => Blockasyncall::Enable,
        }
    }
    #[doc = "Asynchronous fast clock requests are controlled by the requesting peripheral"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasyncall::Disable
    }
    #[doc = "All asynchronous fast clock requests are blocked"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blockasyncall::Enable
    }
}
#[doc = "Field `BLOCKASYNCALL` writer - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type BlockasyncallW<'a, REG> = crate::BitWriter<'a, REG, Blockasyncall>;
impl<'a, REG> BlockasyncallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous fast clock requests are controlled by the requesting peripheral"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasyncall::Disable)
    }
    #[doc = "All asynchronous fast clock requests are blocked"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasyncall::Enable)
    }
}
#[doc = "if disabled CPU will not wakeup and continue in STANDBY\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastcpuevent {
    #[doc = "0: An interrupt to the CPU will not assert a fast clock request"]
    Disable = 0,
    #[doc = "1: An interrupt to the CPU will assert a fast clock request"]
    Enable = 1,
}
impl From<Fastcpuevent> for bool {
    #[inline(always)]
    fn from(variant: Fastcpuevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTCPUEVENT` reader - if disabled CPU will not wakeup and continue in STANDBY"]
pub type FastcpueventR = crate::BitReader<Fastcpuevent>;
impl FastcpueventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastcpuevent {
        match self.bits {
            false => Fastcpuevent::Disable,
            true => Fastcpuevent::Enable,
        }
    }
    #[doc = "An interrupt to the CPU will not assert a fast clock request"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fastcpuevent::Disable
    }
    #[doc = "An interrupt to the CPU will assert a fast clock request"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fastcpuevent::Enable
    }
}
#[doc = "Field `FASTCPUEVENT` writer - if disabled CPU will not wakeup and continue in STANDBY"]
pub type FastcpueventW<'a, REG> = crate::BitWriter<'a, REG, Fastcpuevent>;
impl<'a, REG> FastcpueventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An interrupt to the CPU will not assert a fast clock request"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastcpuevent::Disable)
    }
    #[doc = "An interrupt to the CPU will assert a fast clock request"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastcpuevent::Enable)
    }
}
impl R {
    #[doc = "Bit 10 - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn blockasyncall(&self) -> BlockasyncallR {
        BlockasyncallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - if disabled CPU will not wakeup and continue in STANDBY"]
    #[inline(always)]
    pub fn fastcpuevent(&self) -> FastcpueventR {
        FastcpueventR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSOSCCFG")
            .field("blockasyncall", &self.blockasyncall())
            .field("disable", &self.disable())
            .field("fastcpuevent", &self.fastcpuevent())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn blockasyncall(&mut self) -> BlockasyncallW<'_, SysosccfgSpec> {
        BlockasyncallW::new(self, 16)
    }
    #[doc = "Bit 17 - if disabled CPU will not wakeup and continue in STANDBY"]
    #[inline(always)]
    pub fn fastcpuevent(&mut self) -> FastcpueventW<'_, SysosccfgSpec> {
        FastcpueventW::new(self, 17)
    }
}
#[doc = "SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysosccfgSpec;
impl crate::RegisterSpec for SysosccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysosccfg::R`](R) reader structure"]
impl crate::Readable for SysosccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysosccfg::W`](W) writer structure"]
impl crate::Writable for SysosccfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSOSCCFG to value 0x0002_0100"]
impl crate::Resettable for SysosccfgSpec {
    const RESET_VALUE: u32 = 0x0002_0100;
}
