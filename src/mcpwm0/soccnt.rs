#[doc = "Register `SOCCNT` reader"]
pub type R = crate::R<SoccntSpec>;
#[doc = "Register `SOCCNT` writer"]
pub type W = crate::W<SoccntSpec>;
#[doc = "Field `SOCA_CNT` reader - Start of conversion (SOC) A COUNT"]
pub type SocaCntR = crate::FieldReader;
#[doc = "Field `SOCB_CNT` reader - Start of conversion (SOC) B COUNT"]
pub type SocbCntR = crate::FieldReader;
#[doc = "Field `SOCC_CNT` reader - Start of conversion (SOC) C COUNT"]
pub type SoccCntR = crate::FieldReader;
#[doc = "Field `SOCD_CNT` reader - Start of conversion (SOC) D COUNT"]
pub type SocdCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Start of conversion (SOC) A COUNT"]
    #[inline(always)]
    pub fn soca_cnt(&self) -> SocaCntR {
        SocaCntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Start of conversion (SOC) B COUNT"]
    #[inline(always)]
    pub fn socb_cnt(&self) -> SocbCntR {
        SocbCntR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Start of conversion (SOC) C COUNT"]
    #[inline(always)]
    pub fn socc_cnt(&self) -> SoccCntR {
        SoccCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Start of conversion (SOC) D COUNT"]
    #[inline(always)]
    pub fn socd_cnt(&self) -> SocdCntR {
        SocdCntR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCCNT")
            .field("soca_cnt", &self.soca_cnt())
            .field("socb_cnt", &self.socb_cnt())
            .field("socc_cnt", &self.socc_cnt())
            .field("socd_cnt", &self.socd_cnt())
            .finish()
    }
}
impl W {}
#[doc = "Start of conversion count\n\nYou can [`read`](crate::Reg::read) this register and get [`soccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoccntSpec;
impl crate::RegisterSpec for SoccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soccnt::R`](R) reader structure"]
impl crate::Readable for SoccntSpec {}
#[doc = "`write(|w| ..)` method takes [`soccnt::W`](W) writer structure"]
impl crate::Writable for SoccntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCCNT to value 0"]
impl crate::Resettable for SoccntSpec {}
