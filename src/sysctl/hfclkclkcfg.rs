#[doc = "Register `HFCLKCLKCFG` reader"]
pub type R = crate::R<HfclkclkcfgSpec>;
#[doc = "Register `HFCLKCLKCFG` writer"]
pub type W = crate::W<HfclkclkcfgSpec>;
#[doc = "XTALTIME specifies the XTAL startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), XTAL will be checked after this time expires.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xtaltime {
    #[doc = "0: Minimum startup time (approximatly zero)"]
    Minstarttime = 0,
    #[doc = "255: Maximum startup time (approximatly 16.32ms)"]
    Maxstarttime = 255,
}
impl From<Xtaltime> for u8 {
    #[inline(always)]
    fn from(variant: Xtaltime) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xtaltime {
    type Ux = u8;
}
impl crate::IsEnum for Xtaltime {}
#[doc = "Field `XTALTIME` reader - XTALTIME specifies the XTAL startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), XTAL will be checked after this time expires."]
pub type XtaltimeR = crate::FieldReader<Xtaltime>;
impl XtaltimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xtaltime> {
        match self.bits {
            0 => Some(Xtaltime::Minstarttime),
            255 => Some(Xtaltime::Maxstarttime),
            _ => None,
        }
    }
    #[doc = "Minimum startup time (approximatly zero)"]
    #[inline(always)]
    pub fn is_minstarttime(&self) -> bool {
        *self == Xtaltime::Minstarttime
    }
    #[doc = "Maximum startup time (approximatly 16.32ms)"]
    #[inline(always)]
    pub fn is_maxstarttime(&self) -> bool {
        *self == Xtaltime::Maxstarttime
    }
}
#[doc = "Field `XTALTIME` writer - XTALTIME specifies the XTAL startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), XTAL will be checked after this time expires."]
pub type XtaltimeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Xtaltime, crate::Safe>;
impl<'a, REG> XtaltimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum startup time (approximatly zero)"]
    #[inline(always)]
    pub fn minstarttime(self) -> &'a mut crate::W<REG> {
        self.variant(Xtaltime::Minstarttime)
    }
    #[doc = "Maximum startup time (approximatly 16.32ms)"]
    #[inline(always)]
    pub fn maxstarttime(self) -> &'a mut crate::W<REG> {
        self.variant(Xtaltime::Maxstarttime)
    }
}
#[doc = "HFCLKFLTCHK enables or disables the HFCLK startup monitor.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkfltchk {
    #[doc = "0: HFCLK startup is not checked"]
    Disable = 0,
    #[doc = "1: HFCLK startup is checked"]
    Enable = 1,
}
impl From<Hfclkfltchk> for bool {
    #[inline(always)]
    fn from(variant: Hfclkfltchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKFLTCHK` reader - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
pub type HfclkfltchkR = crate::BitReader<Hfclkfltchk>;
impl HfclkfltchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkfltchk {
        match self.bits {
            false => Hfclkfltchk::Disable,
            true => Hfclkfltchk::Enable,
        }
    }
    #[doc = "HFCLK startup is not checked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hfclkfltchk::Disable
    }
    #[doc = "HFCLK startup is checked"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hfclkfltchk::Enable
    }
}
#[doc = "Field `HFCLKFLTCHK` writer - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
pub type HfclkfltchkW<'a, REG> = crate::BitWriter<'a, REG, Hfclkfltchk>;
impl<'a, REG> HfclkfltchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFCLK startup is not checked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkfltchk::Disable)
    }
    #[doc = "HFCLK startup is checked"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclkfltchk::Enable)
    }
}
impl R {
    #[doc = "Bits 0:7 - XTALTIME specifies the XTAL startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), XTAL will be checked after this time expires."]
    #[inline(always)]
    pub fn xtaltime(&self) -> XtaltimeR {
        XtaltimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 28 - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
    #[inline(always)]
    pub fn hfclkfltchk(&self) -> HfclkfltchkR {
        HfclkfltchkR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCLKCLKCFG")
            .field("xtaltime", &self.xtaltime())
            .field("hfclkfltchk", &self.hfclkfltchk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - XTALTIME specifies the XTAL startup time in 64us resolution. If the HFCLK startup monitor is enabled (HFCLKFLTCHK), XTAL will be checked after this time expires."]
    #[inline(always)]
    pub fn xtaltime(&mut self) -> XtaltimeW<'_, HfclkclkcfgSpec> {
        XtaltimeW::new(self, 0)
    }
    #[doc = "Bit 28 - HFCLKFLTCHK enables or disables the HFCLK startup monitor."]
    #[inline(always)]
    pub fn hfclkfltchk(&mut self) -> HfclkfltchkW<'_, HfclkclkcfgSpec> {
        HfclkfltchkW::new(self, 28)
    }
}
#[doc = "High-frequency clock (HFCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkclkcfgSpec;
impl crate::RegisterSpec for HfclkclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkclkcfg::R`](R) reader structure"]
impl crate::Readable for HfclkclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclkclkcfg::W`](W) writer structure"]
impl crate::Writable for HfclkclkcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HFCLKCLKCFG to value 0x1000_0000"]
impl crate::Resettable for HfclkclkcfgSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
