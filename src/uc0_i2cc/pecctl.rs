#[doc = "Register `PECCTL` reader"]
pub type R = crate::R<PecctlSpec>;
#[doc = "Register `PECCTL` writer"]
pub type W = crate::W<PecctlSpec>;
#[doc = "Field `PECCNT` reader - PEC Count"]
pub type PeccntR = crate::FieldReader<u16>;
#[doc = "Field `PECCNT` writer - PEC Count"]
pub type PeccntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
#[doc = "PEC Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecen {
    #[doc = "0: PEC is disabled"]
    Disable = 0,
    #[doc = "1: PEC is enabled"]
    Enable = 1,
}
impl From<Pecen> for bool {
    #[inline(always)]
    fn from(variant: Pecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC Enable"]
pub type PecenR = crate::BitReader<Pecen>;
impl PecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecen {
        match self.bits {
            false => Pecen::Disable,
            true => Pecen::Enable,
        }
    }
    #[doc = "PEC is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pecen::Disable
    }
    #[doc = "PEC is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pecen::Enable
    }
}
#[doc = "Field `PECEN` writer - PEC Enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG, Pecen>;
impl<'a, REG> PecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Disable)
    }
    #[doc = "PEC is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:8 - PEC Count"]
    #[inline(always)]
    pub fn peccnt(&self) -> PeccntR {
        PeccntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - PEC Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECCTL")
            .field("peccnt", &self.peccnt())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - PEC Count"]
    #[inline(always)]
    pub fn peccnt(&mut self) -> PeccntW<'_, PecctlSpec> {
        PeccntW::new(self, 0)
    }
    #[doc = "Bit 12 - PEC Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, PecctlSpec> {
        PecenW::new(self, 12)
    }
}
#[doc = "I2C PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pecctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PecctlSpec;
impl crate::RegisterSpec for PecctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecctl::R`](R) reader structure"]
impl crate::Readable for PecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pecctl::W`](W) writer structure"]
impl crate::Writable for PecctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PECCTL to value 0"]
impl crate::Resettable for PecctlSpec {}
