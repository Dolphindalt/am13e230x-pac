#[doc = "Register `SYSPLLPARAM2` reader"]
pub type R = crate::R<Syspllparam2Spec>;
#[doc = "Register `SYSPLLPARAM2` writer"]
pub type W = crate::W<Syspllparam2Spec>;
#[doc = "Field `LPFCAPC` reader - Loop filter Cap C"]
pub type LpfcapcR = crate::FieldReader;
#[doc = "Field `LPFCAPC` writer - Loop filter Cap C"]
pub type LpfcapcW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `RNGFIXVCOIBIASCFG` reader - 0 value for Temperature Compensation R addition"]
pub type RngfixvcoibiascfgR = crate::BitReader;
#[doc = "Field `RNGFIXVCOIBIASCFG` writer - 0 value for Temperature Compensation R addition"]
pub type RngfixvcoibiascfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Loop filter Cap C"]
    #[inline(always)]
    pub fn lpfcapc(&self) -> LpfcapcR {
        LpfcapcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - 0 value for Temperature Compensation R addition"]
    #[inline(always)]
    pub fn rngfixvcoibiascfg(&self) -> RngfixvcoibiascfgR {
        RngfixvcoibiascfgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLPARAM2")
            .field("rngfixvcoibiascfg", &self.rngfixvcoibiascfg())
            .field("lpfcapc", &self.lpfcapc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Loop filter Cap C"]
    #[inline(always)]
    pub fn lpfcapc(&mut self) -> LpfcapcW<'_, Syspllparam2Spec> {
        LpfcapcW::new(self, 0)
    }
    #[doc = "Bit 3 - 0 value for Temperature Compensation R addition"]
    #[inline(always)]
    pub fn rngfixvcoibiascfg(&mut self) -> RngfixvcoibiascfgW<'_, Syspllparam2Spec> {
        RngfixvcoibiascfgW::new(self, 3)
    }
}
#[doc = "SYSPLL PARAM2 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam2Spec;
impl crate::RegisterSpec for Syspllparam2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam2::R`](R) reader structure"]
impl crate::Readable for Syspllparam2Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllparam2::W`](W) writer structure"]
impl crate::Writable for Syspllparam2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLPARAM2 to value 0x08"]
impl crate::Resettable for Syspllparam2Spec {
    const RESET_VALUE: u32 = 0x08;
}
