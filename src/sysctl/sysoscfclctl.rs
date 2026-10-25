#[doc = "Register `SYSOSCFCLCTL` writer"]
pub type W = crate::W<SysoscfclctlSpec>;
#[doc = "Field `SETUSEFCL` writer - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
pub type SetusefclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x2A\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "42: Issue Command"]
    Unlock = 42,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x2A"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue Command"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SysoscfclctlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
    #[inline(always)]
    pub fn setusefcl(&mut self) -> SetusefclW<'_, SysoscfclctlSpec> {
        SetusefclW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x2A"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SysoscfclctlSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscfclctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysoscfclctlSpec;
impl crate::RegisterSpec for SysoscfclctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysoscfclctl::W`](W) writer structure"]
impl crate::Writable for SysoscfclctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSOSCFCLCTL to value 0"]
impl crate::Resettable for SysoscfclctlSpec {}
