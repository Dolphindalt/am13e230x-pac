#[doc = "Register `CCLKCTL` reader"]
pub type R = crate::R<CclkctlSpec>;
#[doc = "Register `CCLKCTL` writer"]
pub type W = crate::W<CclkctlSpec>;
#[doc = "Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "0: Clock is disabled."]
    Disabled = 0,
    #[doc = "1: Clock is enabled"]
    Enabled = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::Disabled,
            true => Clken::Enabled,
        }
    }
    #[doc = "Clock is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clken::Disabled
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Clken::Enabled
    }
}
#[doc = "Field `CLKEN` writer - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Disabled)
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCLKCTL")
            .field("clken", &self.clken())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, CclkctlSpec> {
        ClkenW::new(self, 0)
    }
}
#[doc = "Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CclkctlSpec;
impl crate::RegisterSpec for CclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cclkctl::R`](R) reader structure"]
impl crate::Readable for CclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cclkctl::W`](W) writer structure"]
impl crate::Writable for CclkctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCLKCTL to value 0"]
impl crate::Resettable for CclkctlSpec {}
