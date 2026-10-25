#[doc = "Register `ECC_ENABLE` reader"]
pub type R = crate::R<EccEnableSpec>;
#[doc = "Register `ECC_ENABLE` writer"]
pub type W = crate::W<EccEnableSpec>;
#[doc = "Field `ENABLE` reader - Enable ECC"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `ENABLE` writer - Enable ECC"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Enable ECC"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_ENABLE")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable ECC"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, EccEnableSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "ECC Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccEnableSpec;
impl crate::RegisterSpec for EccEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_enable::R`](R) reader structure"]
impl crate::Readable for EccEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_enable::W`](W) writer structure"]
impl crate::Writable for EccEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_ENABLE to value 0x0a"]
impl crate::Resettable for EccEnableSpec {
    const RESET_VALUE: u32 = 0x0a;
}
