#[doc = "Register `INTCTL` writer"]
pub type W = crate::W<IntctlSpec>;
#[doc = "Writing a 1 to this field re-evaluates the interrupt sources.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inteval {
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Eval = 1,
}
impl From<Inteval> for bool {
    #[inline(always)]
    fn from(variant: Inteval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEVAL` writer - Writing a 1 to this field re-evaluates the interrupt sources."]
pub type IntevalW<'a, REG> = crate::BitWriter<'a, REG, Inteval>;
impl<'a, REG> IntevalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inteval::Disable)
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn eval(self) -> &'a mut crate::W<REG> {
        self.variant(Inteval::Eval)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IntctlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this field re-evaluates the interrupt sources."]
    #[inline(always)]
    pub fn inteval(&mut self) -> IntevalW<'_, IntctlSpec> {
        IntevalW::new(self, 0)
    }
}
#[doc = "Interrupt control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntctlSpec;
impl crate::RegisterSpec for IntctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intctl::W`](W) writer structure"]
impl crate::Writable for IntctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTCTL to value 0"]
impl crate::Resettable for IntctlSpec {}
