#[doc = "Register `EXRSTPIN` writer"]
pub type W = crate::W<ExrstpinSpec>;
#[doc = "Field `DISABLE` writer - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x1E\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "30: Issue command"]
    Unlock = 30,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x1E"]
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
impl core::fmt::Debug for crate::generic::Reg<ExrstpinSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, ExrstpinSpec> {
        DisableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x1E"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, ExrstpinSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrstpin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExrstpinSpec;
impl crate::RegisterSpec for ExrstpinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`exrstpin::W`](W) writer structure"]
impl crate::Writable for ExrstpinSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXRSTPIN to value 0"]
impl crate::Resettable for ExrstpinSpec {}
