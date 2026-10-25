#[doc = "Register `SHDNIOREL` writer"]
pub type W = crate::W<ShdniorelSpec>;
#[doc = "Field `RELEASE` writer - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
pub type ReleaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x91\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "145: Issue command"]
    Unlock = 145,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x91"]
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
impl core::fmt::Debug for crate::generic::Reg<ShdniorelSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
    #[inline(always)]
    pub fn release(&mut self) -> ReleaseW<'_, ShdniorelSpec> {
        ReleaseW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x91"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, ShdniorelSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shdniorel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdniorelSpec;
impl crate::RegisterSpec for ShdniorelSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shdniorel::W`](W) writer structure"]
impl crate::Writable for ShdniorelSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHDNIOREL to value 0"]
impl crate::Resettable for ShdniorelSpec {}
