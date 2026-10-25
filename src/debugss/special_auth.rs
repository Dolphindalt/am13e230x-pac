#[doc = "Register `SPECIAL_AUTH` reader"]
pub type R = crate::R<SpecialAuthSpec>;
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secapen {
    #[doc = "0: Disable SEC-AP"]
    Disable = 0,
    #[doc = "1: Enable SEC-AP"]
    Enable = 1,
}
impl From<Secapen> for bool {
    #[inline(always)]
    fn from(variant: Secapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP."]
pub type SecapenR = crate::BitReader<Secapen>;
impl SecapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secapen {
        match self.bits {
            false => Secapen::Disable,
            true => Secapen::Enable,
        }
    }
    #[doc = "Disable SEC-AP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Secapen::Disable
    }
    #[doc = "Enable SEC-AP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Secapen::Enable
    }
}
#[doc = "When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdporten {
    #[doc = "0: Disable SWD port"]
    Disable = 0,
    #[doc = "1: Enable SWD port"]
    Enable = 1,
}
impl From<Swdporten> for bool {
    #[inline(always)]
    fn from(variant: Swdporten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDPORTEN` reader - When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access."]
pub type SwdportenR = crate::BitReader<Swdporten>;
impl SwdportenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdporten {
        match self.bits {
            false => Swdporten::Disable,
            true => Swdporten::Enable,
        }
    }
    #[doc = "Disable SWD port"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Swdporten::Disable
    }
    #[doc = "Enable SWD port"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Swdporten::Enable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dftapen {
    #[doc = "0: Disable DFT-TAP"]
    Disable = 0,
    #[doc = "1: Enable DFT-TAP"]
    Enable = 1,
}
impl From<Dftapen> for bool {
    #[inline(always)]
    fn from(variant: Dftapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFTAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type DftapenR = crate::BitReader<Dftapen>;
impl DftapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dftapen {
        match self.bits {
            false => Dftapen::Disable,
            true => Dftapen::Enable,
        }
    }
    #[doc = "Disable DFT-TAP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dftapen::Disable
    }
    #[doc = "Enable DFT-TAP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dftapen::Enable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etapen {
    #[doc = "0: Disable ET+ -AP"]
    Disable = 0,
    #[doc = "1: Enable ET+ -AP"]
    Enable = 1,
}
impl From<Etapen> for bool {
    #[inline(always)]
    fn from(variant: Etapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type EtapenR = crate::BitReader<Etapen>;
impl EtapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etapen {
        match self.bits {
            false => Etapen::Disable,
            true => Etapen::Enable,
        }
    }
    #[doc = "Disable ET+ -AP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Etapen::Disable
    }
    #[doc = "Enable ET+ -AP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Etapen::Enable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgapen {
    #[doc = "0: Disable CFG-AP"]
    Disable = 0,
    #[doc = "1: Enable CFG-AP"]
    Enable = 1,
}
impl From<Cfgapen> for bool {
    #[inline(always)]
    fn from(variant: Cfgapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP."]
pub type CfgapenR = crate::BitReader<Cfgapen>;
impl CfgapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgapen {
        match self.bits {
            false => Cfgapen::Disable,
            true => Cfgapen::Enable,
        }
    }
    #[doc = "Disable CFG-AP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cfgapen::Disable
    }
    #[doc = "Enable CFG-AP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cfgapen::Enable
    }
}
#[doc = "Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbapen {
    #[doc = "0: Disable AHB-AP"]
    Disable = 0,
    #[doc = "1: Enable AHB-AP"]
    Enable = 1,
}
impl From<Ahbapen> for bool {
    #[inline(always)]
    fn from(variant: Ahbapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBAPEN` reader - Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation."]
pub type AhbapenR = crate::BitReader<Ahbapen>;
impl AhbapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahbapen {
        match self.bits {
            false => Ahbapen::Disable,
            true => Ahbapen::Enable,
        }
    }
    #[doc = "Disable AHB-AP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ahbapen::Disable
    }
    #[doc = "Enable AHB-AP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ahbapen::Enable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrapen {
    #[doc = "0: Disable PWR-AP"]
    Disable = 0,
    #[doc = "1: Enable PWR-AP"]
    Enable = 1,
}
impl From<Pwrapen> for bool {
    #[inline(always)]
    fn from(variant: Pwrapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type PwrapenR = crate::BitReader<Pwrapen>;
impl PwrapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrapen {
        match self.bits {
            false => Pwrapen::Disable,
            true => Pwrapen::Enable,
        }
    }
    #[doc = "Disable PWR-AP"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwrapen::Disable
    }
    #[doc = "Enable PWR-AP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwrapen::Enable
    }
}
impl R {
    #[doc = "Bit 0 - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP."]
    #[inline(always)]
    pub fn secapen(&self) -> SecapenR {
        SecapenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access."]
    #[inline(always)]
    pub fn swdporten(&self) -> SwdportenR {
        SwdportenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn dftapen(&self) -> DftapenR {
        DftapenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn etapen(&self) -> EtapenR {
        EtapenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP."]
    #[inline(always)]
    pub fn cfgapen(&self) -> CfgapenR {
        CfgapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation."]
    #[inline(always)]
    pub fn ahbapen(&self) -> AhbapenR {
        AhbapenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn pwrapen(&self) -> PwrapenR {
        PwrapenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPECIAL_AUTH")
            .field("secapen", &self.secapen())
            .field("swdporten", &self.swdporten())
            .field("dftapen", &self.dftapen())
            .field("etapen", &self.etapen())
            .field("cfgapen", &self.cfgapen())
            .field("ahbapen", &self.ahbapen())
            .field("pwrapen", &self.pwrapen())
            .finish()
    }
}
#[doc = "Special enable authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`special_auth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpecialAuthSpec;
impl crate::RegisterSpec for SpecialAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`special_auth::R`](R) reader structure"]
impl crate::Readable for SpecialAuthSpec {}
#[doc = "`reset()` method sets SPECIAL_AUTH to value 0x13"]
impl crate::Resettable for SpecialAuthSpec {
    const RESET_VALUE: u32 = 0x13;
}
