#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "FASTWAKEONLY for the global control of fastwake\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastwakeonly {
    #[doc = "0: The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    NotGlobal = 0,
    #[doc = "1: The global control of fastwake is enabled"]
    Global = 1,
}
impl From<Fastwakeonly> for bool {
    #[inline(always)]
    fn from(variant: Fastwakeonly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKEONLY` reader - FASTWAKEONLY for the global control of fastwake"]
pub type FastwakeonlyR = crate::BitReader<Fastwakeonly>;
impl FastwakeonlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastwakeonly {
        match self.bits {
            false => Fastwakeonly::NotGlobal,
            true => Fastwakeonly::Global,
        }
    }
    #[doc = "The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    #[inline(always)]
    pub fn is_not_global(&self) -> bool {
        *self == Fastwakeonly::NotGlobal
    }
    #[doc = "The global control of fastwake is enabled"]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == Fastwakeonly::Global
    }
}
#[doc = "Field `FASTWAKEONLY` writer - FASTWAKEONLY for the global control of fastwake"]
pub type FastwakeonlyW<'a, REG> = crate::BitWriter<'a, REG, Fastwakeonly>;
impl<'a, REG> FastwakeonlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    #[inline(always)]
    pub fn not_global(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwakeonly::NotGlobal)
    }
    #[doc = "The global control of fastwake is enabled"]
    #[inline(always)]
    pub fn global(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwakeonly::Global)
    }
}
impl R {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn fastwakeonly(&self) -> FastwakeonlyR {
        FastwakeonlyR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("fastwakeonly", &self.fastwakeonly())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn fastwakeonly(&mut self) -> FastwakeonlyW<'_, CtlSpec> {
        FastwakeonlyW::new(self, 0)
    }
}
#[doc = "FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
