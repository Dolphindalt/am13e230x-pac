#[doc = "Register `ADCPPB2TRIPHI` reader"]
pub type R = crate::R<Adcppb2triphiSpec>;
#[doc = "Register `ADCPPB2TRIPHI` writer"]
pub type W = crate::W<Adcppb2triphiSpec>;
#[doc = "Field `LIMITHI` reader - ADC Post Processing Block 2 Trip High Limit"]
pub type LimithiR = crate::FieldReader<u16>;
#[doc = "Field `LIMITHI` writer - ADC Post Processing Block 2 Trip High Limit"]
pub type LimithiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC Post Processing Block 2 Trip High Limit"]
    #[inline(always)]
    pub fn limithi(&self) -> LimithiR {
        LimithiR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB2TRIPHI")
            .field("limithi", &self.limithi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Post Processing Block 2 Trip High Limit"]
    #[inline(always)]
    pub fn limithi(&mut self) -> LimithiW<'_, Adcppb2triphiSpec> {
        LimithiW::new(self, 0)
    }
}
#[doc = "ADC PPB2 Trip High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2triphi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2triphi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb2triphiSpec;
impl crate::RegisterSpec for Adcppb2triphiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb2triphi::R`](R) reader structure"]
impl crate::Readable for Adcppb2triphiSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb2triphi::W`](W) writer structure"]
impl crate::Writable for Adcppb2triphiSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB2TRIPHI to value 0"]
impl crate::Resettable for Adcppb2triphiSpec {}
