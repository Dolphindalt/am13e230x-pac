#[doc = "Register `SOCSEL` reader"]
pub type R = crate::R<SocselSpec>;
#[doc = "Register `SOCSEL` writer"]
pub type W = crate::W<SocselSpec>;
#[doc = "Field `SOCA_SEL` reader - Start of conversion (SOC) A select"]
pub type SocaSelR = crate::FieldReader;
#[doc = "Field `SOCA_SEL` writer - Start of conversion (SOC) A select"]
pub type SocaSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SOCB_SEL` reader - Start of conversion (SOC) B select"]
pub type SocbSelR = crate::FieldReader;
#[doc = "Field `SOCB_SEL` writer - Start of conversion (SOC) B select"]
pub type SocbSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SOCC_SEL` reader - Start of conversion (SOC) C select"]
pub type SoccSelR = crate::FieldReader;
#[doc = "Field `SOCC_SEL` writer - Start of conversion (SOC) C select"]
pub type SoccSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SOCD_SEL` reader - Start of conversion (SOC) D select"]
pub type SocdSelR = crate::FieldReader;
#[doc = "Field `SOCD_SEL` writer - Start of conversion (SOC) D select"]
pub type SocdSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - Start of conversion (SOC) A select"]
    #[inline(always)]
    pub fn soca_sel(&self) -> SocaSelR {
        SocaSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Start of conversion (SOC) B select"]
    #[inline(always)]
    pub fn socb_sel(&self) -> SocbSelR {
        SocbSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Start of conversion (SOC) C select"]
    #[inline(always)]
    pub fn socc_sel(&self) -> SoccSelR {
        SoccSelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Start of conversion (SOC) D select"]
    #[inline(always)]
    pub fn socd_sel(&self) -> SocdSelR {
        SocdSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCSEL")
            .field("soca_sel", &self.soca_sel())
            .field("socb_sel", &self.socb_sel())
            .field("socc_sel", &self.socc_sel())
            .field("socd_sel", &self.socd_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Start of conversion (SOC) A select"]
    #[inline(always)]
    pub fn soca_sel(&mut self) -> SocaSelW<'_, SocselSpec> {
        SocaSelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Start of conversion (SOC) B select"]
    #[inline(always)]
    pub fn socb_sel(&mut self) -> SocbSelW<'_, SocselSpec> {
        SocbSelW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Start of conversion (SOC) C select"]
    #[inline(always)]
    pub fn socc_sel(&mut self) -> SoccSelW<'_, SocselSpec> {
        SoccSelW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Start of conversion (SOC) D select"]
    #[inline(always)]
    pub fn socd_sel(&mut self) -> SocdSelW<'_, SocselSpec> {
        SocdSelW::new(self, 24)
    }
}
#[doc = "Start of conversion selection\n\nYou can [`read`](crate::Reg::read) this register and get [`socsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocselSpec;
impl crate::RegisterSpec for SocselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`socsel::R`](R) reader structure"]
impl crate::Readable for SocselSpec {}
#[doc = "`write(|w| ..)` method takes [`socsel::W`](W) writer structure"]
impl crate::Writable for SocselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCSEL to value 0"]
impl crate::Resettable for SocselSpec {}
