#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "Error Raw Interrupt Stat Error Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errris {
    #[doc = "0: An error has not occurred."]
    ErrClr = 0,
    #[doc = "1: A WFIFO Full, a Read Stalled, or a Timeout error has occurred. To determine which error occurred, read the status of the EPI Error Interrupt Status and Clear (EPIEISC) register. This bit is cleared by writing a 1 to the bit in the EPIEISC register that caused the interrupt."]
    Err = 1,
}
impl From<Errris> for bool {
    #[inline(always)]
    fn from(variant: Errris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRRIS` reader - Error Raw Interrupt Stat Error Raw Interrupt Status"]
pub type ErrrisR = crate::BitReader<Errris>;
impl ErrrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errris {
        match self.bits {
            false => Errris::ErrClr,
            true => Errris::Err,
        }
    }
    #[doc = "An error has not occurred."]
    #[inline(always)]
    pub fn is_err_clr(&self) -> bool {
        *self == Errris::ErrClr
    }
    #[doc = "A WFIFO Full, a Read Stalled, or a Timeout error has occurred. To determine which error occurred, read the status of the EPI Error Interrupt Status and Clear (EPIEISC) register. This bit is cleared by writing a 1 to the bit in the EPIEISC register that caused the interrupt."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Errris::Err
    }
}
#[doc = "Field `ERRRIS` writer - Error Raw Interrupt Stat Error Raw Interrupt Status"]
pub type ErrrisW<'a, REG> = crate::BitWriter<'a, REG, Errris>;
impl<'a, REG> ErrrisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An error has not occurred."]
    #[inline(always)]
    pub fn err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Errris::ErrClr)
    }
    #[doc = "A WFIFO Full, a Read Stalled, or a Timeout error has occurred. To determine which error occurred, read the status of the EPI Error Interrupt Status and Clear (EPIEISC) register. This bit is cleared by writing a 1 to the bit in the EPIEISC register that caused the interrupt."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Errris::Err)
    }
}
#[doc = "Read Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdris {
    #[doc = "0: The number of valid entries in the NBRFIFO is below the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    RdClr = 0,
    #[doc = "1: The number of valid entries in the NBRFIFO is in the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    Rd = 1,
}
impl From<Rdris> for bool {
    #[inline(always)]
    fn from(variant: Rdris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRIS` reader - Read Raw Interrupt Status"]
pub type RdrisR = crate::BitReader<Rdris>;
impl RdrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdris {
        match self.bits {
            false => Rdris::RdClr,
            true => Rdris::Rd,
        }
    }
    #[doc = "The number of valid entries in the NBRFIFO is below the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn is_rd_clr(&self) -> bool {
        *self == Rdris::RdClr
    }
    #[doc = "The number of valid entries in the NBRFIFO is in the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn is_rd(&self) -> bool {
        *self == Rdris::Rd
    }
}
#[doc = "Field `RDRIS` writer - Read Raw Interrupt Status"]
pub type RdrisW<'a, REG> = crate::BitWriter<'a, REG, Rdris>;
impl<'a, REG> RdrisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of valid entries in the NBRFIFO is below the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn rd_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rdris::RdClr)
    }
    #[doc = "The number of valid entries in the NBRFIFO is in the trigger range specified by the RDFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn rd(self) -> &'a mut crate::W<REG> {
        self.variant(Rdris::Rd)
    }
}
#[doc = "Write Raw Interrupt Status\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrris {
    #[doc = "0: The number of available entries in the WFIFO is above the range specified by the WRFIFO field in the EPIFIFOLVL register."]
    WrClr = 0,
    #[doc = "1: The number of available entries in the WFIFO is within the trigger range specified by the WRFIFO field in the EPIFIFOLVL register."]
    Wr = 1,
}
impl From<Wrris> for bool {
    #[inline(always)]
    fn from(variant: Wrris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRRIS` reader - Write Raw Interrupt Status"]
pub type WrrisR = crate::BitReader<Wrris>;
impl WrrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrris {
        match self.bits {
            false => Wrris::WrClr,
            true => Wrris::Wr,
        }
    }
    #[doc = "The number of available entries in the WFIFO is above the range specified by the WRFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn is_wr_clr(&self) -> bool {
        *self == Wrris::WrClr
    }
    #[doc = "The number of available entries in the WFIFO is within the trigger range specified by the WRFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn is_wr(&self) -> bool {
        *self == Wrris::Wr
    }
}
#[doc = "Field `WRRIS` writer - Write Raw Interrupt Status"]
pub type WrrisW<'a, REG> = crate::BitWriter<'a, REG, Wrris>;
impl<'a, REG> WrrisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of available entries in the WFIFO is above the range specified by the WRFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn wr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Wrris::WrClr)
    }
    #[doc = "The number of available entries in the WFIFO is within the trigger range specified by the WRFIFO field in the EPIFIFOLVL register."]
    #[inline(always)]
    pub fn wr(self) -> &'a mut crate::W<REG> {
        self.variant(Wrris::Wr)
    }
}
#[doc = "Read uDMA Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmardris {
    #[doc = "0: The read uDMA has not completed."]
    DmardClr = 0,
    #[doc = "1: The read uDMA has completed."]
    Dmard = 1,
}
impl From<Dmardris> for bool {
    #[inline(always)]
    fn from(variant: Dmardris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARDRIS` reader - Read uDMA Raw Interrupt Status"]
pub type DmardrisR = crate::BitReader<Dmardris>;
impl DmardrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmardris {
        match self.bits {
            false => Dmardris::DmardClr,
            true => Dmardris::Dmard,
        }
    }
    #[doc = "The read uDMA has not completed."]
    #[inline(always)]
    pub fn is_dmard_clr(&self) -> bool {
        *self == Dmardris::DmardClr
    }
    #[doc = "The read uDMA has completed."]
    #[inline(always)]
    pub fn is_dmard(&self) -> bool {
        *self == Dmardris::Dmard
    }
}
#[doc = "Field `DMARDRIS` writer - Read uDMA Raw Interrupt Status"]
pub type DmardrisW<'a, REG> = crate::BitWriter<'a, REG, Dmardris>;
impl<'a, REG> DmardrisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The read uDMA has not completed."]
    #[inline(always)]
    pub fn dmard_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardris::DmardClr)
    }
    #[doc = "The read uDMA has completed."]
    #[inline(always)]
    pub fn dmard(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardris::Dmard)
    }
}
#[doc = "Write uDMA Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmawrris {
    #[doc = "0: The write uDMA has not completed."]
    DmawrClr = 0,
    #[doc = "1: The write uDMA has completed."]
    Dmawr = 1,
}
impl From<Dmawrris> for bool {
    #[inline(always)]
    fn from(variant: Dmawrris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWRRIS` reader - Write uDMA Raw Interrupt Status"]
pub type DmawrrisR = crate::BitReader<Dmawrris>;
impl DmawrrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmawrris {
        match self.bits {
            false => Dmawrris::DmawrClr,
            true => Dmawrris::Dmawr,
        }
    }
    #[doc = "The write uDMA has not completed."]
    #[inline(always)]
    pub fn is_dmawr_clr(&self) -> bool {
        *self == Dmawrris::DmawrClr
    }
    #[doc = "The write uDMA has completed."]
    #[inline(always)]
    pub fn is_dmawr(&self) -> bool {
        *self == Dmawrris::Dmawr
    }
}
#[doc = "Field `DMAWRRIS` writer - Write uDMA Raw Interrupt Status"]
pub type DmawrrisW<'a, REG> = crate::BitWriter<'a, REG, Dmawrris>;
impl<'a, REG> DmawrrisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write uDMA has not completed."]
    #[inline(always)]
    pub fn dmawr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrris::DmawrClr)
    }
    #[doc = "The write uDMA has completed."]
    #[inline(always)]
    pub fn dmawr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrris::Dmawr)
    }
}
impl R {
    #[doc = "Bit 0 - Error Raw Interrupt Stat Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn errris(&self) -> ErrrisR {
        ErrrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn rdris(&self) -> RdrisR {
        RdrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn wrris(&self) -> WrrisR {
        WrrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmardris(&self) -> DmardrisR {
        DmardrisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmawrris(&self) -> DmawrrisR {
        DmawrrisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("errris", &self.errris())
            .field("rdris", &self.rdris())
            .field("wrris", &self.wrris())
            .field("dmardris", &self.dmardris())
            .field("dmawrris", &self.dmawrris())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error Raw Interrupt Stat Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn errris(&mut self) -> ErrrisW<'_, RisSpec> {
        ErrrisW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn rdris(&mut self) -> RdrisW<'_, RisSpec> {
        RdrisW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn wrris(&mut self) -> WrrisW<'_, RisSpec> {
        WrrisW::new(self, 2)
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmardris(&mut self) -> DmardrisW<'_, RisSpec> {
        DmardrisW::new(self, 3)
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmawrris(&mut self) -> DmawrrisW<'_, RisSpec> {
        DmawrrisW::new(self, 4)
    }
}
#[doc = "EPI Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RIS to value 0x04"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0x04;
}
