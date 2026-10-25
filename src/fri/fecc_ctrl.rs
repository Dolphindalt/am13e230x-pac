#[doc = "Register `FECC_CTRL` reader"]
pub type R = crate::R<FeccCtrlSpec>;
#[doc = "Register `FECC_CTRL` writer"]
pub type W = crate::W<FeccCtrlSpec>;
#[doc = "Field `ECC_TEST_EN` reader - Enable ECC Test Logic"]
pub type EccTestEnR = crate::FieldReader;
#[doc = "Field `ECC_TEST_EN` writer - Enable ECC Test Logic"]
pub type EccTestEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Enable ECC Test Logic"]
    #[inline(always)]
    pub fn ecc_test_en(&self) -> EccTestEnR {
        EccTestEnR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FECC_CTRL")
            .field("ecc_test_en", &self.ecc_test_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable ECC Test Logic"]
    #[inline(always)]
    pub fn ecc_test_en(&mut self) -> EccTestEnW<'_, FeccCtrlSpec> {
        EccTestEnW::new(self, 0)
    }
}
#[doc = "ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccCtrlSpec;
impl crate::RegisterSpec for FeccCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fecc_ctrl::R`](R) reader structure"]
impl crate::Readable for FeccCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fecc_ctrl::W`](W) writer structure"]
impl crate::Writable for FeccCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECC_CTRL to value 0"]
impl crate::Resettable for FeccCtrlSpec {}
