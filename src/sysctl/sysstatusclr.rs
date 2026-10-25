#[doc = "Register `SYSSTATUSCLR` writer"]
pub type W = crate::W<SysstatusclrSpec>;
#[doc = "Field `ALLECC` writer - Set ALLECC to clear all ECC related SYSSTATUS indicators."]
pub type AlleccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0xCE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "206: Issue command"]
    Unlock = 206,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xCE"]
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
impl core::fmt::Debug for crate::generic::Reg<SysstatusclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set ALLECC to clear all ECC related SYSSTATUS indicators."]
    #[inline(always)]
    pub fn allecc(&mut self) -> AlleccW<'_, SysstatusclrSpec> {
        AlleccW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xCE"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SysstatusclrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Clear sticky bits of SYSSTATUS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysstatusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysstatusclrSpec;
impl crate::RegisterSpec for SysstatusclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysstatusclr::W`](W) writer structure"]
impl crate::Writable for SysstatusclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSSTATUSCLR to value 0"]
impl crate::Resettable for SysstatusclrSpec {}
