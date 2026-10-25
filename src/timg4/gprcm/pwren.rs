#[doc = "Register `PWREN` reader"]
pub type R = crate::R<PwrenSpec>;
#[doc = "Register `PWREN` writer"]
pub type W = crate::W<PwrenSpec>;
#[doc = "Field `ENABLE` reader - Power Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Power Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "38: KEY to allow write access to this register"]
    Unlock = 38,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x26"]
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
impl R {
    #[doc = "Bit 0 - Power Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWREN")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Power Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, PwrenSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x26"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, PwrenSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenSpec;
impl crate::RegisterSpec for PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren::R`](R) reader structure"]
impl crate::Readable for PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren::W`](W) writer structure"]
impl crate::Writable for PwrenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWREN to value 0"]
impl crate::Resettable for PwrenSpec {}
