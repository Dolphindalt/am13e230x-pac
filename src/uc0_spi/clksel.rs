#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<ClkselSpec>;
#[doc = "Selects buscclk as clock source if enabled\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BusclkSel {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<BusclkSel> for bool {
    #[inline(always)]
    fn from(variant: BusclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSCLK_SEL` reader - Selects buscclk as clock source if enabled"]
pub type BusclkSelR = crate::BitReader<BusclkSel>;
impl BusclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BusclkSel {
        match self.bits {
            false => BusclkSel::Disable,
            true => BusclkSel::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BusclkSel::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BusclkSel::Enable
    }
}
#[doc = "Field `BUSCLK_SEL` writer - Selects buscclk as clock source if enabled"]
pub type BusclkSelW<'a, REG> = crate::BitWriter<'a, REG, BusclkSel>;
impl<'a, REG> BusclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BusclkSel::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BusclkSel::Enable)
    }
}
impl R {
    #[doc = "Bit 3 - Selects buscclk as clock source if enabled"]
    #[inline(always)]
    pub fn busclk_sel(&self) -> BusclkSelR {
        BusclkSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSEL")
            .field("busclk_sel", &self.busclk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Selects buscclk as clock source if enabled"]
    #[inline(always)]
    pub fn busclk_sel(&mut self) -> BusclkSelW<'_, ClkselSpec> {
        BusclkSelW::new(self, 3)
    }
}
#[doc = "Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for ClkselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for ClkselSpec {}
