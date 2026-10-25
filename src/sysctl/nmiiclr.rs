#[doc = "Register `NMIICLR` writer"]
pub type W = crate::W<NmiiclrSpec>;
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
impl core::fmt::Debug for crate::generic::Reg<NmiiclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<'_, NmiiclrSpec> {
        Wwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - Security Fault"]
    #[inline(always)]
    pub fn security(&mut self) -> SecurityW<'_, NmiiclrSpec> {
        SecurityW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&mut self) -> FlashdedW<'_, NmiiclrSpec> {
        FlashdedW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Parity Error Detect"]
    #[inline(always)]
    pub fn srampar(&mut self) -> SramparW<'_, NmiiclrSpec> {
        SramparW::new(self, 4)
    }
    #[doc = "Bit 5 - TMU ROM Parity error"]
    #[inline(always)]
    pub fn tmurompar(&mut self) -> TmuromparW<'_, NmiiclrSpec> {
        TmuromparW::new(self, 5)
    }
    #[doc = "Bit 6 - SYSMEM Access error"]
    #[inline(always)]
    pub fn sysmemacc(&mut self) -> SysmemaccW<'_, NmiiclrSpec> {
        SysmemaccW::new(self, 6)
    }
}
#[doc = "NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiiclrSpec;
impl crate::RegisterSpec for NmiiclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nmiiclr::W`](W) writer structure"]
impl crate::Writable for NmiiclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets NMIICLR to value 0"]
impl crate::Resettable for NmiiclrSpec {}
