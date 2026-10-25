#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Error Masked Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errmis {
    #[doc = "0: An error has not occurred or the interrupt is masked."]
    ErrClr = 0,
    #[doc = "1: A WFIFO Full, a Read Stalled, or a Timeout error has occurred and the ERIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    Err = 1,
}
impl From<Errmis> for bool {
    #[inline(always)]
    fn from(variant: Errmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRMIS` reader - Error Masked Interrupt Status"]
pub type ErrmisR = crate::BitReader<Errmis>;
impl ErrmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errmis {
        match self.bits {
            false => Errmis::ErrClr,
            true => Errmis::Err,
        }
    }
    #[doc = "An error has not occurred or the interrupt is masked."]
    #[inline(always)]
    pub fn is_err_clr(&self) -> bool {
        *self == Errmis::ErrClr
    }
    #[doc = "A WFIFO Full, a Read Stalled, or a Timeout error has occurred and the ERIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Errmis::Err
    }
}
#[doc = "Field `ERRMIS` writer - Error Masked Interrupt Status"]
pub type ErrmisW<'a, REG> = crate::BitWriter<'a, REG, Errmis>;
impl<'a, REG> ErrmisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An error has not occurred or the interrupt is masked."]
    #[inline(always)]
    pub fn err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Errmis::ErrClr)
    }
    #[doc = "A WFIFO Full, a Read Stalled, or a Timeout error has occurred and the ERIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Errmis::Err)
    }
}
#[doc = "Read Masked Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdmis {
    #[doc = "0: The number of valid entries in the NBRFIFO is below the range specified by the trigger level or the interrupt is masked."]
    RdClr = 0,
    #[doc = "1: The number of valid entries in the NBRFIFO is within the range specified by the trigger level (the RDFIFO field in the EPIFIFOLVL register) and the RDIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    Rd = 1,
}
impl From<Rdmis> for bool {
    #[inline(always)]
    fn from(variant: Rdmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMIS` reader - Read Masked Interrupt Status"]
pub type RdmisR = crate::BitReader<Rdmis>;
impl RdmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdmis {
        match self.bits {
            false => Rdmis::RdClr,
            true => Rdmis::Rd,
        }
    }
    #[doc = "The number of valid entries in the NBRFIFO is below the range specified by the trigger level or the interrupt is masked."]
    #[inline(always)]
    pub fn is_rd_clr(&self) -> bool {
        *self == Rdmis::RdClr
    }
    #[doc = "The number of valid entries in the NBRFIFO is within the range specified by the trigger level (the RDFIFO field in the EPIFIFOLVL register) and the RDIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_rd(&self) -> bool {
        *self == Rdmis::Rd
    }
}
#[doc = "Field `RDMIS` writer - Read Masked Interrupt Status"]
pub type RdmisW<'a, REG> = crate::BitWriter<'a, REG, Rdmis>;
impl<'a, REG> RdmisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of valid entries in the NBRFIFO is below the range specified by the trigger level or the interrupt is masked."]
    #[inline(always)]
    pub fn rd_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmis::RdClr)
    }
    #[doc = "The number of valid entries in the NBRFIFO is within the range specified by the trigger level (the RDFIFO field in the EPIFIFOLVL register) and the RDIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn rd(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmis::Rd)
    }
}
#[doc = "Write Masked Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrmis {
    #[doc = "0: The number of available entries in the WFIFO is above the range specified by the trigger level or the interrupt is masked."]
    WrClr = 0,
    #[doc = "1: The number of available entries in the WFIFO is within the range specified by the trigger level (the WRFIFO field in the EPIFIFOLVL register) and the WRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    Wr = 1,
}
impl From<Wrmis> for bool {
    #[inline(always)]
    fn from(variant: Wrmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRMIS` reader - Write Masked Interrupt Status"]
pub type WrmisR = crate::BitReader<Wrmis>;
impl WrmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrmis {
        match self.bits {
            false => Wrmis::WrClr,
            true => Wrmis::Wr,
        }
    }
    #[doc = "The number of available entries in the WFIFO is above the range specified by the trigger level or the interrupt is masked."]
    #[inline(always)]
    pub fn is_wr_clr(&self) -> bool {
        *self == Wrmis::WrClr
    }
    #[doc = "The number of available entries in the WFIFO is within the range specified by the trigger level (the WRFIFO field in the EPIFIFOLVL register) and the WRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_wr(&self) -> bool {
        *self == Wrmis::Wr
    }
}
#[doc = "Field `WRMIS` writer - Write Masked Interrupt Status"]
pub type WrmisW<'a, REG> = crate::BitWriter<'a, REG, Wrmis>;
impl<'a, REG> WrmisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of available entries in the WFIFO is above the range specified by the trigger level or the interrupt is masked."]
    #[inline(always)]
    pub fn wr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmis::WrClr)
    }
    #[doc = "The number of available entries in the WFIFO is within the range specified by the trigger level (the WRFIFO field in the EPIFIFOLVL register) and the WRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn wr(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmis::Wr)
    }
}
#[doc = "Read uDMA Masked Interrupt Status.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmardmis {
    #[doc = "0: The read uDMA has not completed or the interrupt is masked."]
    DmardClr = 0,
    #[doc = "1: The read uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    Dmard = 1,
}
impl From<Dmardmis> for bool {
    #[inline(always)]
    fn from(variant: Dmardmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARDMIS` reader - Read uDMA Masked Interrupt Status."]
pub type DmardmisR = crate::BitReader<Dmardmis>;
impl DmardmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmardmis {
        match self.bits {
            false => Dmardmis::DmardClr,
            true => Dmardmis::Dmard,
        }
    }
    #[doc = "The read uDMA has not completed or the interrupt is masked."]
    #[inline(always)]
    pub fn is_dmard_clr(&self) -> bool {
        *self == Dmardmis::DmardClr
    }
    #[doc = "The read uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_dmard(&self) -> bool {
        *self == Dmardmis::Dmard
    }
}
#[doc = "Field `DMARDMIS` writer - Read uDMA Masked Interrupt Status."]
pub type DmardmisW<'a, REG> = crate::BitWriter<'a, REG, Dmardmis>;
impl<'a, REG> DmardmisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The read uDMA has not completed or the interrupt is masked."]
    #[inline(always)]
    pub fn dmard_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardmis::DmardClr)
    }
    #[doc = "The read uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn dmard(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardmis::Dmard)
    }
}
#[doc = "Write uDMA Masked Interrupt Status.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmawrmis {
    #[doc = "0: The write uDMA has not completed or the interrupt is masked."]
    DmawrClr = 0,
    #[doc = "1: The write uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    Dmawr = 1,
}
impl From<Dmawrmis> for bool {
    #[inline(always)]
    fn from(variant: Dmawrmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWRMIS` reader - Write uDMA Masked Interrupt Status."]
pub type DmawrmisR = crate::BitReader<Dmawrmis>;
impl DmawrmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmawrmis {
        match self.bits {
            false => Dmawrmis::DmawrClr,
            true => Dmawrmis::Dmawr,
        }
    }
    #[doc = "The write uDMA has not completed or the interrupt is masked."]
    #[inline(always)]
    pub fn is_dmawr_clr(&self) -> bool {
        *self == Dmawrmis::DmawrClr
    }
    #[doc = "The write uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_dmawr(&self) -> bool {
        *self == Dmawrmis::Dmawr
    }
}
#[doc = "Field `DMAWRMIS` writer - Write uDMA Masked Interrupt Status."]
pub type DmawrmisW<'a, REG> = crate::BitWriter<'a, REG, Dmawrmis>;
impl<'a, REG> DmawrmisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write uDMA has not completed or the interrupt is masked."]
    #[inline(always)]
    pub fn dmawr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrmis::DmawrClr)
    }
    #[doc = "The write uDMA has completed and the DMAWRIM bit in the EPIIM register is set, triggering an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn dmawr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrmis::Dmawr)
    }
}
impl R {
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn errmis(&self) -> ErrmisR {
        ErrmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn rdmis(&self) -> RdmisR {
        RdmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn wrmis(&self) -> WrmisR {
        WrmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status."]
    #[inline(always)]
    pub fn dmardmis(&self) -> DmardmisR {
        DmardmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status."]
    #[inline(always)]
    pub fn dmawrmis(&self) -> DmawrmisR {
        DmawrmisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("errmis", &self.errmis())
            .field("rdmis", &self.rdmis())
            .field("wrmis", &self.wrmis())
            .field("dmardmis", &self.dmardmis())
            .field("dmawrmis", &self.dmawrmis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn errmis(&mut self) -> ErrmisW<'_, MisSpec> {
        ErrmisW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn rdmis(&mut self) -> RdmisW<'_, MisSpec> {
        RdmisW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn wrmis(&mut self) -> WrmisW<'_, MisSpec> {
        WrmisW::new(self, 2)
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status."]
    #[inline(always)]
    pub fn dmardmis(&mut self) -> DmardmisW<'_, MisSpec> {
        DmardmisW::new(self, 3)
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status."]
    #[inline(always)]
    pub fn dmawrmis(&mut self) -> DmawrmisW<'_, MisSpec> {
        DmawrmisW::new(self, 4)
    }
}
#[doc = "EPI Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
