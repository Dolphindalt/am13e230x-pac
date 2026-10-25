#[doc = "Register `RESETLEVEL` reader"]
pub type R = crate::R<ResetlevelSpec>;
#[doc = "Register `RESETLEVEL` writer"]
pub type W = crate::W<ResetlevelSpec>;
#[doc = "LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    #[doc = "0: Issue a SYSRST (CPU plus peripherals only)"]
    Sys = 0,
    #[doc = "1: Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    Boot = 1,
    #[doc = "2: Issue a SYSRST and enter the boot strap loader (BSL)"]
    Bootloaderentry = 2,
    #[doc = "3: Issue a power-on reset (POR)"]
    Por = 3,
    #[doc = "4: Issue a SYSRST and exit the boot strap loader (BSL)"]
    Bootloaderexit = 4,
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Level {
    type Ux = u8;
}
impl crate::IsEnum for Level {}
#[doc = "Field `LEVEL` reader - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type LevelR = crate::FieldReader<Level>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Level> {
        match self.bits {
            0 => Some(Level::Sys),
            1 => Some(Level::Boot),
            2 => Some(Level::Bootloaderentry),
            3 => Some(Level::Por),
            4 => Some(Level::Bootloaderexit),
            _ => None,
        }
    }
    #[doc = "Issue a SYSRST (CPU plus peripherals only)"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == Level::Sys
    }
    #[doc = "Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == Level::Boot
    }
    #[doc = "Issue a SYSRST and enter the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn is_bootloaderentry(&self) -> bool {
        *self == Level::Bootloaderentry
    }
    #[doc = "Issue a power-on reset (POR)"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == Level::Por
    }
    #[doc = "Issue a SYSRST and exit the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn is_bootloaderexit(&self) -> bool {
        *self == Level::Bootloaderexit
    }
}
#[doc = "Field `LEVEL` writer - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Level, crate::Safe>;
impl<'a, REG> LevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue a SYSRST (CPU plus peripherals only)"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Sys)
    }
    #[doc = "Issue a BOOTRST (CPU, peripherals, and boot configuration routine)"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Boot)
    }
    #[doc = "Issue a SYSRST and enter the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn bootloaderentry(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Bootloaderentry)
    }
    #[doc = "Issue a power-on reset (POR)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Por)
    }
    #[doc = "Issue a SYSRST and exit the boot strap loader (BSL)"]
    #[inline(always)]
    pub fn bootloaderexit(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Bootloaderexit)
    }
}
impl R {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESETLEVEL")
            .field("level", &self.level())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, ResetlevelSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`resetlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetlevelSpec;
impl crate::RegisterSpec for ResetlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetlevel::R`](R) reader structure"]
impl crate::Readable for ResetlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`resetlevel::W`](W) writer structure"]
impl crate::Writable for ResetlevelSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RESETLEVEL to value 0"]
impl crate::Resettable for ResetlevelSpec {}
