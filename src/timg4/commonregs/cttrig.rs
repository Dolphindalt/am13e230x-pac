#[doc = "Register `CTTRIG` writer"]
pub type W = crate::W<CttrigSpec>;
#[doc = "Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Cross trigger generation disabled"]
    Disabled = 0,
    #[doc = "1: Generate Cross trigger pulse"]
    Generate = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` writer - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cross trigger generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::Disabled)
    }
    #[doc = "Generate Cross trigger pulse"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::Generate)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CttrigSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, CttrigSpec> {
        TrigW::new(self, 0)
    }
}
#[doc = "Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CttrigSpec;
impl crate::RegisterSpec for CttrigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cttrig::W`](W) writer structure"]
impl crate::Writable for CttrigSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTTRIG to value 0"]
impl crate::Resettable for CttrigSpec {}
