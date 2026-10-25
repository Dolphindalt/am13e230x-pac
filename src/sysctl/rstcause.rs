#[doc = "Register `RSTCAUSE` reader"]
pub type R = crate::R<RstcauseSpec>;
#[doc = "ID is a read-to-clear field which indicates the lowest level reset cause since the last read.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: No reset since last read"]
    Norst = 0,
    #[doc = "1: POR- violation, SHUTDNSTOREx or PMU trim parity fault"]
    Porhwfail = 1,
    #[doc = "2: NRST triggered POR (>1s hold)"]
    Porexnrst = 2,
    #[doc = "3: Software triggered POR"]
    Porsw = 3,
    #[doc = "4: BOR0- violation"]
    Borsupply = 4,
    #[doc = "5: SHUTDOWN mode exit"]
    Borwakeshutdn = 5,
    #[doc = "8: Non-PMU trim parity fault"]
    Bootnonpmuparity = 8,
    #[doc = "9: Fatal clock failure"]
    Bootclkfail = 9,
    #[doc = "12: NRST triggered BOOTRST (<1s hold)"]
    Bootexnrst = 12,
    #[doc = "13: Software triggered BOOTRST"]
    Bootsw = 13,
    #[doc = "14: WWDT0 violation"]
    Bootwwdt0 = 14,
    #[doc = "16: BSL exit"]
    Sysbslexit = 16,
    #[doc = "17: BSL entry"]
    Sysbslentry = 17,
    #[doc = "20: Flash uncorrectable ECC error"]
    Sysflashecc = 20,
    #[doc = "21: CPULOCK violation"]
    Syscpulock = 21,
    #[doc = "26: Debug triggered SYSRST"]
    Sysdbg = 26,
    #[doc = "27: Software triggered SYSRST"]
    Syssw = 27,
    #[doc = "28: Debug triggered CPURST"]
    Cpudbg = 28,
    #[doc = "29: Software triggered CPURST"]
    Cpusw = 29,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - ID is a read-to-clear field which indicates the lowest level reset cause since the last read."]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Id> {
        match self.bits {
            0 => Some(Id::Norst),
            1 => Some(Id::Porhwfail),
            2 => Some(Id::Porexnrst),
            3 => Some(Id::Porsw),
            4 => Some(Id::Borsupply),
            5 => Some(Id::Borwakeshutdn),
            8 => Some(Id::Bootnonpmuparity),
            9 => Some(Id::Bootclkfail),
            12 => Some(Id::Bootexnrst),
            13 => Some(Id::Bootsw),
            14 => Some(Id::Bootwwdt0),
            16 => Some(Id::Sysbslexit),
            17 => Some(Id::Sysbslentry),
            20 => Some(Id::Sysflashecc),
            21 => Some(Id::Syscpulock),
            26 => Some(Id::Sysdbg),
            27 => Some(Id::Syssw),
            28 => Some(Id::Cpudbg),
            29 => Some(Id::Cpusw),
            _ => None,
        }
    }
    #[doc = "No reset since last read"]
    #[inline(always)]
    pub fn is_norst(&self) -> bool {
        *self == Id::Norst
    }
    #[doc = "POR- violation, SHUTDNSTOREx or PMU trim parity fault"]
    #[inline(always)]
    pub fn is_porhwfail(&self) -> bool {
        *self == Id::Porhwfail
    }
    #[doc = "NRST triggered POR (>1s hold)"]
    #[inline(always)]
    pub fn is_porexnrst(&self) -> bool {
        *self == Id::Porexnrst
    }
    #[doc = "Software triggered POR"]
    #[inline(always)]
    pub fn is_porsw(&self) -> bool {
        *self == Id::Porsw
    }
    #[doc = "BOR0- violation"]
    #[inline(always)]
    pub fn is_borsupply(&self) -> bool {
        *self == Id::Borsupply
    }
    #[doc = "SHUTDOWN mode exit"]
    #[inline(always)]
    pub fn is_borwakeshutdn(&self) -> bool {
        *self == Id::Borwakeshutdn
    }
    #[doc = "Non-PMU trim parity fault"]
    #[inline(always)]
    pub fn is_bootnonpmuparity(&self) -> bool {
        *self == Id::Bootnonpmuparity
    }
    #[doc = "Fatal clock failure"]
    #[inline(always)]
    pub fn is_bootclkfail(&self) -> bool {
        *self == Id::Bootclkfail
    }
    #[doc = "NRST triggered BOOTRST (<1s hold)"]
    #[inline(always)]
    pub fn is_bootexnrst(&self) -> bool {
        *self == Id::Bootexnrst
    }
    #[doc = "Software triggered BOOTRST"]
    #[inline(always)]
    pub fn is_bootsw(&self) -> bool {
        *self == Id::Bootsw
    }
    #[doc = "WWDT0 violation"]
    #[inline(always)]
    pub fn is_bootwwdt0(&self) -> bool {
        *self == Id::Bootwwdt0
    }
    #[doc = "BSL exit"]
    #[inline(always)]
    pub fn is_sysbslexit(&self) -> bool {
        *self == Id::Sysbslexit
    }
    #[doc = "BSL entry"]
    #[inline(always)]
    pub fn is_sysbslentry(&self) -> bool {
        *self == Id::Sysbslentry
    }
    #[doc = "Flash uncorrectable ECC error"]
    #[inline(always)]
    pub fn is_sysflashecc(&self) -> bool {
        *self == Id::Sysflashecc
    }
    #[doc = "CPULOCK violation"]
    #[inline(always)]
    pub fn is_syscpulock(&self) -> bool {
        *self == Id::Syscpulock
    }
    #[doc = "Debug triggered SYSRST"]
    #[inline(always)]
    pub fn is_sysdbg(&self) -> bool {
        *self == Id::Sysdbg
    }
    #[doc = "Software triggered SYSRST"]
    #[inline(always)]
    pub fn is_syssw(&self) -> bool {
        *self == Id::Syssw
    }
    #[doc = "Debug triggered CPURST"]
    #[inline(always)]
    pub fn is_cpudbg(&self) -> bool {
        *self == Id::Cpudbg
    }
    #[doc = "Software triggered CPURST"]
    #[inline(always)]
    pub fn is_cpusw(&self) -> bool {
        *self == Id::Cpusw
    }
}
impl R {
    #[doc = "Bits 0:4 - ID is a read-to-clear field which indicates the lowest level reset cause since the last read."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCAUSE").field("id", &self.id()).finish()
    }
}
#[doc = "Reset cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcauseSpec;
impl crate::RegisterSpec for RstcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcause::R`](R) reader structure"]
impl crate::Readable for RstcauseSpec {}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RstcauseSpec {}
