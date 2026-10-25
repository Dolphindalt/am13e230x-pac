#[doc = "Register `RESETCMD` writer"]
pub type W = crate::W<ResetcmdSpec>;
#[doc = "Field `GO` writer - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0xE4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "228: Issue reset"]
    Unlock = 228,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xE4"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue reset"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ResetcmdSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<'_, ResetcmdSpec> {
        GoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xE4"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, ResetcmdSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetcmdSpec;
impl crate::RegisterSpec for ResetcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`resetcmd::W`](W) writer structure"]
impl crate::Writable for ResetcmdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RESETCMD to value 0"]
impl crate::Resettable for ResetcmdSpec {}
