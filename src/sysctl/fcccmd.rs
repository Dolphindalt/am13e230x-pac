#[doc = "Register `FCCCMD` writer"]
pub type W = crate::W<FcccmdSpec>;
#[doc = "Field `GO` writer - Set GO to start a capture with the frequency clock counter (FCC)."]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x0E\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "14: Issue command"]
    Unlock = 14,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x0E"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue command"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FcccmdSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set GO to start a capture with the frequency clock counter (FCC)."]
    #[inline(always)]
    pub fn go(&mut self) -> GoW<'_, FcccmdSpec> {
        GoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x0E"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, FcccmdSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcccmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcccmdSpec;
impl crate::RegisterSpec for FcccmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcccmd::W`](W) writer structure"]
impl crate::Writable for FcccmdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FCCCMD to value 0"]
impl crate::Resettable for FcccmdSpec {}
