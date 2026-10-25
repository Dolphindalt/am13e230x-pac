#[doc = "Register `QDECCTL` reader"]
pub type R = crate::R<QdecctlSpec>;
#[doc = "Register `QDECCTL` writer"]
pub type W = crate::W<QdecctlSpec>;
#[doc = "Field `QIDIRE` reader - Qep Index Direction Enhancement enable"]
pub type QidireR = crate::BitReader;
#[doc = "Field `QIDIRE` writer - Qep Index Direction Enhancement enable"]
pub type QidireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "QEPS input polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qsp {
    #[doc = "0: No effect"]
    Nopolar = 0,
    #[doc = "1: Negates QEPS input"]
    Polar = 1,
}
impl From<Qsp> for bool {
    #[inline(always)]
    fn from(variant: Qsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSP` reader - QEPS input polarity"]
pub type QspR = crate::BitReader<Qsp>;
impl QspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qsp {
        match self.bits {
            false => Qsp::Nopolar,
            true => Qsp::Polar,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_nopolar(&self) -> bool {
        *self == Qsp::Nopolar
    }
    #[doc = "Negates QEPS input"]
    #[inline(always)]
    pub fn is_polar(&self) -> bool {
        *self == Qsp::Polar
    }
}
#[doc = "Field `QSP` writer - QEPS input polarity"]
pub type QspW<'a, REG> = crate::BitWriter<'a, REG, Qsp>;
impl<'a, REG> QspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nopolar(self) -> &'a mut crate::W<REG> {
        self.variant(Qsp::Nopolar)
    }
    #[doc = "Negates QEPS input"]
    #[inline(always)]
    pub fn polar(self) -> &'a mut crate::W<REG> {
        self.variant(Qsp::Polar)
    }
}
#[doc = "QEPI input polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qip {
    #[doc = "0: No effect"]
    Nopolar = 0,
    #[doc = "1: Negates QEPI input"]
    Polar = 1,
}
impl From<Qip> for bool {
    #[inline(always)]
    fn from(variant: Qip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QIP` reader - QEPI input polarity"]
pub type QipR = crate::BitReader<Qip>;
impl QipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qip {
        match self.bits {
            false => Qip::Nopolar,
            true => Qip::Polar,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_nopolar(&self) -> bool {
        *self == Qip::Nopolar
    }
    #[doc = "Negates QEPI input"]
    #[inline(always)]
    pub fn is_polar(&self) -> bool {
        *self == Qip::Polar
    }
}
#[doc = "Field `QIP` writer - QEPI input polarity"]
pub type QipW<'a, REG> = crate::BitWriter<'a, REG, Qip>;
impl<'a, REG> QipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nopolar(self) -> &'a mut crate::W<REG> {
        self.variant(Qip::Nopolar)
    }
    #[doc = "Negates QEPI input"]
    #[inline(always)]
    pub fn polar(self) -> &'a mut crate::W<REG> {
        self.variant(Qip::Polar)
    }
}
#[doc = "QEPB input polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qbp {
    #[doc = "0: No effect"]
    Nopolar = 0,
    #[doc = "1: Negates QEPB input"]
    Polar = 1,
}
impl From<Qbp> for bool {
    #[inline(always)]
    fn from(variant: Qbp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QBP` reader - QEPB input polarity"]
pub type QbpR = crate::BitReader<Qbp>;
impl QbpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qbp {
        match self.bits {
            false => Qbp::Nopolar,
            true => Qbp::Polar,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_nopolar(&self) -> bool {
        *self == Qbp::Nopolar
    }
    #[doc = "Negates QEPB input"]
    #[inline(always)]
    pub fn is_polar(&self) -> bool {
        *self == Qbp::Polar
    }
}
#[doc = "Field `QBP` writer - QEPB input polarity"]
pub type QbpW<'a, REG> = crate::BitWriter<'a, REG, Qbp>;
impl<'a, REG> QbpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nopolar(self) -> &'a mut crate::W<REG> {
        self.variant(Qbp::Nopolar)
    }
    #[doc = "Negates QEPB input"]
    #[inline(always)]
    pub fn polar(self) -> &'a mut crate::W<REG> {
        self.variant(Qbp::Polar)
    }
}
#[doc = "QEPA input polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qap {
    #[doc = "0: No effect"]
    Nopolar = 0,
    #[doc = "1: Negates QEPA input"]
    Polar = 1,
}
impl From<Qap> for bool {
    #[inline(always)]
    fn from(variant: Qap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QAP` reader - QEPA input polarity"]
pub type QapR = crate::BitReader<Qap>;
impl QapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qap {
        match self.bits {
            false => Qap::Nopolar,
            true => Qap::Polar,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_nopolar(&self) -> bool {
        *self == Qap::Nopolar
    }
    #[doc = "Negates QEPA input"]
    #[inline(always)]
    pub fn is_polar(&self) -> bool {
        *self == Qap::Polar
    }
}
#[doc = "Field `QAP` writer - QEPA input polarity"]
pub type QapW<'a, REG> = crate::BitWriter<'a, REG, Qap>;
impl<'a, REG> QapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nopolar(self) -> &'a mut crate::W<REG> {
        self.variant(Qap::Nopolar)
    }
    #[doc = "Negates QEPA input"]
    #[inline(always)]
    pub fn polar(self) -> &'a mut crate::W<REG> {
        self.variant(Qap::Polar)
    }
}
#[doc = "Index pulse gating option\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Igate {
    #[doc = "0: Disable gating of Index pulse"]
    Disable = 0,
    #[doc = "1: Gate the index pin with strobe"]
    Enable = 1,
}
impl From<Igate> for bool {
    #[inline(always)]
    fn from(variant: Igate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IGATE` reader - Index pulse gating option"]
pub type IgateR = crate::BitReader<Igate>;
impl IgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Igate {
        match self.bits {
            false => Igate::Disable,
            true => Igate::Enable,
        }
    }
    #[doc = "Disable gating of Index pulse"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Igate::Disable
    }
    #[doc = "Gate the index pin with strobe"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Igate::Enable
    }
}
#[doc = "Field `IGATE` writer - Index pulse gating option"]
pub type IgateW<'a, REG> = crate::BitWriter<'a, REG, Igate>;
impl<'a, REG> IgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable gating of Index pulse"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Igate::Disable)
    }
    #[doc = "Gate the index pin with strobe"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Igate::Enable)
    }
}
#[doc = "CLK/DIR Signal Source for Position Counter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap {
    #[doc = "0: Quadrature-clock inputs are not swapped"]
    Disable = 0,
    #[doc = "1: Quadrature-clock inputs are swapped"]
    Enable = 1,
}
impl From<Swap> for bool {
    #[inline(always)]
    fn from(variant: Swap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - CLK/DIR Signal Source for Position Counter"]
pub type SwapR = crate::BitReader<Swap>;
impl SwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap {
        match self.bits {
            false => Swap::Disable,
            true => Swap::Enable,
        }
    }
    #[doc = "Quadrature-clock inputs are not swapped"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Swap::Disable
    }
    #[doc = "Quadrature-clock inputs are swapped"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Swap::Enable
    }
}
#[doc = "Field `SWAP` writer - CLK/DIR Signal Source for Position Counter"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG, Swap>;
impl<'a, REG> SwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature-clock inputs are not swapped"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::Disable)
    }
    #[doc = "Quadrature-clock inputs are swapped"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::Enable)
    }
}
#[doc = "External Clock Rate\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xcr {
    #[doc = "0: 2x resolution: Count the rising/falling edge"]
    Xcr2xresol = 0,
    #[doc = "1: 1x resolution: Count the rising edge only"]
    Xcr1xresol = 1,
}
impl From<Xcr> for bool {
    #[inline(always)]
    fn from(variant: Xcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XCR` reader - External Clock Rate"]
pub type XcrR = crate::BitReader<Xcr>;
impl XcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xcr {
        match self.bits {
            false => Xcr::Xcr2xresol,
            true => Xcr::Xcr1xresol,
        }
    }
    #[doc = "2x resolution: Count the rising/falling edge"]
    #[inline(always)]
    pub fn is_xcr_2xresol(&self) -> bool {
        *self == Xcr::Xcr2xresol
    }
    #[doc = "1x resolution: Count the rising edge only"]
    #[inline(always)]
    pub fn is_xcr_1xresol(&self) -> bool {
        *self == Xcr::Xcr1xresol
    }
}
#[doc = "Field `XCR` writer - External Clock Rate"]
pub type XcrW<'a, REG> = crate::BitWriter<'a, REG, Xcr>;
impl<'a, REG> XcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2x resolution: Count the rising/falling edge"]
    #[inline(always)]
    pub fn xcr_2xresol(self) -> &'a mut crate::W<REG> {
        self.variant(Xcr::Xcr2xresol)
    }
    #[doc = "1x resolution: Count the rising edge only"]
    #[inline(always)]
    pub fn xcr_1xresol(self) -> &'a mut crate::W<REG> {
        self.variant(Xcr::Xcr1xresol)
    }
}
#[doc = "Sync output pin selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spsel {
    #[doc = "0: Index pin is used for sync output"]
    Index = 0,
    #[doc = "1: Strobe pin is used for sync output"]
    Strobe = 1,
}
impl From<Spsel> for bool {
    #[inline(always)]
    fn from(variant: Spsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPSEL` reader - Sync output pin selection"]
pub type SpselR = crate::BitReader<Spsel>;
impl SpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spsel {
        match self.bits {
            false => Spsel::Index,
            true => Spsel::Strobe,
        }
    }
    #[doc = "Index pin is used for sync output"]
    #[inline(always)]
    pub fn is_index(&self) -> bool {
        *self == Spsel::Index
    }
    #[doc = "Strobe pin is used for sync output"]
    #[inline(always)]
    pub fn is_strobe(&self) -> bool {
        *self == Spsel::Strobe
    }
}
#[doc = "Field `SPSEL` writer - Sync output pin selection"]
pub type SpselW<'a, REG> = crate::BitWriter<'a, REG, Spsel>;
impl<'a, REG> SpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index pin is used for sync output"]
    #[inline(always)]
    pub fn index(self) -> &'a mut crate::W<REG> {
        self.variant(Spsel::Index)
    }
    #[doc = "Strobe pin is used for sync output"]
    #[inline(always)]
    pub fn strobe(self) -> &'a mut crate::W<REG> {
        self.variant(Spsel::Strobe)
    }
}
#[doc = "Sync output-enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soen {
    #[doc = "0: Disable position-compare sync output"]
    Disable = 0,
    #[doc = "1: Enable position-compare sync output"]
    Enable = 1,
}
impl From<Soen> for bool {
    #[inline(always)]
    fn from(variant: Soen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOEN` reader - Sync output-enable"]
pub type SoenR = crate::BitReader<Soen>;
impl SoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soen {
        match self.bits {
            false => Soen::Disable,
            true => Soen::Enable,
        }
    }
    #[doc = "Disable position-compare sync output"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Soen::Disable
    }
    #[doc = "Enable position-compare sync output"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Soen::Enable
    }
}
#[doc = "Field `SOEN` writer - Sync output-enable"]
pub type SoenW<'a, REG> = crate::BitWriter<'a, REG, Soen>;
impl<'a, REG> SoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable position-compare sync output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Soen::Disable)
    }
    #[doc = "Enable position-compare sync output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Soen::Enable)
    }
}
#[doc = "Position-counter source selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qsrc {
    #[doc = "0: Quadrature count mode (QCLK = iCLK, QDIR = iDIR)"]
    Quad = 0,
    #[doc = "1: Direction-count mode (QCLK = xCLK, QDIR = xDIR)"]
    Direct = 1,
    #[doc = "2: UP count mode for frequency measurement (QCLK = xCLK, QDIR = 1)"]
    Up = 2,
    #[doc = "3: DOWN count mode for frequency measurement (QCLK = xCLK, QDIR = 0)"]
    Down = 3,
}
impl From<Qsrc> for u8 {
    #[inline(always)]
    fn from(variant: Qsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qsrc {
    type Ux = u8;
}
impl crate::IsEnum for Qsrc {}
#[doc = "Field `QSRC` reader - Position-counter source selection"]
pub type QsrcR = crate::FieldReader<Qsrc>;
impl QsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qsrc {
        match self.bits {
            0 => Qsrc::Quad,
            1 => Qsrc::Direct,
            2 => Qsrc::Up,
            3 => Qsrc::Down,
            _ => unreachable!(),
        }
    }
    #[doc = "Quadrature count mode (QCLK = iCLK, QDIR = iDIR)"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Qsrc::Quad
    }
    #[doc = "Direction-count mode (QCLK = xCLK, QDIR = xDIR)"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Qsrc::Direct
    }
    #[doc = "UP count mode for frequency measurement (QCLK = xCLK, QDIR = 1)"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Qsrc::Up
    }
    #[doc = "DOWN count mode for frequency measurement (QCLK = xCLK, QDIR = 0)"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Qsrc::Down
    }
}
#[doc = "Field `QSRC` writer - Position-counter source selection"]
pub type QsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qsrc, crate::Safe>;
impl<'a, REG> QsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Quadrature count mode (QCLK = iCLK, QDIR = iDIR)"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Quad)
    }
    #[doc = "Direction-count mode (QCLK = xCLK, QDIR = xDIR)"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Direct)
    }
    #[doc = "UP count mode for frequency measurement (QCLK = xCLK, QDIR = 1)"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Up)
    }
    #[doc = "DOWN count mode for frequency measurement (QCLK = xCLK, QDIR = 0)"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Down)
    }
}
impl R {
    #[doc = "Bit 0 - Qep Index Direction Enhancement enable"]
    #[inline(always)]
    pub fn qidire(&self) -> QidireR {
        QidireR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - QEPS input polarity"]
    #[inline(always)]
    pub fn qsp(&self) -> QspR {
        QspR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - QEPI input polarity"]
    #[inline(always)]
    pub fn qip(&self) -> QipR {
        QipR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - QEPB input polarity"]
    #[inline(always)]
    pub fn qbp(&self) -> QbpR {
        QbpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - QEPA input polarity"]
    #[inline(always)]
    pub fn qap(&self) -> QapR {
        QapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Index pulse gating option"]
    #[inline(always)]
    pub fn igate(&self) -> IgateR {
        IgateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLK/DIR Signal Source for Position Counter"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Clock Rate"]
    #[inline(always)]
    pub fn xcr(&self) -> XcrR {
        XcrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sync output pin selection"]
    #[inline(always)]
    pub fn spsel(&self) -> SpselR {
        SpselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sync output-enable"]
    #[inline(always)]
    pub fn soen(&self) -> SoenR {
        SoenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Position-counter source selection"]
    #[inline(always)]
    pub fn qsrc(&self) -> QsrcR {
        QsrcR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QDECCTL")
            .field("qidire", &self.qidire())
            .field("qsp", &self.qsp())
            .field("qip", &self.qip())
            .field("qbp", &self.qbp())
            .field("qap", &self.qap())
            .field("igate", &self.igate())
            .field("swap", &self.swap())
            .field("xcr", &self.xcr())
            .field("spsel", &self.spsel())
            .field("soen", &self.soen())
            .field("qsrc", &self.qsrc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Qep Index Direction Enhancement enable"]
    #[inline(always)]
    pub fn qidire(&mut self) -> QidireW<'_, QdecctlSpec> {
        QidireW::new(self, 0)
    }
    #[doc = "Bit 5 - QEPS input polarity"]
    #[inline(always)]
    pub fn qsp(&mut self) -> QspW<'_, QdecctlSpec> {
        QspW::new(self, 5)
    }
    #[doc = "Bit 6 - QEPI input polarity"]
    #[inline(always)]
    pub fn qip(&mut self) -> QipW<'_, QdecctlSpec> {
        QipW::new(self, 6)
    }
    #[doc = "Bit 7 - QEPB input polarity"]
    #[inline(always)]
    pub fn qbp(&mut self) -> QbpW<'_, QdecctlSpec> {
        QbpW::new(self, 7)
    }
    #[doc = "Bit 8 - QEPA input polarity"]
    #[inline(always)]
    pub fn qap(&mut self) -> QapW<'_, QdecctlSpec> {
        QapW::new(self, 8)
    }
    #[doc = "Bit 9 - Index pulse gating option"]
    #[inline(always)]
    pub fn igate(&mut self) -> IgateW<'_, QdecctlSpec> {
        IgateW::new(self, 9)
    }
    #[doc = "Bit 10 - CLK/DIR Signal Source for Position Counter"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, QdecctlSpec> {
        SwapW::new(self, 10)
    }
    #[doc = "Bit 11 - External Clock Rate"]
    #[inline(always)]
    pub fn xcr(&mut self) -> XcrW<'_, QdecctlSpec> {
        XcrW::new(self, 11)
    }
    #[doc = "Bit 12 - Sync output pin selection"]
    #[inline(always)]
    pub fn spsel(&mut self) -> SpselW<'_, QdecctlSpec> {
        SpselW::new(self, 12)
    }
    #[doc = "Bit 13 - Sync output-enable"]
    #[inline(always)]
    pub fn soen(&mut self) -> SoenW<'_, QdecctlSpec> {
        SoenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Position-counter source selection"]
    #[inline(always)]
    pub fn qsrc(&mut self) -> QsrcW<'_, QdecctlSpec> {
        QsrcW::new(self, 14)
    }
}
#[doc = "Quadrature Decoder Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qdecctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdecctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdecctlSpec;
impl crate::RegisterSpec for QdecctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qdecctl::R`](R) reader structure"]
impl crate::Readable for QdecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qdecctl::W`](W) writer structure"]
impl crate::Writable for QdecctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QDECCTL to value 0"]
impl crate::Resettable for QdecctlSpec {}
