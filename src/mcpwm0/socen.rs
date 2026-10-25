#[doc = "Register `SOCEN` reader"]
pub type R = crate::R<SocenSpec>;
#[doc = "Register `SOCEN` writer"]
pub type W = crate::W<SocenSpec>;
#[doc = "Field `SOCA_ENABLE` reader - Start of conversion (SOC) A enable"]
pub type SocaEnableR = crate::BitReader;
#[doc = "Field `SOCA_ENABLE` writer - Start of conversion (SOC) A enable"]
pub type SocaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB_ENABLE` reader - Start of conversion (SOC) B enable"]
pub type SocbEnableR = crate::BitReader;
#[doc = "Field `SOCB_ENABLE` writer - Start of conversion (SOC) B enable"]
pub type SocbEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCC_ENABLE` reader - Start of conversion (SOC) C enable"]
pub type SoccEnableR = crate::BitReader;
#[doc = "Field `SOCC_ENABLE` writer - Start of conversion (SOC) C enable"]
pub type SoccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCD_ENABLE` reader - Start of conversion (SOC) D enable"]
pub type SocdEnableR = crate::BitReader;
#[doc = "Field `SOCD_ENABLE` writer - Start of conversion (SOC) D enable"]
pub type SocdEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of conversion (SOC) A enable"]
    #[inline(always)]
    pub fn soca_enable(&self) -> SocaEnableR {
        SocaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of conversion (SOC) B enable"]
    #[inline(always)]
    pub fn socb_enable(&self) -> SocbEnableR {
        SocbEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of conversion (SOC) C enable"]
    #[inline(always)]
    pub fn socc_enable(&self) -> SoccEnableR {
        SoccEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of conversion (SOC) D enable"]
    #[inline(always)]
    pub fn socd_enable(&self) -> SocdEnableR {
        SocdEnableR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCEN")
            .field("soca_enable", &self.soca_enable())
            .field("socb_enable", &self.socb_enable())
            .field("socc_enable", &self.socc_enable())
            .field("socd_enable", &self.socd_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start of conversion (SOC) A enable"]
    #[inline(always)]
    pub fn soca_enable(&mut self) -> SocaEnableW<'_, SocenSpec> {
        SocaEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of conversion (SOC) B enable"]
    #[inline(always)]
    pub fn socb_enable(&mut self) -> SocbEnableW<'_, SocenSpec> {
        SocbEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of conversion (SOC) C enable"]
    #[inline(always)]
    pub fn socc_enable(&mut self) -> SoccEnableW<'_, SocenSpec> {
        SoccEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of conversion (SOC) D enable"]
    #[inline(always)]
    pub fn socd_enable(&mut self) -> SocdEnableW<'_, SocenSpec> {
        SocdEnableW::new(self, 3)
    }
}
#[doc = "Start of conversion enable\n\nYou can [`read`](crate::Reg::read) this register and get [`socen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocenSpec;
impl crate::RegisterSpec for SocenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`socen::R`](R) reader structure"]
impl crate::Readable for SocenSpec {}
#[doc = "`write(|w| ..)` method takes [`socen::W`](W) writer structure"]
impl crate::Writable for SocenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCEN to value 0"]
impl crate::Resettable for SocenSpec {}
