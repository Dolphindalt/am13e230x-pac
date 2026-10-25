#[doc = "Register `PLLSTARTUP0_16_32MHZ` reader"]
pub type R = crate::R<Pllstartup0_16_32mhzSpec>;
#[doc = "Field `STARTTIME` reader - Startup time from Enable to Locked Clock in resolution of 1usec"]
pub type StarttimeR = crate::FieldReader;
#[doc = "Field `STARTTIMELP` reader - Startup time from Low Power Exit to Locked Clock in resolution of 1usec"]
pub type StarttimelpR = crate::FieldReader;
#[doc = "Field `CPCURRENT` reader - Charge Pump Current"]
pub type CpcurrentR = crate::FieldReader;
#[doc = "Field `CAPBVAL` reader - Override Value for Cap B"]
pub type CapbvalR = crate::FieldReader;
#[doc = "Override Enable For Cap B\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capboverride {
    #[doc = "0: DISABLE"]
    Disable = 0,
    #[doc = "1: ENABLE"]
    Enable = 1,
}
impl From<Capboverride> for bool {
    #[inline(always)]
    fn from(variant: Capboverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPBOVERRIDE` reader - Override Enable For Cap B"]
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
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Capboverride::Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Capboverride::Enable
    }
}
impl R {
    #[doc = "Bits 0:5 - Startup time from Enable to Locked Clock in resolution of 1usec"]
    #[inline(always)]
    pub fn starttime(&self) -> StarttimeR {
        StarttimeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Startup time from Low Power Exit to Locked Clock in resolution of 1usec"]
    #[inline(always)]
    pub fn starttimelp(&self) -> StarttimelpR {
        StarttimelpR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Charge Pump Current"]
    #[inline(always)]
    pub fn cpcurrent(&self) -> CpcurrentR {
        CpcurrentR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Override Value for Cap B"]
    #[inline(always)]
    pub fn capbval(&self) -> CapbvalR {
        CapbvalR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Override Enable For Cap B"]
    #[inline(always)]
    pub fn capboverride(&self) -> CapboverrideR {
        CapboverrideR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSTARTUP0_16_32MHZ")
            .field("starttime", &self.starttime())
            .field("starttimelp", &self.starttimelp())
            .field("cpcurrent", &self.cpcurrent())
            .field("capbval", &self.capbval())
            .field("capboverride", &self.capboverride())
            .finish()
    }
}
#[doc = "PLLSTARTUP0_16_32MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup0_16_32mhz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllstartup0_16_32mhzSpec;
impl crate::RegisterSpec for Pllstartup0_16_32mhzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstartup0_16_32mhz::R`](R) reader structure"]
impl crate::Readable for Pllstartup0_16_32mhzSpec {}
#[doc = "`reset()` method sets PLLSTARTUP0_16_32MHZ to value 0"]
impl crate::Resettable for Pllstartup0_16_32mhzSpec {}
