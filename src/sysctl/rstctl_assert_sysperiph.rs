#[doc = "Register `RSTCTL_ASSERT_SYSPERIPH` writer"]
pub type W = crate::W<RstctlAssertSysperiphSpec>;
#[doc = "Field `tinie` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type TinieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pga0` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pga0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pga1` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pga1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pga2` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type Pga2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `epi` writer - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
pub type EpiW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for crate::generic::Reg<RstctlAssertSysperiphSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn tinie(&mut self) -> TinieW<'_, RstctlAssertSysperiphSpec> {
        TinieW::new(self, 0)
    }
    #[doc = "Bit 1 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pga0(&mut self) -> Pga0W<'_, RstctlAssertSysperiphSpec> {
        Pga0W::new(self, 1)
    }
    #[doc = "Bit 2 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pga1(&mut self) -> Pga1W<'_, RstctlAssertSysperiphSpec> {
        Pga1W::new(self, 2)
    }
    #[doc = "Bit 3 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn pga2(&mut self) -> Pga2W<'_, RstctlAssertSysperiphSpec> {
        Pga2W::new(self, 3)
    }
    #[doc = "Bit 4 - assert reset to the peripheral, KEY must be set to B1h to write to this bit"]
    #[inline(always)]
    pub fn epi(&mut self) -> EpiW<'_, RstctlAssertSysperiphSpec> {
        EpiW::new(self, 4)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xB1"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, RstctlAssertSysperiphSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "rstcl assert Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_assert_sysperiph::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlAssertSysperiphSpec;
impl crate::RegisterSpec for RstctlAssertSysperiphSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstctl_assert_sysperiph::W`](W) writer structure"]
impl crate::Writable for RstctlAssertSysperiphSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RSTCTL_ASSERT_SYSPERIPH to value 0"]
impl crate::Resettable for RstctlAssertSysperiphSpec {}
