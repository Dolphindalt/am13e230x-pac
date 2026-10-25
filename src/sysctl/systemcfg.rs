#[doc = "Register `SYSTEMCFG` reader"]
pub type R = crate::R<SystemcfgSpec>;
#[doc = "Register `SYSTEMCFG` writer"]
pub type W = crate::W<SystemcfgSpec>;
#[doc = "Field `WWDTLP0RSTDIS` reader - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type Wwdtlp0rstdisR = crate::BitReader;
#[doc = "Field `WWDTLP0RSTDIS` writer - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type Wwdtlp0rstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHECCRSTDIS` reader - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type FlasheccrstdisR = crate::BitReader;
#[doc = "Field `FLASHECCRSTDIS` writer - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type FlasheccrstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0x1B\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "27: Issue write"]
    Unlock = 27,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x1B"]
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
impl R {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn wwdtlp0rstdis(&self) -> Wwdtlp0rstdisR {
        Wwdtlp0rstdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn flasheccrstdis(&self) -> FlasheccrstdisR {
        FlasheccrstdisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEMCFG")
            .field("flasheccrstdis", &self.flasheccrstdis())
            .field("wwdtlp0rstdis", &self.wwdtlp0rstdis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn wwdtlp0rstdis(&mut self) -> Wwdtlp0rstdisW<'_, SystemcfgSpec> {
        Wwdtlp0rstdisW::new(self, 0)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn flasheccrstdis(&mut self) -> FlasheccrstdisW<'_, SystemcfgSpec> {
        FlasheccrstdisW::new(self, 2)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x1B"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SystemcfgSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`systemcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemcfgSpec;
impl crate::RegisterSpec for SystemcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systemcfg::R`](R) reader structure"]
impl crate::Readable for SystemcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`systemcfg::W`](W) writer structure"]
impl crate::Writable for SystemcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSTEMCFG to value 0x04"]
impl crate::Resettable for SystemcfgSpec {
    const RESET_VALUE: u32 = 0x04;
}
