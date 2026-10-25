#[doc = "Register `SYSPLLPARAM1` reader"]
pub type R = crate::R<Syspllparam1Spec>;
#[doc = "Register `SYSPLLPARAM1` writer"]
pub type W = crate::W<Syspllparam1Spec>;
#[doc = "Field `LPFCAPA` reader - Loop filter Cap A"]
pub type LpfcapaR = crate::FieldReader;
#[doc = "Field `LPFCAPA` writer - Loop filter Cap A"]
pub type LpfcapaW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `LPFRESA` reader - Loop filter Res A"]
pub type LpfresaR = crate::FieldReader<u16>;
#[doc = "Field `LPFRESA` writer - Loop filter Res A"]
pub type LpfresaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `LPFRESC` reader - Loop filter Res C"]
pub type LpfrescR = crate::FieldReader;
#[doc = "Field `LPFRESC` writer - Loop filter Res C"]
pub type LpfrescW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - Loop filter Cap A"]
    #[inline(always)]
    pub fn lpfcapa(&self) -> LpfcapaR {
        LpfcapaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:17 - Loop filter Res A"]
    #[inline(always)]
    pub fn lpfresa(&self) -> LpfresaR {
        LpfresaR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - Loop filter Res C"]
    #[inline(always)]
    pub fn lpfresc(&self) -> LpfrescR {
        LpfrescR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLPARAM1")
            .field("lpfcapa", &self.lpfcapa())
            .field("lpfresc", &self.lpfresc())
            .field("lpfresa", &self.lpfresa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Loop filter Cap A"]
    #[inline(always)]
    pub fn lpfcapa(&mut self) -> LpfcapaW<'_, Syspllparam1Spec> {
        LpfcapaW::new(self, 0)
    }
    #[doc = "Bits 8:17 - Loop filter Res A"]
    #[inline(always)]
    pub fn lpfresa(&mut self) -> LpfresaW<'_, Syspllparam1Spec> {
        LpfresaW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Loop filter Res C"]
    #[inline(always)]
    pub fn lpfresc(&mut self) -> LpfrescW<'_, Syspllparam1Spec> {
        LpfrescW::new(self, 24)
    }
}
#[doc = "SYSPLL PARAM1 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam1Spec;
impl crate::RegisterSpec for Syspllparam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam1::R`](R) reader structure"]
impl crate::Readable for Syspllparam1Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllparam1::W`](W) writer structure"]
impl crate::Writable for Syspllparam1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLPARAM1 to value 0x0f00_010f"]
impl crate::Resettable for Syspllparam1Spec {
    const RESET_VALUE: u32 = 0x0f00_010f;
}
