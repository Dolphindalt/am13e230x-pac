#[doc = "Register `QFLG` reader"]
pub type R = crate::R<QflgSpec>;
#[doc = "Register `QFLG` writer"]
pub type W = crate::W<QflgSpec>;
#[doc = "Global interrupt status flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Interrupt was generated"]
    Flag = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Global interrupt status flag"]
pub type IntR = crate::BitReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int {
        match self.bits {
            false => Int::Noflag,
            true => Int::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Int::Noflag
    }
    #[doc = "Interrupt was generated"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Int::Flag
    }
}
#[doc = "Position counter error interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Position counter error"]
    Flag = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Position counter error interrupt flag"]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::Noflag,
            true => Pce::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Pce::Noflag
    }
    #[doc = "Position counter error"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Pce::Flag
    }
}
#[doc = "Quadrature phase error interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phe {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Set on simultaneous transition of QEPA and QEPB"]
    Flag = 1,
}
impl From<Phe> for bool {
    #[inline(always)]
    fn from(variant: Phe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHE` reader - Quadrature phase error interrupt flag"]
pub type PheR = crate::BitReader<Phe>;
impl PheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phe {
        match self.bits {
            false => Phe::Noflag,
            true => Phe::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Phe::Noflag
    }
    #[doc = "Set on simultaneous transition of QEPA and QEPB"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Phe::Flag
    }
}
#[doc = "Quadrature direction change interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdc {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Interrupt was generated"]
    Flag = 1,
}
impl From<Qdc> for bool {
    #[inline(always)]
    fn from(variant: Qdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDC` reader - Quadrature direction change interrupt flag"]
pub type QdcR = crate::BitReader<Qdc>;
impl QdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdc {
        match self.bits {
            false => Qdc::Noflag,
            true => Qdc::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Qdc::Noflag
    }
    #[doc = "Interrupt was generated"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Qdc::Flag
    }
}
#[doc = "Watchdog timeout interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wto {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Set by watchdog timeout"]
    Flag = 1,
}
impl From<Wto> for bool {
    #[inline(always)]
    fn from(variant: Wto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTO` reader - Watchdog timeout interrupt flag"]
pub type WtoR = crate::BitReader<Wto>;
impl WtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wto {
        match self.bits {
            false => Wto::Noflag,
            true => Wto::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Wto::Noflag
    }
    #[doc = "Set by watchdog timeout"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Wto::Flag
    }
}
#[doc = "Position counter underflow interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcu {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set on position counter underflow."]
    Flag = 1,
}
impl From<Pcu> for bool {
    #[inline(always)]
    fn from(variant: Pcu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCU` reader - Position counter underflow interrupt flag"]
pub type PcuR = crate::BitReader<Pcu>;
impl PcuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcu {
        match self.bits {
            false => Pcu::Noflag,
            true => Pcu::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Pcu::Noflag
    }
    #[doc = "This bit is set on position counter underflow."]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Pcu::Flag
    }
}
#[doc = "Position counter overflow interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pco {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set on position counter overflow."]
    Flag = 1,
}
impl From<Pco> for bool {
    #[inline(always)]
    fn from(variant: Pco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCO` reader - Position counter overflow interrupt flag"]
pub type PcoR = crate::BitReader<Pco>;
impl PcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pco {
        match self.bits {
            false => Pco::Noflag,
            true => Pco::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Pco::Noflag
    }
    #[doc = "This bit is set on position counter overflow."]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Pco::Flag
    }
}
#[doc = "Position-compare ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set after transferring the shadow register value to the active position compare register"]
    Flag = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Position-compare ready interrupt flag"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::Noflag,
            true => Pcr::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Pcr::Noflag
    }
    #[doc = "This bit is set after transferring the shadow register value to the active position compare register"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Pcr::Flag
    }
}
#[doc = "eQEP compare match event interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcm {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set on position-compare match"]
    Flag = 1,
}
impl From<Pcm> for bool {
    #[inline(always)]
    fn from(variant: Pcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM` reader - eQEP compare match event interrupt flag"]
pub type PcmR = crate::BitReader<Pcm>;
impl PcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcm {
        match self.bits {
            false => Pcm::Noflag,
            true => Pcm::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Pcm::Noflag
    }
    #[doc = "This bit is set on position-compare match"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Pcm::Flag
    }
}
#[doc = "Strobe event latch interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set after latching the QPOSCNT to QPOSSLAT"]
    Flag = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Strobe event latch interrupt flag"]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Noflag,
            true => Sel::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Sel::Noflag
    }
    #[doc = "This bit is set after latching the QPOSCNT to QPOSSLAT"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Sel::Flag
    }
}
#[doc = "Index event latch interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iel {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: This bit is set after latching the QPOSCNT to QPOSILAT"]
    Flag = 1,
}
impl From<Iel> for bool {
    #[inline(always)]
    fn from(variant: Iel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEL` reader - Index event latch interrupt flag"]
pub type IelR = crate::BitReader<Iel>;
impl IelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iel {
        match self.bits {
            false => Iel::Noflag,
            true => Iel::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Iel::Noflag
    }
    #[doc = "This bit is set after latching the QPOSCNT to QPOSILAT"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Iel::Flag
    }
}
#[doc = "Unit time out interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uto {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Set by eQEP unit timer period match"]
    Flag = 1,
}
impl From<Uto> for bool {
    #[inline(always)]
    fn from(variant: Uto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTO` reader - Unit time out interrupt flag"]
pub type UtoR = crate::BitReader<Uto>;
impl UtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uto {
        match self.bits {
            false => Uto::Noflag,
            true => Uto::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Uto::Noflag
    }
    #[doc = "Set by eQEP unit timer period match"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Uto::Flag
    }
}
#[doc = "QMA error interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qmae {
    #[doc = "0: No interrupt generated"]
    Noflag = 0,
    #[doc = "1: Interrupt was generated"]
    Flag = 1,
}
impl From<Qmae> for bool {
    #[inline(always)]
    fn from(variant: Qmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QMAE` reader - QMA error interrupt flag"]
pub type QmaeR = crate::BitReader<Qmae>;
impl QmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qmae {
        match self.bits {
            false => Qmae::Noflag,
            true => Qmae::Flag,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_noflag(&self) -> bool {
        *self == Qmae::Noflag
    }
    #[doc = "Interrupt was generated"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == Qmae::Flag
    }
}
impl R {
    #[doc = "Bit 0 - Global interrupt status flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Position counter error interrupt flag"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt flag"]
    #[inline(always)]
    pub fn phe(&self) -> PheR {
        PheR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog timeout interrupt flag"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt flag"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - eQEP compare match event interrupt flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt flag"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Index event latch interrupt flag"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Unit time out interrupt flag"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - QMA error interrupt flag"]
    #[inline(always)]
    pub fn qmae(&self) -> QmaeR {
        QmaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QFLG")
            .field("int", &self.int())
            .field("pce", &self.pce())
            .field("phe", &self.phe())
            .field("qdc", &self.qdc())
            .field("wto", &self.wto())
            .field("pcu", &self.pcu())
            .field("pco", &self.pco())
            .field("pcr", &self.pcr())
            .field("pcm", &self.pcm())
            .field("sel", &self.sel())
            .field("iel", &self.iel())
            .field("uto", &self.uto())
            .field("qmae", &self.qmae())
            .finish()
    }
}
impl W {}
#[doc = "QEP Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`qflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QflgSpec;
impl crate::RegisterSpec for QflgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qflg::R`](R) reader structure"]
impl crate::Readable for QflgSpec {}
#[doc = "`write(|w| ..)` method takes [`qflg::W`](W) writer structure"]
impl crate::Writable for QflgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QFLG to value 0"]
impl crate::Resettable for QflgSpec {}
