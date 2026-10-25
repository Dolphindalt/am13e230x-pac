#[doc = "Register `ECAPSYNCINSEL` reader"]
pub type R = crate::R<EcapsyncinselSpec>;
#[doc = "Register `ECAPSYNCINSEL` writer"]
pub type W = crate::W<EcapsyncinselSpec>;
#[doc = "Field `SEL` reader - SYNCIN source select"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - SYNCIN source select"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - SYNCIN source select"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECAPSYNCINSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - SYNCIN source select"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, EcapsyncinselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "SYNC source select register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecapsyncinsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecapsyncinsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcapsyncinselSpec;
impl crate::RegisterSpec for EcapsyncinselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecapsyncinsel::R`](R) reader structure"]
impl crate::Readable for EcapsyncinselSpec {}
#[doc = "`write(|w| ..)` method takes [`ecapsyncinsel::W`](W) writer structure"]
impl crate::Writable for EcapsyncinselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECAPSYNCINSEL to value 0x01"]
impl crate::Resettable for EcapsyncinselSpec {
    const RESET_VALUE: u32 = 0x01;
}
