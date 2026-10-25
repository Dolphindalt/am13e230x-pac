#[doc = "Register `HSCLKCFG` reader"]
pub type R = crate::R<HsclkcfgSpec>;
#[doc = "Register `HSCLKCFG` writer"]
pub type W = crate::W<HsclkcfgSpec>;
#[doc = "HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclksel {
    #[doc = "0: HSCLK is sourced from the SYSPLL"]
    Syspll = 0,
    #[doc = "1: HSCLK is sourced from the HFCLK"]
    Hfclkclk = 1,
}
impl From<Hsclksel> for bool {
    #[inline(always)]
    fn from(variant: Hsclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKSEL` reader - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
pub type HsclkselR = crate::BitReader<Hsclksel>;
impl HsclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclksel {
        match self.bits {
            false => Hsclksel::Syspll,
            true => Hsclksel::Hfclkclk,
        }
    }
    #[doc = "HSCLK is sourced from the SYSPLL"]
    #[inline(always)]
    pub fn is_syspll(&self) -> bool {
        *self == Hsclksel::Syspll
    }
    #[doc = "HSCLK is sourced from the HFCLK"]
    #[inline(always)]
    pub fn is_hfclkclk(&self) -> bool {
        *self == Hsclksel::Hfclkclk
    }
}
#[doc = "Field `HSCLKSEL` writer - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
pub type HsclkselW<'a, REG> = crate::BitWriter<'a, REG, Hsclksel>;
impl<'a, REG> HsclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSCLK is sourced from the SYSPLL"]
    #[inline(always)]
    pub fn syspll(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclksel::Syspll)
    }
    #[doc = "HSCLK is sourced from the HFCLK"]
    #[inline(always)]
    pub fn hfclkclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclksel::Hfclkclk)
    }
}
impl R {
    #[doc = "Bit 0 - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
    #[inline(always)]
    pub fn hsclksel(&self) -> HsclkselR {
        HsclkselR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCLKCFG")
            .field("hsclksel", &self.hsclksel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HSCLKSEL selects the HSCLK source (SYSPLL or HFCLK)."]
    #[inline(always)]
    pub fn hsclksel(&mut self) -> HsclkselW<'_, HsclkcfgSpec> {
        HsclkselW::new(self, 0)
    }
}
#[doc = "High-speed clock (HSCLK) source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsclkcfgSpec;
impl crate::RegisterSpec for HsclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsclkcfg::R`](R) reader structure"]
impl crate::Readable for HsclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hsclkcfg::W`](W) writer structure"]
impl crate::Writable for HsclkcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HSCLKCFG to value 0"]
impl crate::Resettable for HsclkcfgSpec {}
