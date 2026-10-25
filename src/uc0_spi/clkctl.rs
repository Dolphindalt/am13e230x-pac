#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "Field `SCR` reader - Serial clock divider"]
pub type ScrR = crate::FieldReader<u16>;
#[doc = "Field `SCR` writer - Serial clock divider"]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `DSAMPLE` reader - Delayed sampling."]
pub type DsampleR = crate::FieldReader;
#[doc = "Field `DSAMPLE` writer - Delayed sampling."]
pub type DsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:9 - Serial clock divider"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Delayed sampling."]
    #[inline(always)]
    pub fn dsample(&self) -> DsampleR {
        DsampleR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTL")
            .field("scr", &self.scr())
            .field("dsample", &self.dsample())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Serial clock divider"]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, ClkctlSpec> {
        ScrW::new(self, 0)
    }
    #[doc = "Bits 28:31 - Delayed sampling."]
    #[inline(always)]
    pub fn dsample(&mut self) -> DsampleW<'_, ClkctlSpec> {
        DsampleW::new(self, 28)
    }
}
#[doc = "Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {}
