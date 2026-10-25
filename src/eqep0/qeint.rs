#[doc = "Register `QEINT` reader"]
pub type R = crate::R<QeintSpec>;
#[doc = "Register `QEINT` writer"]
pub type W = crate::W<QeintSpec>;
#[doc = "Position counter error interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Position counter error interrupt enable"]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::Disable,
            true => Pce::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pce::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pce::Enable
    }
}
#[doc = "Field `PCE` writer - Position counter error interrupt enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG, Pce>;
impl<'a, REG> PceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Enable)
    }
}
#[doc = "Quadrature phase error interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qpe {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Qpe> for bool {
    #[inline(always)]
    fn from(variant: Qpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QPE` reader - Quadrature phase error interrupt enable"]
pub type QpeR = crate::BitReader<Qpe>;
impl QpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qpe {
        match self.bits {
            false => Qpe::Disable,
            true => Qpe::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qpe::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Qpe::Enable
    }
}
#[doc = "Field `QPE` writer - Quadrature phase error interrupt enable"]
pub type QpeW<'a, REG> = crate::BitWriter<'a, REG, Qpe>;
impl<'a, REG> QpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qpe::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Qpe::Enable)
    }
}
#[doc = "Quadrature direction change interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdc {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Qdc> for bool {
    #[inline(always)]
    fn from(variant: Qdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDC` reader - Quadrature direction change interrupt enable"]
pub type QdcR = crate::BitReader<Qdc>;
impl QdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdc {
        match self.bits {
            false => Qdc::Disable,
            true => Qdc::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qdc::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Qdc::Enable
    }
}
#[doc = "Field `QDC` writer - Quadrature direction change interrupt enable"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG, Qdc>;
impl<'a, REG> QdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qdc::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Qdc::Enable)
    }
}
#[doc = "Watchdog time out interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wto {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Wto> for bool {
    #[inline(always)]
    fn from(variant: Wto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTO` reader - Watchdog time out interrupt enable"]
pub type WtoR = crate::BitReader<Wto>;
impl WtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wto {
        match self.bits {
            false => Wto::Disable,
            true => Wto::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wto::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wto::Enable
    }
}
#[doc = "Field `WTO` writer - Watchdog time out interrupt enable"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG, Wto>;
impl<'a, REG> WtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wto::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wto::Enable)
    }
}
#[doc = "Position counter underflow interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcu {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Pcu> for bool {
    #[inline(always)]
    fn from(variant: Pcu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCU` reader - Position counter underflow interrupt enable"]
pub type PcuR = crate::BitReader<Pcu>;
impl PcuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcu {
        match self.bits {
            false => Pcu::Disable,
            true => Pcu::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pcu::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pcu::Enable
    }
}
#[doc = "Field `PCU` writer - Position counter underflow interrupt enable"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG, Pcu>;
impl<'a, REG> PcuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcu::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcu::Enable)
    }
}
#[doc = "Position counter overflow interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pco {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Pco> for bool {
    #[inline(always)]
    fn from(variant: Pco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCO` reader - Position counter overflow interrupt enable"]
pub type PcoR = crate::BitReader<Pco>;
impl PcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pco {
        match self.bits {
            false => Pco::Disable,
            true => Pco::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pco::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pco::Enable
    }
}
#[doc = "Field `PCO` writer - Position counter overflow interrupt enable"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG, Pco>;
impl<'a, REG> PcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pco::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pco::Enable)
    }
}
#[doc = "Position-compare ready interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Position-compare ready interrupt enable"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::Disable,
            true => Pcr::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pcr::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pcr::Enable
    }
}
#[doc = "Field `PCR` writer - Position-compare ready interrupt enable"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::Enable)
    }
}
#[doc = "Position-compare match interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcm {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Pcm> for bool {
    #[inline(always)]
    fn from(variant: Pcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM` reader - Position-compare match interrupt enable"]
pub type PcmR = crate::BitReader<Pcm>;
impl PcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcm {
        match self.bits {
            false => Pcm::Disable,
            true => Pcm::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pcm::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pcm::Enable
    }
}
#[doc = "Field `PCM` writer - Position-compare match interrupt enable"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG, Pcm>;
impl<'a, REG> PcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcm::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcm::Enable)
    }
}
#[doc = "Strobe event latch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Strobe event latch interrupt enable"]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Disable,
            true => Sel::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sel::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sel::Enable
    }
}
#[doc = "Field `SEL` writer - Strobe event latch interrupt enable"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enable)
    }
}
#[doc = "Index event latch interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iel {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Iel> for bool {
    #[inline(always)]
    fn from(variant: Iel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEL` reader - Index event latch interrupt enable"]
pub type IelR = crate::BitReader<Iel>;
impl IelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iel {
        match self.bits {
            false => Iel::Disable,
            true => Iel::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Iel::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Iel::Enable
    }
}
#[doc = "Field `IEL` writer - Index event latch interrupt enable"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG, Iel>;
impl<'a, REG> IelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Enable)
    }
}
#[doc = "Unit time out interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uto {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Uto> for bool {
    #[inline(always)]
    fn from(variant: Uto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTO` reader - Unit time out interrupt enable"]
pub type UtoR = crate::BitReader<Uto>;
impl UtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uto {
        match self.bits {
            false => Uto::Disable,
            true => Uto::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uto::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uto::Enable
    }
}
#[doc = "Field `UTO` writer - Unit time out interrupt enable"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG, Uto>;
impl<'a, REG> UtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uto::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uto::Enable)
    }
}
#[doc = "QMA error interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qmae {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Qmae> for bool {
    #[inline(always)]
    fn from(variant: Qmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QMAE` reader - QMA error interrupt enable"]
pub type QmaeR = crate::BitReader<Qmae>;
impl QmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qmae {
        match self.bits {
            false => Qmae::Disable,
            true => Qmae::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qmae::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Qmae::Enable
    }
}
#[doc = "Field `QMAE` writer - QMA error interrupt enable"]
pub type QmaeW<'a, REG> = crate::BitWriter<'a, REG, Qmae>;
impl<'a, REG> QmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qmae::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Qmae::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - Position counter error interrupt enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt enable"]
    #[inline(always)]
    pub fn qpe(&self) -> QpeR {
        QpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt enable"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog time out interrupt enable"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt enable"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt enable"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt enable"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Position-compare match interrupt enable"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt enable"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Index event latch interrupt enable"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Unit time out interrupt enable"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - QMA error interrupt enable"]
    #[inline(always)]
    pub fn qmae(&self) -> QmaeR {
        QmaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QEINT")
            .field("pce", &self.pce())
            .field("qpe", &self.qpe())
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
impl W {
    #[doc = "Bit 1 - Position counter error interrupt enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, QeintSpec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt enable"]
    #[inline(always)]
    pub fn qpe(&mut self) -> QpeW<'_, QeintSpec> {
        QpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt enable"]
    #[inline(always)]
    pub fn qdc(&mut self) -> QdcW<'_, QeintSpec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog time out interrupt enable"]
    #[inline(always)]
    pub fn wto(&mut self) -> WtoW<'_, QeintSpec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt enable"]
    #[inline(always)]
    pub fn pcu(&mut self) -> PcuW<'_, QeintSpec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt enable"]
    #[inline(always)]
    pub fn pco(&mut self) -> PcoW<'_, QeintSpec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt enable"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<'_, QeintSpec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - Position-compare match interrupt enable"]
    #[inline(always)]
    pub fn pcm(&mut self) -> PcmW<'_, QeintSpec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt enable"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, QeintSpec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - Index event latch interrupt enable"]
    #[inline(always)]
    pub fn iel(&mut self) -> IelW<'_, QeintSpec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - Unit time out interrupt enable"]
    #[inline(always)]
    pub fn uto(&mut self) -> UtoW<'_, QeintSpec> {
        UtoW::new(self, 11)
    }
    #[doc = "Bit 12 - QMA error interrupt enable"]
    #[inline(always)]
    pub fn qmae(&mut self) -> QmaeW<'_, QeintSpec> {
        QmaeW::new(self, 12)
    }
}
#[doc = "QEP Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qeint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QeintSpec;
impl crate::RegisterSpec for QeintSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qeint::R`](R) reader structure"]
impl crate::Readable for QeintSpec {}
#[doc = "`write(|w| ..)` method takes [`qeint::W`](W) writer structure"]
impl crate::Writable for QeintSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QEINT to value 0"]
impl crate::Resettable for QeintSpec {}
