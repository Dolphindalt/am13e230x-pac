#[doc = "Register `QEPSTS` reader"]
pub type R = crate::R<QepstsSpec>;
#[doc = "Register `QEPSTS` writer"]
pub type W = crate::W<QepstsSpec>;
#[doc = "Position counter error flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcef {
    #[doc = "0: No error occurred during the last index transition"]
    Noerror = 0,
    #[doc = "1: Position counter error"]
    Error = 1,
}
impl From<Pcef> for bool {
    #[inline(always)]
    fn from(variant: Pcef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCEF` reader - Position counter error flag."]
pub type PcefR = crate::BitReader<Pcef>;
impl PcefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcef {
        match self.bits {
            false => Pcef::Noerror,
            true => Pcef::Error,
        }
    }
    #[doc = "No error occurred during the last index transition"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Pcef::Noerror
    }
    #[doc = "Position counter error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pcef::Error
    }
}
#[doc = "First index marker flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fimf {
    #[doc = "0: First index pulse has not occurred."]
    Wrt1 = 0,
    #[doc = "1: Set by first occurrence of index pulse. This bit is cleared by writing a '1'."]
    Setindex = 1,
}
impl From<Fimf> for bool {
    #[inline(always)]
    fn from(variant: Fimf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIMF` reader - First index marker flag"]
pub type FimfR = crate::BitReader<Fimf>;
impl FimfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fimf {
        match self.bits {
            false => Fimf::Wrt1,
            true => Fimf::Setindex,
        }
    }
    #[doc = "First index pulse has not occurred."]
    #[inline(always)]
    pub fn is_wrt1(&self) -> bool {
        *self == Fimf::Wrt1
    }
    #[doc = "Set by first occurrence of index pulse. This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn is_setindex(&self) -> bool {
        *self == Fimf::Setindex
    }
}
#[doc = "Field `FIMF` writer - First index marker flag"]
pub type FimfW<'a, REG> = crate::BitWriter<'a, REG, Fimf>;
impl<'a, REG> FimfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First index pulse has not occurred."]
    #[inline(always)]
    pub fn wrt1(self) -> &'a mut crate::W<REG> {
        self.variant(Fimf::Wrt1)
    }
    #[doc = "Set by first occurrence of index pulse. This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn setindex(self) -> &'a mut crate::W<REG> {
        self.variant(Fimf::Setindex)
    }
}
#[doc = "Capture direction error flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdef {
    #[doc = "0: Capture direction error has not occurred."]
    Wrt1 = 0,
    #[doc = "1: Direction change occurred between the capture position event. This bit is cleared by writing a '1'."]
    Direct = 1,
}
impl From<Cdef> for bool {
    #[inline(always)]
    fn from(variant: Cdef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEF` reader - Capture direction error flag"]
pub type CdefR = crate::BitReader<Cdef>;
impl CdefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdef {
        match self.bits {
            false => Cdef::Wrt1,
            true => Cdef::Direct,
        }
    }
    #[doc = "Capture direction error has not occurred."]
    #[inline(always)]
    pub fn is_wrt1(&self) -> bool {
        *self == Cdef::Wrt1
    }
    #[doc = "Direction change occurred between the capture position event. This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Cdef::Direct
    }
}
#[doc = "Field `CDEF` writer - Capture direction error flag"]
pub type CdefW<'a, REG> = crate::BitWriter<'a, REG, Cdef>;
impl<'a, REG> CdefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture direction error has not occurred."]
    #[inline(always)]
    pub fn wrt1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdef::Wrt1)
    }
    #[doc = "Direction change occurred between the capture position event. This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Cdef::Direct)
    }
}
#[doc = "Capture overflow error flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coef {
    #[doc = "0: Overflow has not occurred."]
    Wrt1 = 0,
    #[doc = "1: Overflow occurred in eQEP Capture timer (QEPCTMR). This bit is cleared by writing a '1'."]
    Ovf = 1,
}
impl From<Coef> for bool {
    #[inline(always)]
    fn from(variant: Coef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COEF` reader - Capture overflow error flag"]
pub type CoefR = crate::BitReader<Coef>;
impl CoefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coef {
        match self.bits {
            false => Coef::Wrt1,
            true => Coef::Ovf,
        }
    }
    #[doc = "Overflow has not occurred."]
    #[inline(always)]
    pub fn is_wrt1(&self) -> bool {
        *self == Coef::Wrt1
    }
    #[doc = "Overflow occurred in eQEP Capture timer (QEPCTMR). This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn is_ovf(&self) -> bool {
        *self == Coef::Ovf
    }
}
#[doc = "Field `COEF` writer - Capture overflow error flag"]
pub type CoefW<'a, REG> = crate::BitWriter<'a, REG, Coef>;
impl<'a, REG> CoefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow has not occurred."]
    #[inline(always)]
    pub fn wrt1(self) -> &'a mut crate::W<REG> {
        self.variant(Coef::Wrt1)
    }
    #[doc = "Overflow occurred in eQEP Capture timer (QEPCTMR). This bit is cleared by writing a '1'."]
    #[inline(always)]
    pub fn ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Coef::Ovf)
    }
}
#[doc = "eQEP direction latch flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdlf {
    #[doc = "0: Counter-clockwise rotation (or reverse movement) on index event marker"]
    Counterclk = 0,
    #[doc = "1: Clockwise rotation (or forward movement) on index event marker"]
    Clk = 1,
}
impl From<Qdlf> for bool {
    #[inline(always)]
    fn from(variant: Qdlf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDLF` reader - eQEP direction latch flag"]
pub type QdlfR = crate::BitReader<Qdlf>;
impl QdlfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdlf {
        match self.bits {
            false => Qdlf::Counterclk,
            true => Qdlf::Clk,
        }
    }
    #[doc = "Counter-clockwise rotation (or reverse movement) on index event marker"]
    #[inline(always)]
    pub fn is_counterclk(&self) -> bool {
        *self == Qdlf::Counterclk
    }
    #[doc = "Clockwise rotation (or forward movement) on index event marker"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Qdlf::Clk
    }
}
#[doc = "Quadrature direction flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdf {
    #[doc = "0: Counter-clockwise rotation (or reverse movement)"]
    Counterclk = 0,
    #[doc = "1: Clockwise rotation (or forward movement)"]
    Clk = 1,
}
impl From<Qdf> for bool {
    #[inline(always)]
    fn from(variant: Qdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDF` reader - Quadrature direction flag"]
pub type QdfR = crate::BitReader<Qdf>;
impl QdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdf {
        match self.bits {
            false => Qdf::Counterclk,
            true => Qdf::Clk,
        }
    }
    #[doc = "Counter-clockwise rotation (or reverse movement)"]
    #[inline(always)]
    pub fn is_counterclk(&self) -> bool {
        *self == Qdf::Counterclk
    }
    #[doc = "Clockwise rotation (or forward movement)"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Qdf::Clk
    }
}
#[doc = "The first index marker\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fidf {
    #[doc = "0: Counter-clockwise rotation (or reverse movement) on the first index event"]
    Counterclk = 0,
    #[doc = "1: Clockwise rotation (or forward movement) on the first index event"]
    Clk = 1,
}
impl From<Fidf> for bool {
    #[inline(always)]
    fn from(variant: Fidf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIDF` reader - The first index marker"]
pub type FidfR = crate::BitReader<Fidf>;
impl FidfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fidf {
        match self.bits {
            false => Fidf::Counterclk,
            true => Fidf::Clk,
        }
    }
    #[doc = "Counter-clockwise rotation (or reverse movement) on the first index event"]
    #[inline(always)]
    pub fn is_counterclk(&self) -> bool {
        *self == Fidf::Counterclk
    }
    #[doc = "Clockwise rotation (or forward movement) on the first index event"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Fidf::Clk
    }
}
#[doc = "Unit position event flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upevnt {
    #[doc = "0: No unit position event detected"]
    Nodetct = 0,
    #[doc = "1: Unit position event detected. Write 1 to clear"]
    Detct = 1,
}
impl From<Upevnt> for bool {
    #[inline(always)]
    fn from(variant: Upevnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPEVNT` reader - Unit position event flag"]
pub type UpevntR = crate::BitReader<Upevnt>;
impl UpevntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upevnt {
        match self.bits {
            false => Upevnt::Nodetct,
            true => Upevnt::Detct,
        }
    }
    #[doc = "No unit position event detected"]
    #[inline(always)]
    pub fn is_nodetct(&self) -> bool {
        *self == Upevnt::Nodetct
    }
    #[doc = "Unit position event detected. Write 1 to clear"]
    #[inline(always)]
    pub fn is_detct(&self) -> bool {
        *self == Upevnt::Detct
    }
}
#[doc = "Field `UPEVNT` writer - Unit position event flag"]
pub type UpevntW<'a, REG> = crate::BitWriter<'a, REG, Upevnt>;
impl<'a, REG> UpevntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No unit position event detected"]
    #[inline(always)]
    pub fn nodetct(self) -> &'a mut crate::W<REG> {
        self.variant(Upevnt::Nodetct)
    }
    #[doc = "Unit position event detected. Write 1 to clear"]
    #[inline(always)]
    pub fn detct(self) -> &'a mut crate::W<REG> {
        self.variant(Upevnt::Detct)
    }
}
impl R {
    #[doc = "Bit 0 - Position counter error flag."]
    #[inline(always)]
    pub fn pcef(&self) -> PcefR {
        PcefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - First index marker flag"]
    #[inline(always)]
    pub fn fimf(&self) -> FimfR {
        FimfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture direction error flag"]
    #[inline(always)]
    pub fn cdef(&self) -> CdefR {
        CdefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture overflow error flag"]
    #[inline(always)]
    pub fn coef(&self) -> CoefR {
        CoefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - eQEP direction latch flag"]
    #[inline(always)]
    pub fn qdlf(&self) -> QdlfR {
        QdlfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature direction flag"]
    #[inline(always)]
    pub fn qdf(&self) -> QdfR {
        QdfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The first index marker"]
    #[inline(always)]
    pub fn fidf(&self) -> FidfR {
        FidfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unit position event flag"]
    #[inline(always)]
    pub fn upevnt(&self) -> UpevntR {
        UpevntR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QEPSTS")
            .field("pcef", &self.pcef())
            .field("fimf", &self.fimf())
            .field("cdef", &self.cdef())
            .field("coef", &self.coef())
            .field("qdlf", &self.qdlf())
            .field("qdf", &self.qdf())
            .field("fidf", &self.fidf())
            .field("upevnt", &self.upevnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - First index marker flag"]
    #[inline(always)]
    pub fn fimf(&mut self) -> FimfW<'_, QepstsSpec> {
        FimfW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture direction error flag"]
    #[inline(always)]
    pub fn cdef(&mut self) -> CdefW<'_, QepstsSpec> {
        CdefW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture overflow error flag"]
    #[inline(always)]
    pub fn coef(&mut self) -> CoefW<'_, QepstsSpec> {
        CoefW::new(self, 3)
    }
    #[doc = "Bit 7 - Unit position event flag"]
    #[inline(always)]
    pub fn upevnt(&mut self) -> UpevntW<'_, QepstsSpec> {
        UpevntW::new(self, 7)
    }
}
#[doc = "QEP Status\n\nYou can [`read`](crate::Reg::read) this register and get [`qepsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepstsSpec;
impl crate::RegisterSpec for QepstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qepsts::R`](R) reader structure"]
impl crate::Readable for QepstsSpec {}
#[doc = "`write(|w| ..)` method takes [`qepsts::W`](W) writer structure"]
impl crate::Writable for QepstsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QEPSTS to value 0"]
impl crate::Resettable for QepstsSpec {}
