#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasync {
    #[doc = "0: Not block async clock request"]
    Disable = 0,
    #[doc = "1: Block async clock request"]
    Enable = 1,
}
impl From<Blockasync> for bool {
    #[inline(always)]
    fn from(variant: Blockasync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKASYNC` reader - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncR = crate::BitReader<Blockasync>;
impl BlockasyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockasync {
        match self.bits {
            false => Blockasync::Disable,
            true => Blockasync::Enable,
        }
    }
    #[doc = "Not block async clock request"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasync::Disable
    }
    #[doc = "Block async clock request"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blockasync::Enable
    }
}
#[doc = "Field `BLOCKASYNC` writer - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncW<'a, REG> = crate::BitWriter<'a, REG, Blockasync>;
impl<'a, REG> BlockasyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not block async clock request"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Disable)
    }
    #[doc = "Block async clock request"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Enable)
    }
}
#[doc = "Write key: the write only takes effect if KEY is 0xA9\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "169: key value to allow change field of GPRCM"]
    Unlock = 169,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xA9"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "key value to allow change field of GPRCM"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&self) -> BlockasyncR {
        BlockasyncR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCFG")
            .field("blockasync", &self.blockasync())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&mut self) -> BlockasyncW<'_, ClkcfgSpec> {
        BlockasyncW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xA9"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, ClkcfgSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for ClkcfgSpec {}
