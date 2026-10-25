#[doc = "Register `PDBGCTL` reader"]
pub type R = crate::R<PdbgctlSpec>;
#[doc = "Register `PDBGCTL` writer"]
pub type W = crate::W<PdbgctlSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Free {
    #[doc = "0: The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    Stop = 0,
    #[doc = "1: The peripheral ignores the state of the Core Halted input"]
    Run = 1,
}
impl From<Free> for bool {
    #[inline(always)]
    fn from(variant: Free) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREE` reader - Free run control"]
pub type FreeR = crate::BitReader<Free>;
impl FreeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Free {
        match self.bits {
            false => Free::Stop,
            true => Free::Run,
        }
    }
    #[doc = "The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Free::Stop
    }
    #[doc = "The peripheral ignores the state of the Core Halted input"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Free::Run
    }
}
#[doc = "Field `FREE` writer - Free run control"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG, Free>;
impl<'a, REG> FreeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Stop)
    }
    #[doc = "The peripheral ignores the state of the Core Halted input"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Run)
    }
}
#[doc = "Soft halt boundary control. This function is only available, if \\[FREE\\] is set to 'STOP'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soft {
    #[doc = "0: Not supported"]
    Deprecated = 0,
    #[doc = "1: The peripheral blocks the debug freeze until it has reached a boundary where it can resume without corruption"]
    Delayed = 1,
}
impl From<Soft> for bool {
    #[inline(always)]
    fn from(variant: Soft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFT` reader - Soft halt boundary control. This function is only available, if \\[FREE\\] is set to 'STOP'"]
pub type SoftR = crate::BitReader<Soft>;
impl SoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soft {
        match self.bits {
            false => Soft::Deprecated,
            true => Soft::Delayed,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_deprecated(&self) -> bool {
        *self == Soft::Deprecated
    }
    #[doc = "The peripheral blocks the debug freeze until it has reached a boundary where it can resume without corruption"]
    #[inline(always)]
    pub fn is_delayed(&self) -> bool {
        *self == Soft::Delayed
    }
}
#[doc = "Field `SOFT` writer - Soft halt boundary control. This function is only available, if \\[FREE\\] is set to 'STOP'"]
pub type SoftW<'a, REG> = crate::BitWriter<'a, REG, Soft>;
impl<'a, REG> SoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn deprecated(self) -> &'a mut crate::W<REG> {
        self.variant(Soft::Deprecated)
    }
    #[doc = "The peripheral blocks the debug freeze until it has reached a boundary where it can resume without corruption"]
    #[inline(always)]
    pub fn delayed(self) -> &'a mut crate::W<REG> {
        self.variant(Soft::Delayed)
    }
}
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft halt boundary control. This function is only available, if \\[FREE\\] is set to 'STOP'"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDBGCTL")
            .field("free", &self.free())
            .field("soft", &self.soft())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&mut self) -> FreeW<'_, PdbgctlSpec> {
        FreeW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft halt boundary control. This function is only available, if \\[FREE\\] is set to 'STOP'"]
    #[inline(always)]
    pub fn soft(&mut self) -> SoftW<'_, PdbgctlSpec> {
        SoftW::new(self, 1)
    }
}
#[doc = "Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbgctlSpec;
impl crate::RegisterSpec for PdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdbgctl::R`](R) reader structure"]
impl crate::Readable for PdbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pdbgctl::W`](W) writer structure"]
impl crate::Writable for PdbgctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PDBGCTL to value 0"]
impl crate::Resettable for PdbgctlSpec {}
