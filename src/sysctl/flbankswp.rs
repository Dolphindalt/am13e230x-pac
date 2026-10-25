#[doc = "Register `FLBANKSWP` writer"]
pub type W = crate::W<FlbankswpSpec>;
#[doc = "1: Use Upper Bank as Logical 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useupper {
    #[doc = "0: Normal (default) memory map addressing scheme"]
    Disable = 0,
    #[doc = "1: Flash upper region address space swapped with lower region"]
    Enable = 1,
}
impl From<Useupper> for bool {
    #[inline(always)]
    fn from(variant: Useupper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEUPPER` writer - 1: Use Upper Bank as Logical 0"]
pub type UseupperW<'a, REG> = crate::BitWriter<'a, REG, Useupper>;
impl<'a, REG> UseupperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal (default) memory map addressing scheme"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Useupper::Disable)
    }
    #[doc = "Flash upper region address space swapped with lower region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Useupper::Enable)
    }
}
#[doc = "The key value of 58h (88) must be written with USEUPPER to change the bank swap configuration.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "88: Issue write"]
    Unlock = 88,
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
#[doc = "Field `KEY` writer - The key value of 58h (88) must be written with USEUPPER to change the bank swap configuration."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue write"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FlbankswpSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 1: Use Upper Bank as Logical 0"]
    #[inline(always)]
    pub fn useupper(&mut self) -> UseupperW<'_, FlbankswpSpec> {
        UseupperW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value of 58h (88) must be written with USEUPPER to change the bank swap configuration."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, FlbankswpSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Flash MAIN bank address swap\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flbankswp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlbankswpSpec;
impl crate::RegisterSpec for FlbankswpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flbankswp::W`](W) writer structure"]
impl crate::Writable for FlbankswpSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FLBANKSWP to value 0"]
impl crate::Resettable for FlbankswpSpec {}
