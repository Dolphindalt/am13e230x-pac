#[doc = "Register `NMIISET` writer"]
pub type W = crate::W<NmiisetSpec>;
#[doc = "Field `WWDT0` writer - Watch Dog 0 Fault"]
pub type Wwdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECURITY` writer - Security Fault"]
pub type SecurityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHDED` writer - Flash Double Error Detect"]
pub type FlashdedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMPAR` writer - SRAM Parity Error Detect"]
pub type SramparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMUROMPAR` writer - TMU ROM Parity error"]
pub type TmuromparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSMEMACC` writer - SYSMEM Access error"]
pub type SysmemaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<NmiisetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<'_, NmiisetSpec> {
        Wwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - Security Fault"]
    #[inline(always)]
    pub fn security(&mut self) -> SecurityW<'_, NmiisetSpec> {
        SecurityW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&mut self) -> FlashdedW<'_, NmiisetSpec> {
        FlashdedW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Parity Error Detect"]
    #[inline(always)]
    pub fn srampar(&mut self) -> SramparW<'_, NmiisetSpec> {
        SramparW::new(self, 4)
    }
    #[doc = "Bit 5 - TMU ROM Parity error"]
    #[inline(always)]
    pub fn tmurompar(&mut self) -> TmuromparW<'_, NmiisetSpec> {
        TmuromparW::new(self, 5)
    }
    #[doc = "Bit 6 - SYSMEM Access error"]
    #[inline(always)]
    pub fn sysmemacc(&mut self) -> SysmemaccW<'_, NmiisetSpec> {
        SysmemaccW::new(self, 6)
    }
}
#[doc = "NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiisetSpec;
impl crate::RegisterSpec for NmiisetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nmiiset::W`](W) writer structure"]
impl crate::Writable for NmiisetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets NMIISET to value 0"]
impl crate::Resettable for NmiisetSpec {}
