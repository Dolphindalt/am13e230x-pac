#[doc = "Register `OUTPUTXBARLOCK` reader"]
pub type R = crate::R<OutputxbarlockSpec>;
#[doc = "Register `OUTPUTXBARLOCK` writer"]
pub type W = crate::W<OutputxbarlockSpec>;
#[doc = "Field `LOCK` reader - Locks the configuration for EOUTPUT-XBAR"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Locks the configuration for EOUTPUT-XBAR"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x5A5A\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "23130: Bit 0 of this register can be set only if KEY = 0x5A5A"]
    Unlock = 23130,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x5A5A"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Bit 0 of this register can be set only if KEY = 0x5A5A"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bit 0 - Locks the configuration for EOUTPUT-XBAR"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARLOCK")
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Locks the configuration for EOUTPUT-XBAR"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, OutputxbarlockSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write key: the write only takes effect if KEY is 0x5A5A"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, OutputxbarlockSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Configuration Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarlockSpec;
impl crate::RegisterSpec for OutputxbarlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarlock::R`](R) reader structure"]
impl crate::Readable for OutputxbarlockSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarlock::W`](W) writer structure"]
impl crate::Writable for OutputxbarlockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARLOCK to value 0"]
impl crate::Resettable for OutputxbarlockSpec {}
