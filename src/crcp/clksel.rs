#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Selects main clock (MCLK) if enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MclkSel {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<MclkSel> for bool {
    #[inline(always)]
    fn from(variant: MclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK_SEL` reader - Selects main clock (MCLK) if enabled"]
pub type MclkSelR = crate::BitReader<MclkSel>;
impl MclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MclkSel {
        match self.bits {
            false => MclkSel::Disable,
            true => MclkSel::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MclkSel::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MclkSel::Enable
    }
}
impl R {
    #[doc = "Bit 0 - Selects main clock (MCLK) if enabled"]
    #[inline(always)]
    pub fn mclk_sel(&self) -> MclkSelR {
        MclkSelR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSEL")
            .field("mclk_sel", &self.mclk_sel())
            .finish()
    }
}
#[doc = "Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`reset()` method sets CLKSEL to value 0x01"]
impl crate::Resettable for ClkselSpec {
    const RESET_VALUE: u32 = 0x01;
}
