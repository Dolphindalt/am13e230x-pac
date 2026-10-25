#[doc = "Register `SYSPLLPARAM0` reader"]
pub type R = crate::R<Syspllparam0Spec>;
#[doc = "Register `SYSPLLPARAM0` writer"]
pub type W = crate::W<Syspllparam0Spec>;
#[doc = "Field `STARTTIME` reader - Startup time from enable to locked clock, in 1us resolution"]
pub type StarttimeR = crate::FieldReader;
#[doc = "Field `STARTTIME` writer - Startup time from enable to locked clock, in 1us resolution"]
pub type StarttimeW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Field `STARTTIMELP` reader - Startup time from low power mode exit to locked clock, in 1us resolution"]
pub type StarttimelpR = crate::FieldReader;
#[doc = "Field `STARTTIMELP` writer - Startup time from low power mode exit to locked clock, in 1us resolution"]
pub type StarttimelpW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Field `CPCURRENT` reader - Charge pump current"]
pub type CpcurrentR = crate::FieldReader;
#[doc = "Field `CPCURRENT` writer - Charge pump current"]
pub type CpcurrentW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Field `CAPBVAL` reader - Override value for Cap B"]
pub type CapbvalR = crate::FieldReader;
#[doc = "Field `CAPBVAL` writer - Override value for Cap B"]
pub type CapbvalW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "CAPBOVERRIDE controls the override for Cap B\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capboverride {
    #[doc = "0: Cap B override disabled"]
    Disable = 0,
    #[doc = "1: Cap B override enabled"]
    Enable = 1,
}
impl From<Capboverride> for bool {
    #[inline(always)]
    fn from(variant: Capboverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPBOVERRIDE` reader - CAPBOVERRIDE controls the override for Cap B"]
pub type CapboverrideR = crate::BitReader<Capboverride>;
impl CapboverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capboverride {
        match self.bits {
            false => Capboverride::Disable,
            true => Capboverride::Enable,
        }
    }
    #[doc = "Cap B override disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capboverride::Disable
    }
    #[doc = "Cap B override enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capboverride::Enable
    }
}
#[doc = "Field `CAPBOVERRIDE` writer - CAPBOVERRIDE controls the override for Cap B"]
pub type CapboverrideW<'a, REG> = crate::BitWriter<'a, REG, Capboverride>;
impl<'a, REG> CapboverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cap B override disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Capboverride::Disable)
    }
    #[doc = "Cap B override enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Capboverride::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - Startup time from enable to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn starttime(&self) -> StarttimeR {
        StarttimeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Startup time from low power mode exit to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn starttimelp(&self) -> StarttimelpR {
        StarttimelpR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Charge pump current"]
    #[inline(always)]
    pub fn cpcurrent(&self) -> CpcurrentR {
        CpcurrentR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Override value for Cap B"]
    #[inline(always)]
    pub fn capbval(&self) -> CapbvalR {
        CapbvalR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - CAPBOVERRIDE controls the override for Cap B"]
    #[inline(always)]
    pub fn capboverride(&self) -> CapboverrideR {
        CapboverrideR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLPARAM0")
            .field("cpcurrent", &self.cpcurrent())
            .field("capboverride", &self.capboverride())
            .field("starttime", &self.starttime())
            .field("capbval", &self.capbval())
            .field("starttimelp", &self.starttimelp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Startup time from enable to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn starttime(&mut self) -> StarttimeW<'_, Syspllparam0Spec> {
        StarttimeW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Startup time from low power mode exit to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn starttimelp(&mut self) -> StarttimelpW<'_, Syspllparam0Spec> {
        StarttimelpW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Charge pump current"]
    #[inline(always)]
    pub fn cpcurrent(&mut self) -> CpcurrentW<'_, Syspllparam0Spec> {
        CpcurrentW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Override value for Cap B"]
    #[inline(always)]
    pub fn capbval(&mut self) -> CapbvalW<'_, Syspllparam0Spec> {
        CapbvalW::new(self, 24)
    }
    #[doc = "Bit 31 - CAPBOVERRIDE controls the override for Cap B"]
    #[inline(always)]
    pub fn capboverride(&mut self) -> CapboverrideW<'_, Syspllparam0Spec> {
        CapboverrideW::new(self, 31)
    }
}
#[doc = "SYSPLL PARAM0 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam0Spec;
impl crate::RegisterSpec for Syspllparam0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam0::R`](R) reader structure"]
impl crate::Readable for Syspllparam0Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllparam0::W`](W) writer structure"]
impl crate::Writable for Syspllparam0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLPARAM0 to value 0x810a_0000"]
impl crate::Resettable for Syspllparam0Spec {
    const RESET_VALUE: u32 = 0x810a_0000;
}
