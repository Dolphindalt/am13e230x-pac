#[doc = "Register `SYSPLLCFG1` reader"]
pub type R = crate::R<Syspllcfg1Spec>;
#[doc = "Register `SYSPLLCFG1` writer"]
pub type W = crate::W<Syspllcfg1Spec>;
#[doc = "PDIV selects the SYSPLL reference clock prescale divider.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pdiv {
    #[doc = "0: SYSPLLREF is not divided"]
    Refdiv1 = 0,
    #[doc = "1: SYSPLLREF is divided by 2"]
    Refdiv2 = 1,
    #[doc = "2: SYSPLLREF is divided by 4"]
    Refdiv4 = 2,
    #[doc = "3: SYSPLLREF is divided by 8"]
    Refdiv8 = 3,
}
impl From<Pdiv> for u8 {
    #[inline(always)]
    fn from(variant: Pdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdiv {
    type Ux = u8;
}
impl crate::IsEnum for Pdiv {}
#[doc = "Field `PDIV` reader - PDIV selects the SYSPLL reference clock prescale divider."]
pub type PdivR = crate::FieldReader<Pdiv>;
impl PdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdiv {
        match self.bits {
            0 => Pdiv::Refdiv1,
            1 => Pdiv::Refdiv2,
            2 => Pdiv::Refdiv4,
            3 => Pdiv::Refdiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "SYSPLLREF is not divided"]
    #[inline(always)]
    pub fn is_refdiv1(&self) -> bool {
        *self == Pdiv::Refdiv1
    }
    #[doc = "SYSPLLREF is divided by 2"]
    #[inline(always)]
    pub fn is_refdiv2(&self) -> bool {
        *self == Pdiv::Refdiv2
    }
    #[doc = "SYSPLLREF is divided by 4"]
    #[inline(always)]
    pub fn is_refdiv4(&self) -> bool {
        *self == Pdiv::Refdiv4
    }
    #[doc = "SYSPLLREF is divided by 8"]
    #[inline(always)]
    pub fn is_refdiv8(&self) -> bool {
        *self == Pdiv::Refdiv8
    }
}
#[doc = "Field `PDIV` writer - PDIV selects the SYSPLL reference clock prescale divider."]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pdiv, crate::Safe>;
impl<'a, REG> PdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSPLLREF is not divided"]
    #[inline(always)]
    pub fn refdiv1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::Refdiv1)
    }
    #[doc = "SYSPLLREF is divided by 2"]
    #[inline(always)]
    pub fn refdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::Refdiv2)
    }
    #[doc = "SYSPLLREF is divided by 4"]
    #[inline(always)]
    pub fn refdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::Refdiv4)
    }
    #[doc = "SYSPLLREF is divided by 8"]
    #[inline(always)]
    pub fn refdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::Refdiv8)
    }
}
#[doc = "QDIV selects the SYSPLL feedback path divider.\n\nValue on reset: 35"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qdiv {
    #[doc = "0: Divide-by-one is not a valid QDIV option"]
    Invalid = 0,
    #[doc = "1: Feedback path is divided by 2"]
    Qdivmin = 1,
    #[doc = "126: Feedback path is divided by 127 (0x7E)"]
    Qdivmax = 126,
}
impl From<Qdiv> for u8 {
    #[inline(always)]
    fn from(variant: Qdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qdiv {
    type Ux = u8;
}
impl crate::IsEnum for Qdiv {}
#[doc = "Field `QDIV` reader - QDIV selects the SYSPLL feedback path divider."]
pub type QdivR = crate::FieldReader<Qdiv>;
impl QdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qdiv> {
        match self.bits {
            0 => Some(Qdiv::Invalid),
            1 => Some(Qdiv::Qdivmin),
            126 => Some(Qdiv::Qdivmax),
            _ => None,
        }
    }
    #[doc = "Divide-by-one is not a valid QDIV option"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Qdiv::Invalid
    }
    #[doc = "Feedback path is divided by 2"]
    #[inline(always)]
    pub fn is_qdivmin(&self) -> bool {
        *self == Qdiv::Qdivmin
    }
    #[doc = "Feedback path is divided by 127 (0x7E)"]
    #[inline(always)]
    pub fn is_qdivmax(&self) -> bool {
        *self == Qdiv::Qdivmax
    }
}
#[doc = "Field `QDIV` writer - QDIV selects the SYSPLL feedback path divider."]
pub type QdivW<'a, REG> = crate::FieldWriter<'a, REG, 7, Qdiv, crate::Safe>;
impl<'a, REG> QdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-one is not a valid QDIV option"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(Qdiv::Invalid)
    }
    #[doc = "Feedback path is divided by 2"]
    #[inline(always)]
    pub fn qdivmin(self) -> &'a mut crate::W<REG> {
        self.variant(Qdiv::Qdivmin)
    }
    #[doc = "Feedback path is divided by 127 (0x7E)"]
    #[inline(always)]
    pub fn qdivmax(self) -> &'a mut crate::W<REG> {
        self.variant(Qdiv::Qdivmax)
    }
}
impl R {
    #[doc = "Bits 0:1 - PDIV selects the SYSPLL reference clock prescale divider."]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - QDIV selects the SYSPLL feedback path divider."]
    #[inline(always)]
    pub fn qdiv(&self) -> QdivR {
        QdivR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLCFG1")
            .field("pdiv", &self.pdiv())
            .field("qdiv", &self.qdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - PDIV selects the SYSPLL reference clock prescale divider."]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PdivW<'_, Syspllcfg1Spec> {
        PdivW::new(self, 0)
    }
    #[doc = "Bits 8:14 - QDIV selects the SYSPLL feedback path divider."]
    #[inline(always)]
    pub fn qdiv(&mut self) -> QdivW<'_, Syspllcfg1Spec> {
        QdivW::new(self, 8)
    }
}
#[doc = "SYSPLL reference and feedback divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllcfg1Spec;
impl crate::RegisterSpec for Syspllcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllcfg1::R`](R) reader structure"]
impl crate::Readable for Syspllcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllcfg1::W`](W) writer structure"]
impl crate::Writable for Syspllcfg1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLCFG1 to value 0x2300"]
impl crate::Resettable for Syspllcfg1Spec {
    const RESET_VALUE: u32 = 0x2300;
}
