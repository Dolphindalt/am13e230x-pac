#[doc = "Register `INITDONE` writer"]
pub type W = crate::W<InitdoneSpec>;
#[doc = "Field `PASS` writer - INITCODE writes 1 for PASS, left unwritten a timeout will occur if not blocked"]
pub type PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Must have KEY==0x9D(157) for write\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "157: Issue Reset"]
    Unlock = 157,
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
#[doc = "Field `KEY` writer - Must have KEY==0x9D(157) for write"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue Reset"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<InitdoneSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - INITCODE writes 1 for PASS, left unwritten a timeout will occur if not blocked"]
    #[inline(always)]
    pub fn pass(&mut self) -> PassW<'_, InitdoneSpec> {
        PassW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Must have KEY==0x9D(157) for write"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, InitdoneSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "INITCODE PASS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initdone::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitdoneSpec;
impl crate::RegisterSpec for InitdoneSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`initdone::W`](W) writer structure"]
impl crate::Writable for InitdoneSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INITDONE to value 0"]
impl crate::Resettable for InitdoneSpec {}
