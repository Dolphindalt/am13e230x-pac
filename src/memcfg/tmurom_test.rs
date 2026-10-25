#[doc = "Register `TMUROM_TEST` reader"]
pub type R = crate::R<TmuromTestSpec>;
#[doc = "Register `TMUROM_TEST` writer"]
pub type W = crate::W<TmuromTestSpec>;
#[doc = "Field `TMUROM_PAR_FORCE` reader - ROM wait state 1 enable"]
pub type TmuromParForceR = crate::FieldReader;
#[doc = "Field `TMUROM_PAR_FORCE` writer - ROM wait state 1 enable"]
pub type TmuromParForceW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - ROM wait state 1 enable"]
    #[inline(always)]
    pub fn tmurom_par_force(&self) -> TmuromParForceR {
        TmuromParForceR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMUROM_TEST")
            .field("tmurom_par_force", &self.tmurom_par_force())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ROM wait state 1 enable"]
    #[inline(always)]
    pub fn tmurom_par_force(&mut self) -> TmuromParForceW<'_, TmuromTestSpec> {
        TmuromParForceW::new(self, 0)
    }
}
#[doc = "ROM wait state configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmuromTestSpec;
impl crate::RegisterSpec for TmuromTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmurom_test::R`](R) reader structure"]
impl crate::Readable for TmuromTestSpec {}
#[doc = "`write(|w| ..)` method takes [`tmurom_test::W`](W) writer structure"]
impl crate::Writable for TmuromTestSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TMUROM_TEST to value 0x05"]
impl crate::Resettable for TmuromTestSpec {
    const RESET_VALUE: u32 = 0x05;
}
