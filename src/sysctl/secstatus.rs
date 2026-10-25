#[doc = "Register `SECSTATUS` reader"]
pub type R = crate::R<SecstatusSpec>;
#[doc = "Field `INITDONE` reader - 1: CSC has been completed"]
pub type InitdoneR = crate::BitReader;
#[doc = "Field `CSCEXISTS` reader - 1: CSC Exists in the system"]
pub type CscexistsR = crate::BitReader;
#[doc = "1: Upper and Lower Banks allowed to be swapped\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flbankswppolicy {
    #[doc = "0: DISABLED"]
    Disabled = 0,
    #[doc = "1: ENABLED"]
    Enabled = 1,
}
impl From<Flbankswppolicy> for bool {
    #[inline(always)]
    fn from(variant: Flbankswppolicy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLBANKSWPPOLICY` reader - 1: Upper and Lower Banks allowed to be swapped"]
pub type FlbankswppolicyR = crate::BitReader<Flbankswppolicy>;
impl FlbankswppolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flbankswppolicy {
        match self.bits {
            false => Flbankswppolicy::Disabled,
            true => Flbankswppolicy::Enabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flbankswppolicy::Disabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flbankswppolicy::Enabled
    }
}
#[doc = "Field `FLBANKSWP` reader - 1: Upper and Lower Banks have been swapped"]
pub type FlbankswpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: CSC has been completed"]
    #[inline(always)]
    pub fn initdone(&self) -> InitdoneR {
        InitdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: CSC Exists in the system"]
    #[inline(always)]
    pub fn cscexists(&self) -> CscexistsR {
        CscexistsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Upper and Lower Banks allowed to be swapped"]
    #[inline(always)]
    pub fn flbankswppolicy(&self) -> FlbankswppolicyR {
        FlbankswppolicyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Upper and Lower Banks have been swapped"]
    #[inline(always)]
    pub fn flbankswp(&self) -> FlbankswpR {
        FlbankswpR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECSTATUS")
            .field("initdone", &self.initdone())
            .field("cscexists", &self.cscexists())
            .field("flbankswppolicy", &self.flbankswppolicy())
            .field("flbankswp", &self.flbankswp())
            .finish()
    }
}
#[doc = "Security Configuration status\n\nYou can [`read`](crate::Reg::read) this register and get [`secstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecstatusSpec;
impl crate::RegisterSpec for SecstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secstatus::R`](R) reader structure"]
impl crate::Readable for SecstatusSpec {}
#[doc = "`reset()` method sets SECSTATUS to value 0"]
impl crate::Resettable for SecstatusSpec {}
