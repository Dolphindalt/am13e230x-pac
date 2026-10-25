#[doc = "Register `SWDJCFG` writer"]
pub type W = crate::W<SwdjcfgSpec>;
#[doc = "Field `DISABLE` writer - Set DISABLE to disable the SWD/JTAG function on SWD/JTAG pins, allowing the SWD/JTAG pins to be used as GPIO."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x62\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "98: Issue command"]
    Unlock = 98,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x62"]
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
impl core::fmt::Debug for crate::generic::Reg<SwdjcfgSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the SWD/JTAG function on SWD/JTAG pins, allowing the SWD/JTAG pins to be used as GPIO."]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, SwdjcfgSpec> {
        DisableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x62"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SwdjcfgSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Disable the SWD/JTAG function on the SWD/JTAG pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdjcfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdjcfgSpec;
impl crate::RegisterSpec for SwdjcfgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdjcfg::W`](W) writer structure"]
impl crate::Writable for SwdjcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SWDJCFG to value 0"]
impl crate::Resettable for SwdjcfgSpec {}
