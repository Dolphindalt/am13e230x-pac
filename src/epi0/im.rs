#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Error Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errim {
    #[doc = "0: ERRIS in the EPIRIS register is masked and does not cause an interrupt."]
    ErrMask = 0,
    #[doc = "1: ERRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    Err = 1,
}
impl From<Errim> for bool {
    #[inline(always)]
    fn from(variant: Errim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIM` reader - Error Interrupt Mask"]
pub type ErrimR = crate::BitReader<Errim>;
impl ErrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errim {
        match self.bits {
            false => Errim::ErrMask,
            true => Errim::Err,
        }
    }
    #[doc = "ERRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn is_err_mask(&self) -> bool {
        *self == Errim::ErrMask
    }
    #[doc = "ERRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Errim::Err
    }
}
#[doc = "Field `ERRIM` writer - Error Interrupt Mask"]
pub type ErrimW<'a, REG> = crate::BitWriter<'a, REG, Errim>;
impl<'a, REG> ErrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn err_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Errim::ErrMask)
    }
    #[doc = "ERRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Errim::Err)
    }
}
#[doc = "Read FIFO Full Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdim {
    #[doc = "0: RDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    RdMask = 0,
    #[doc = "1: RDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    Rd = 1,
}
impl From<Rdim> for bool {
    #[inline(always)]
    fn from(variant: Rdim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIM` reader - Read FIFO Full Interrupt Mask"]
pub type RdimR = crate::BitReader<Rdim>;
impl RdimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdim {
        match self.bits {
            false => Rdim::RdMask,
            true => Rdim::Rd,
        }
    }
    #[doc = "RDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn is_rd_mask(&self) -> bool {
        *self == Rdim::RdMask
    }
    #[doc = "RDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_rd(&self) -> bool {
        *self == Rdim::Rd
    }
}
#[doc = "Field `RDIM` writer - Read FIFO Full Interrupt Mask"]
pub type RdimW<'a, REG> = crate::BitWriter<'a, REG, Rdim>;
impl<'a, REG> RdimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn rd_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Rdim::RdMask)
    }
    #[doc = "RDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn rd(self) -> &'a mut crate::W<REG> {
        self.variant(Rdim::Rd)
    }
}
#[doc = "Write FIFO Empty Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrim {
    #[doc = "0: WRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    WrMask = 0,
    #[doc = "1: WRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    Wr = 1,
}
impl From<Wrim> for bool {
    #[inline(always)]
    fn from(variant: Wrim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRIM` reader - Write FIFO Empty Interrupt Mask"]
pub type WrimR = crate::BitReader<Wrim>;
impl WrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrim {
        match self.bits {
            false => Wrim::WrMask,
            true => Wrim::Wr,
        }
    }
    #[doc = "WRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn is_wr_mask(&self) -> bool {
        *self == Wrim::WrMask
    }
    #[doc = "WRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_wr(&self) -> bool {
        *self == Wrim::Wr
    }
}
#[doc = "Field `WRIM` writer - Write FIFO Empty Interrupt Mask"]
pub type WrimW<'a, REG> = crate::BitWriter<'a, REG, Wrim>;
impl<'a, REG> WrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn wr_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Wrim::WrMask)
    }
    #[doc = "WRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn wr(self) -> &'a mut crate::W<REG> {
        self.variant(Wrim::Wr)
    }
}
#[doc = "Read uDMA Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmardim {
    #[doc = "0: DMARDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    DmardMask = 0,
    #[doc = "1: DMARDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    Dmard = 1,
}
impl From<Dmardim> for bool {
    #[inline(always)]
    fn from(variant: Dmardim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARDIM` reader - Read uDMA Interrupt Mask"]
pub type DmardimR = crate::BitReader<Dmardim>;
impl DmardimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmardim {
        match self.bits {
            false => Dmardim::DmardMask,
            true => Dmardim::Dmard,
        }
    }
    #[doc = "DMARDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn is_dmard_mask(&self) -> bool {
        *self == Dmardim::DmardMask
    }
    #[doc = "DMARDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_dmard(&self) -> bool {
        *self == Dmardim::Dmard
    }
}
#[doc = "Field `DMARDIM` writer - Read uDMA Interrupt Mask"]
pub type DmardimW<'a, REG> = crate::BitWriter<'a, REG, Dmardim>;
impl<'a, REG> DmardimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMARDRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn dmard_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardim::DmardMask)
    }
    #[doc = "DMARDRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn dmard(self) -> &'a mut crate::W<REG> {
        self.variant(Dmardim::Dmard)
    }
}
#[doc = "Write uDMA Interrupt Mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmawrim {
    #[doc = "0: DMAWRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    DmawrMask = 0,
    #[doc = "1: DMAWRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    Dmawr = 1,
}
impl From<Dmawrim> for bool {
    #[inline(always)]
    fn from(variant: Dmawrim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWRIM` reader - Write uDMA Interrupt Mask"]
pub type DmawrimR = crate::BitReader<Dmawrim>;
impl DmawrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmawrim {
        match self.bits {
            false => Dmawrim::DmawrMask,
            true => Dmawrim::Dmawr,
        }
    }
    #[doc = "DMAWRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn is_dmawr_mask(&self) -> bool {
        *self == Dmawrim::DmawrMask
    }
    #[doc = "DMAWRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn is_dmawr(&self) -> bool {
        *self == Dmawrim::Dmawr
    }
}
#[doc = "Field `DMAWRIM` writer - Write uDMA Interrupt Mask"]
pub type DmawrimW<'a, REG> = crate::BitWriter<'a, REG, Dmawrim>;
impl<'a, REG> DmawrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAWRRIS in the EPIRIS register is masked and does not cause an interrupt."]
    #[inline(always)]
    pub fn dmawr_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrim::DmawrMask)
    }
    #[doc = "DMAWRRIS in the EPIRIS register is not masked and can trigger an interrupt to the interrupt controller."]
    #[inline(always)]
    pub fn dmawr(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawrim::Dmawr)
    }
}
impl R {
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errim(&self) -> ErrimR {
        ErrimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdim(&self) -> RdimR {
        RdimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn wrim(&self) -> WrimR {
        WrimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmardim(&self) -> DmardimR {
        DmardimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmawrim(&self) -> DmawrimR {
        DmawrimR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IM")
            .field("errim", &self.errim())
            .field("rdim", &self.rdim())
            .field("wrim", &self.wrim())
            .field("dmardim", &self.dmardim())
            .field("dmawrim", &self.dmawrim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errim(&mut self) -> ErrimW<'_, ImSpec> {
        ErrimW::new(self, 0)
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdim(&mut self) -> RdimW<'_, ImSpec> {
        RdimW::new(self, 1)
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn wrim(&mut self) -> WrimW<'_, ImSpec> {
        WrimW::new(self, 2)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmardim(&mut self) -> DmardimW<'_, ImSpec> {
        DmardimW::new(self, 3)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn dmawrim(&mut self) -> DmawrimW<'_, ImSpec> {
        DmawrimW::new(self, 4)
    }
}
#[doc = "EPI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {}
