#[doc = "Register `RSTCTL_ASSERT_MCPERIPH` writer"]
pub type W = crate::W<RstctlAssertMcperiphSpec>;
#[doc = "Field `eqep0` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Eqep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eqep1` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Eqep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eqep2` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Eqep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ecap0` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Ecap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ecap1` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Ecap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm0` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pwm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm1` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pwm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm2` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pwm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm3` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pwm3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm4` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pwm4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpss0` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Cmpss0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpss1` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Cmpss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpss2` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Cmpss2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpss3` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Cmpss3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xbar` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type XbarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0xB1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "177: KEY to allow write access to this register"]
    Unlock = 177,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xB1"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "KEY to allow write access to this register"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RstctlAssertMcperiphSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn eqep0(&mut self) -> Eqep0W<'_, RstctlAssertMcperiphSpec> {
        Eqep0W::new(self, 0)
    }
    #[doc = "Bit 1 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn eqep1(&mut self) -> Eqep1W<'_, RstctlAssertMcperiphSpec> {
        Eqep1W::new(self, 1)
    }
    #[doc = "Bit 2 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn eqep2(&mut self) -> Eqep2W<'_, RstctlAssertMcperiphSpec> {
        Eqep2W::new(self, 2)
    }
    #[doc = "Bit 3 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn ecap0(&mut self) -> Ecap0W<'_, RstctlAssertMcperiphSpec> {
        Ecap0W::new(self, 3)
    }
    #[doc = "Bit 4 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn ecap1(&mut self) -> Ecap1W<'_, RstctlAssertMcperiphSpec> {
        Ecap1W::new(self, 4)
    }
    #[doc = "Bit 5 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pwm0(&mut self) -> Pwm0W<'_, RstctlAssertMcperiphSpec> {
        Pwm0W::new(self, 5)
    }
    #[doc = "Bit 6 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pwm1(&mut self) -> Pwm1W<'_, RstctlAssertMcperiphSpec> {
        Pwm1W::new(self, 6)
    }
    #[doc = "Bit 7 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pwm2(&mut self) -> Pwm2W<'_, RstctlAssertMcperiphSpec> {
        Pwm2W::new(self, 7)
    }
    #[doc = "Bit 8 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pwm3(&mut self) -> Pwm3W<'_, RstctlAssertMcperiphSpec> {
        Pwm3W::new(self, 8)
    }
    #[doc = "Bit 9 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pwm4(&mut self) -> Pwm4W<'_, RstctlAssertMcperiphSpec> {
        Pwm4W::new(self, 9)
    }
    #[doc = "Bit 10 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn cmpss0(&mut self) -> Cmpss0W<'_, RstctlAssertMcperiphSpec> {
        Cmpss0W::new(self, 10)
    }
    #[doc = "Bit 11 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn cmpss1(&mut self) -> Cmpss1W<'_, RstctlAssertMcperiphSpec> {
        Cmpss1W::new(self, 11)
    }
    #[doc = "Bit 12 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn cmpss2(&mut self) -> Cmpss2W<'_, RstctlAssertMcperiphSpec> {
        Cmpss2W::new(self, 12)
    }
    #[doc = "Bit 13 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn cmpss3(&mut self) -> Cmpss3W<'_, RstctlAssertMcperiphSpec> {
        Cmpss3W::new(self, 13)
    }
    #[doc = "Bit 14 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn xbar(&mut self) -> XbarW<'_, RstctlAssertMcperiphSpec> {
        XbarW::new(self, 14)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xB1"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, RstctlAssertMcperiphSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "rstctl assert register to control reset assertion - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_assert_mcperiph::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlAssertMcperiphSpec;
impl crate::RegisterSpec for RstctlAssertMcperiphSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstctl_assert_mcperiph::W`](W) writer structure"]
impl crate::Writable for RstctlAssertMcperiphSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RSTCTL_ASSERT_MCPERIPH to value 0"]
impl crate::Resettable for RstctlAssertMcperiphSpec {}
