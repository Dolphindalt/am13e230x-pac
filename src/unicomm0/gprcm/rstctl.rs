#[doc = "Register `RSTCTL` writer"]
pub type W = crate::W<RstctlSpec>;
#[doc = "Field `RESETASSERT` writer - Reset assertion to the peripheral"]
pub type ResetassertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETSTKYCLR` writer - Reset sticky bit clear"]
pub type ResetstkyclrW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for crate::generic::Reg<RstctlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Reset assertion to the peripheral"]
    #[inline(always)]
    pub fn resetassert(&mut self) -> ResetassertW<'_, RstctlSpec> {
        ResetassertW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset sticky bit clear"]
    #[inline(always)]
    pub fn resetstkyclr(&mut self) -> ResetstkyclrW<'_, RstctlSpec> {
        ResetstkyclrW::new(self, 1)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xB1"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, RstctlSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSpec;
impl crate::RegisterSpec for RstctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstctl::W`](W) writer structure"]
impl crate::Writable for RstctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RstctlSpec {}
