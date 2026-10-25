#[doc = "Register `ADCREV` reader"]
pub type R = crate::R<AdcrevSpec>;
#[doc = "Register `ADCREV` writer"]
pub type W = crate::W<AdcrevSpec>;
#[doc = "Field `TYPE` reader - ADC Type"]
pub type TypeR = crate::FieldReader;
#[doc = "Field `REV` reader - ADC Revision"]
pub type RevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - ADC Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - ADC Revision"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCREV")
            .field("type_", &self.type_())
            .field("rev", &self.rev())
            .finish()
    }
}
impl W {}
#[doc = "ADC Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcrevSpec;
impl crate::RegisterSpec for AdcrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcrev::R`](R) reader structure"]
impl crate::Readable for AdcrevSpec {}
#[doc = "`write(|w| ..)` method takes [`adcrev::W`](W) writer structure"]
impl crate::Writable for AdcrevSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCREV to value 0x06"]
impl crate::Resettable for AdcrevSpec {
    const RESET_VALUE: u32 = 0x06;
}
