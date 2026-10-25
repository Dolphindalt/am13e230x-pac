#[doc = "Register `ADCCTL1` reader"]
pub type R = crate::R<Adcctl1Spec>;
#[doc = "Register `ADCCTL1` writer"]
pub type W = crate::W<Adcctl1Spec>;
#[doc = "Field `INTPULSEPOS` reader - ADC Interrupt Pulse Position."]
pub type IntpulseposR = crate::BitReader;
#[doc = "Field `INTPULSEPOS` writer - ADC Interrupt Pulse Position."]
pub type IntpulseposW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPWDNZ` reader - ADC Power Down"]
pub type AdcpwdnzR = crate::BitReader;
#[doc = "Field `ADCPWDNZ` writer - ADC Power Down"]
pub type AdcpwdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBSYCHN` reader - ADC Busy Channel"]
pub type AdcbsychnR = crate::FieldReader;
#[doc = "Field `ADCBSY` reader - ADC Busy"]
pub type AdcbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - ADC Interrupt Pulse Position."]
    #[inline(always)]
    pub fn intpulsepos(&self) -> IntpulseposR {
        IntpulseposR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Power Down"]
    #[inline(always)]
    pub fn adcpwdnz(&self) -> AdcpwdnzR {
        AdcpwdnzR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC Busy Channel"]
    #[inline(always)]
    pub fn adcbsychn(&self) -> AdcbsychnR {
        AdcbsychnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - ADC Busy"]
    #[inline(always)]
    pub fn adcbsy(&self) -> AdcbsyR {
        AdcbsyR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCTL1")
            .field("intpulsepos", &self.intpulsepos())
            .field("adcpwdnz", &self.adcpwdnz())
            .field("adcbsychn", &self.adcbsychn())
            .field("adcbsy", &self.adcbsy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - ADC Interrupt Pulse Position."]
    #[inline(always)]
    pub fn intpulsepos(&mut self) -> IntpulseposW<'_, Adcctl1Spec> {
        IntpulseposW::new(self, 2)
    }
    #[doc = "Bit 7 - ADC Power Down"]
    #[inline(always)]
    pub fn adcpwdnz(&mut self) -> AdcpwdnzW<'_, Adcctl1Spec> {
        AdcpwdnzW::new(self, 7)
    }
}
#[doc = "ADC Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl1Spec;
impl crate::RegisterSpec for Adcctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctl1::R`](R) reader structure"]
impl crate::Readable for Adcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl1::W`](W) writer structure"]
impl crate::Writable for Adcctl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCCTL1 to value 0"]
impl crate::Resettable for Adcctl1Spec {}
