#[doc = "Register `PLLSTARTUP1_16_32MHZ` reader"]
pub type R = crate::R<Pllstartup1_16_32mhzSpec>;
#[doc = "Field `LPFCAPA` reader - Loop Filter Cap A"]
pub type LpfcapaR = crate::FieldReader;
#[doc = "Field `LPFRESA` reader - Loop Filter Res A"]
pub type LpfresaR = crate::FieldReader<u16>;
#[doc = "Field `LPFRESC` reader - Loop Filter Res C"]
pub type LpfrescR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Loop Filter Cap A"]
    #[inline(always)]
    pub fn lpfcapa(&self) -> LpfcapaR {
        LpfcapaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:17 - Loop Filter Res A"]
    #[inline(always)]
    pub fn lpfresa(&self) -> LpfresaR {
        LpfresaR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - Loop Filter Res C"]
    #[inline(always)]
    pub fn lpfresc(&self) -> LpfrescR {
        LpfrescR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSTARTUP1_16_32MHZ")
            .field("lpfcapa", &self.lpfcapa())
            .field("lpfresa", &self.lpfresa())
            .field("lpfresc", &self.lpfresc())
            .finish()
    }
}
#[doc = "PLLSTARTUP1_16_32MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup1_16_32mhz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllstartup1_16_32mhzSpec;
impl crate::RegisterSpec for Pllstartup1_16_32mhzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstartup1_16_32mhz::R`](R) reader structure"]
impl crate::Readable for Pllstartup1_16_32mhzSpec {}
#[doc = "`reset()` method sets PLLSTARTUP1_16_32MHZ to value 0"]
impl crate::Resettable for Pllstartup1_16_32mhzSpec {}
