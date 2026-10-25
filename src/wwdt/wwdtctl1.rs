#[doc = "Register `WWDTCTL1` reader"]
pub type R = crate::R<Wwdtctl1Spec>;
#[doc = "Register `WWDTCTL1` writer"]
pub type W = crate::W<Wwdtctl1Spec>;
#[doc = "Close Window Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winsel {
    #[doc = "0: In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    Win0 = 0,
    #[doc = "1: In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    Win1 = 1,
}
impl From<Winsel> for bool {
    #[inline(always)]
    fn from(variant: Winsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINSEL` reader - Close Window Select"]
pub type WinselR = crate::BitReader<Winsel>;
impl WinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winsel {
        match self.bits {
            false => Winsel::Win0,
            true => Winsel::Win1,
        }
    }
    #[doc = "In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn is_win0(&self) -> bool {
        *self == Winsel::Win0
    }
    #[doc = "In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn is_win1(&self) -> bool {
        *self == Winsel::Win1
    }
}
#[doc = "Field `WINSEL` writer - Close Window Select"]
pub type WinselW<'a, REG> = crate::BitWriter<'a, REG, Winsel>;
impl<'a, REG> WinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn win0(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Win0)
    }
    #[doc = "In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn win1(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Win1)
    }
}
#[doc = "Write key: the write only takes effect if KEY is 0xBE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "190: KEY to allow write access to this register"]
    Unlock = 190,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xBE"]
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
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WinselR {
        WinselR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDTCTL1")
            .field("winsel", &self.winsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn winsel(&mut self) -> WinselW<'_, Wwdtctl1Spec> {
        WinselW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xBE"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Wwdtctl1Spec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdtctl1Spec;
impl crate::RegisterSpec for Wwdtctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtctl1::R`](R) reader structure"]
impl crate::Readable for Wwdtctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`wwdtctl1::W`](W) writer structure"]
impl crate::Writable for Wwdtctl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WWDTCTL1 to value 0"]
impl crate::Resettable for Wwdtctl1Spec {}
