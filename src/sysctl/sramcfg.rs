#[doc = "Register `SRAMCFG` reader"]
pub type R = crate::R<SramcfgSpec>;
#[doc = "Register `SRAMCFG` writer"]
pub type W = crate::W<SramcfgSpec>;
#[doc = "Field `BANKINITDIS0` reader - SRAM BANK0 Initialization"]
pub type Bankinitdis0R = crate::BitReader;
#[doc = "Field `BANKINITDIS0` writer - SRAM BANK0 Initialization"]
pub type Bankinitdis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKINITDIS1` reader - SRAM BANK1 Initialization"]
pub type Bankinitdis1R = crate::BitReader;
#[doc = "Field `BANKINITDIS1` writer - SRAM BANK1 Initialization"]
pub type Bankinitdis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKINITDIS2` reader - SRAM BANK2 Initialization"]
pub type Bankinitdis2R = crate::BitReader;
#[doc = "Field `BANKINITDIS2` writer - SRAM BANK2 Initialization"]
pub type Bankinitdis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKINITDIS3` reader - SRAM BANK3 Initialization"]
pub type Bankinitdis3R = crate::BitReader;
#[doc = "Field `BANKINITDIS3` writer - SRAM BANK3 Initialization"]
pub type Bankinitdis3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0xB5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "181: Issue write"]
    Unlock = 181,
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
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xB5"]
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
    #[doc = "Bit 16 - SRAM BANK0 Initialization"]
    #[inline(always)]
    pub fn bankinitdis0(&self) -> Bankinitdis0R {
        Bankinitdis0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM BANK1 Initialization"]
    #[inline(always)]
    pub fn bankinitdis1(&self) -> Bankinitdis1R {
        Bankinitdis1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SRAM BANK2 Initialization"]
    #[inline(always)]
    pub fn bankinitdis2(&self) -> Bankinitdis2R {
        Bankinitdis2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM BANK3 Initialization"]
    #[inline(always)]
    pub fn bankinitdis3(&self) -> Bankinitdis3R {
        Bankinitdis3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMCFG")
            .field("bankinitdis3", &self.bankinitdis3())
            .field("bankinitdis2", &self.bankinitdis2())
            .field("bankinitdis0", &self.bankinitdis0())
            .field("bankinitdis1", &self.bankinitdis1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - SRAM BANK0 Initialization"]
    #[inline(always)]
    pub fn bankinitdis0(&mut self) -> Bankinitdis0W<'_, SramcfgSpec> {
        Bankinitdis0W::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM BANK1 Initialization"]
    #[inline(always)]
    pub fn bankinitdis1(&mut self) -> Bankinitdis1W<'_, SramcfgSpec> {
        Bankinitdis1W::new(self, 17)
    }
    #[doc = "Bit 18 - SRAM BANK2 Initialization"]
    #[inline(always)]
    pub fn bankinitdis2(&mut self) -> Bankinitdis2W<'_, SramcfgSpec> {
        Bankinitdis2W::new(self, 18)
    }
    #[doc = "Bit 19 - SRAM BANK3 Initialization"]
    #[inline(always)]
    pub fn bankinitdis3(&mut self) -> Bankinitdis3W<'_, SramcfgSpec> {
        Bankinitdis3W::new(self, 19)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xB5"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SramcfgSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "System SRAM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sramcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramcfgSpec;
impl crate::RegisterSpec for SramcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramcfg::R`](R) reader structure"]
impl crate::Readable for SramcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sramcfg::W`](W) writer structure"]
impl crate::Writable for SramcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SRAMCFG to value 0"]
impl crate::Resettable for SramcfgSpec {}
