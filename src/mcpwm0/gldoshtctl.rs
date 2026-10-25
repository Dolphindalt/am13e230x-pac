#[doc = "Register `GLDOSHTCTL` reader"]
pub type R = crate::R<GldoshtctlSpec>;
#[doc = "Register `GLDOSHTCTL` writer"]
pub type W = crate::W<GldoshtctlSpec>;
#[doc = "Field `OSHTLD` reader - Enable Reload Event in One Shot Mode"]
pub type OshtldR = crate::BitReader;
#[doc = "Field `OSHTLD` writer - Enable Reload Event in One Shot Mode"]
pub type OshtldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSHTCLR` reader - Enable Reload Event in One Shot Mode"]
pub type OshtclrR = crate::BitReader;
#[doc = "Field `OSHTCLR` writer - Enable Reload Event in One Shot Mode"]
pub type OshtclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFRCLD` reader - Force load event in one shot mode"]
pub type GfrcldR = crate::BitReader;
#[doc = "Field `GFRCLD` writer - Force load event in one shot mode"]
pub type GfrcldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Reload Event in One Shot Mode"]
    #[inline(always)]
    pub fn oshtld(&self) -> OshtldR {
        OshtldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reload Event in One Shot Mode"]
    #[inline(always)]
    pub fn oshtclr(&self) -> OshtclrR {
        OshtclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force load event in one shot mode"]
    #[inline(always)]
    pub fn gfrcld(&self) -> GfrcldR {
        GfrcldR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLDOSHTCTL")
            .field("oshtld", &self.oshtld())
            .field("oshtclr", &self.oshtclr())
            .field("gfrcld", &self.gfrcld())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Reload Event in One Shot Mode"]
    #[inline(always)]
    pub fn oshtld(&mut self) -> OshtldW<'_, GldoshtctlSpec> {
        OshtldW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reload Event in One Shot Mode"]
    #[inline(always)]
    pub fn oshtclr(&mut self) -> OshtclrW<'_, GldoshtctlSpec> {
        OshtclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force load event in one shot mode"]
    #[inline(always)]
    pub fn gfrcld(&mut self) -> GfrcldW<'_, GldoshtctlSpec> {
        GfrcldW::new(self, 2)
    }
}
#[doc = "Global load one shot control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldoshtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldoshtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GldoshtctlSpec;
impl crate::RegisterSpec for GldoshtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gldoshtctl::R`](R) reader structure"]
impl crate::Readable for GldoshtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gldoshtctl::W`](W) writer structure"]
impl crate::Writable for GldoshtctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GLDOSHTCTL to value 0"]
impl crate::Resettable for GldoshtctlSpec {}
