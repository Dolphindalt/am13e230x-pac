#[doc = "Register `FIFCTL` reader"]
pub type R = crate::R<FifctlSpec>;
#[doc = "Register `FIFCTL` writer"]
pub type W = crate::W<FifctlSpec>;
#[doc = "Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fp {
    #[doc = "0: Filter Period 3"]
    Per3 = 0,
    #[doc = "1: Filter Period 5"]
    Per5 = 1,
    #[doc = "2: Filter Period 8"]
    Per8 = 2,
}
impl From<Fp> for u8 {
    #[inline(always)]
    fn from(variant: Fp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fp {
    type Ux = u8;
}
impl crate::IsEnum for Fp {}
#[doc = "Field `FP` reader - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FpR = crate::FieldReader<Fp>;
impl FpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fp> {
        match self.bits {
            0 => Some(Fp::Per3),
            1 => Some(Fp::Per5),
            2 => Some(Fp::Per8),
            _ => None,
        }
    }
    #[doc = "Filter Period 3"]
    #[inline(always)]
    pub fn is_per_3(&self) -> bool {
        *self == Fp::Per3
    }
    #[doc = "Filter Period 5"]
    #[inline(always)]
    pub fn is_per_5(&self) -> bool {
        *self == Fp::Per5
    }
    #[doc = "Filter Period 8"]
    #[inline(always)]
    pub fn is_per_8(&self) -> bool {
        *self == Fp::Per8
    }
}
#[doc = "Field `FP` writer - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fp, crate::Safe>;
impl<'a, REG> FpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter Period 3"]
    #[inline(always)]
    pub fn per_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Per3)
    }
    #[doc = "Filter Period 5"]
    #[inline(always)]
    pub fn per_5(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Per5)
    }
    #[doc = "Filter Period 8"]
    #[inline(always)]
    pub fn per_8(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Per8)
    }
}
#[doc = "Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpv {
    #[doc = "0: Consecutive Periods. The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    ConsecPer = 0,
    #[doc = "1: Voting. The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output"]
    Voting = 1,
}
impl From<Cpv> for bool {
    #[inline(always)]
    fn from(variant: Cpv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPV` reader - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type CpvR = crate::BitReader<Cpv>;
impl CpvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpv {
        match self.bits {
            false => Cpv::ConsecPer,
            true => Cpv::Voting,
        }
    }
    #[doc = "Consecutive Periods. The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    #[inline(always)]
    pub fn is_consec_per(&self) -> bool {
        *self == Cpv::ConsecPer
    }
    #[doc = "Voting. The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output"]
    #[inline(always)]
    pub fn is_voting(&self) -> bool {
        *self == Cpv::Voting
    }
}
#[doc = "Field `CPV` writer - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type CpvW<'a, REG> = crate::BitWriter<'a, REG, Cpv>;
impl<'a, REG> CpvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Consecutive Periods. The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    #[inline(always)]
    pub fn consec_per(self) -> &'a mut crate::W<REG> {
        self.variant(Cpv::ConsecPer)
    }
    #[doc = "Voting. The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output"]
    #[inline(always)]
    pub fn voting(self) -> &'a mut crate::W<REG> {
        self.variant(Cpv::Voting)
    }
}
#[doc = "Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filten {
    #[doc = "0: Bypass"]
    Bypass = 0,
    #[doc = "1: Filtered."]
    Filtered = 1,
}
impl From<Filten> for bool {
    #[inline(always)]
    fn from(variant: Filten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTEN` reader - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
pub type FiltenR = crate::BitReader<Filten>;
impl FiltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filten {
        match self.bits {
            false => Filten::Bypass,
            true => Filten::Filtered,
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Filten::Bypass
    }
    #[doc = "Filtered."]
    #[inline(always)]
    pub fn is_filtered(&self) -> bool {
        *self == Filten::Filtered
    }
}
#[doc = "Field `FILTEN` writer - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
pub type FiltenW<'a, REG> = crate::BitWriter<'a, REG, Filten>;
impl<'a, REG> FiltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Filten::Bypass)
    }
    #[doc = "Filtered."]
    #[inline(always)]
    pub fn filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Filten::Filtered)
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn cpv(&self) -> CpvR {
        CpvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
    #[inline(always)]
    pub fn filten(&self) -> FiltenR {
        FiltenR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFCTL")
            .field("fp", &self.fp())
            .field("cpv", &self.cpv())
            .field("filten", &self.filten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fp(&mut self) -> FpW<'_, FifctlSpec> {
        FpW::new(self, 0)
    }
    #[doc = "Bit 3 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn cpv(&mut self) -> CpvW<'_, FifctlSpec> {
        CpvW::new(self, 3)
    }
    #[doc = "Bit 4 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
    #[inline(always)]
    pub fn filten(&mut self) -> FiltenW<'_, FifctlSpec> {
        FiltenW::new(self, 4)
    }
}
#[doc = "Fault input Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifctlSpec;
impl crate::RegisterSpec for FifctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifctl::R`](R) reader structure"]
impl crate::Readable for FifctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifctl::W`](W) writer structure"]
impl crate::Writable for FifctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FIFCTL to value 0"]
impl crate::Resettable for FifctlSpec {}
