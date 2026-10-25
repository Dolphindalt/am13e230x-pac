#[doc = "Register `ADCPPB1CONFIG` reader"]
pub type R = crate::R<Adcppb1configSpec>;
#[doc = "Register `ADCPPB1CONFIG` writer"]
pub type W = crate::W<Adcppb1configSpec>;
#[doc = "Field `CBCEN` reader - Cycle By Cycle Enable"]
pub type CbcenR = crate::BitReader;
#[doc = "Field `CBCEN` writer - Cycle By Cycle Enable"]
pub type CbcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Cycle By Cycle Enable"]
    #[inline(always)]
    pub fn cbcen(&self) -> CbcenR {
        CbcenR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB1CONFIG")
            .field("cbcen", &self.cbcen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Cycle By Cycle Enable"]
    #[inline(always)]
    pub fn cbcen(&mut self) -> CbcenW<'_, Adcppb1configSpec> {
        CbcenW::new(self, 5)
    }
}
#[doc = "ADC PPB1 Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb1configSpec;
impl crate::RegisterSpec for Adcppb1configSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb1config::R`](R) reader structure"]
impl crate::Readable for Adcppb1configSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb1config::W`](W) writer structure"]
impl crate::Writable for Adcppb1configSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB1CONFIG to value 0"]
impl crate::Resettable for Adcppb1configSpec {}
