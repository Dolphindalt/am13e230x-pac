#[doc = "Register `FLBANKSWPPOLICY` writer"]
pub type W = crate::W<FlbankswppolicySpec>;
#[doc = "Field `DISABLE` writer - 1: Disables Policy To Allow Flash Bank Swapping"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Must have KEY==0xCA(202) for write\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "202: Write Key"]
    Unlock = 202,
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
#[doc = "Field `KEY` writer - Must have KEY==0xCA(202) for write"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FlbankswppolicySpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 1: Disables Policy To Allow Flash Bank Swapping"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, FlbankswppolicySpec> {
        DisableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Must have KEY==0xCA(202) for write"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, FlbankswppolicySpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Flash Bank Swap Policy\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flbankswppolicy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlbankswppolicySpec;
impl crate::RegisterSpec for FlbankswppolicySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flbankswppolicy::W`](W) writer structure"]
impl crate::Writable for FlbankswppolicySpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FLBANKSWPPOLICY to value 0"]
impl crate::Resettable for FlbankswppolicySpec {}
