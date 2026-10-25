#[doc = "Register `FIFOLVL` reader"]
pub type R = crate::R<FifolvlSpec>;
#[doc = "Register `FIFOLVL` writer"]
pub type W = crate::W<FifolvlSpec>;
#[doc = "Read FIFO\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdfifo {
    #[doc = "1: Trigger when there are 1 or more entries in the NBRFIFO."]
    Entries1 = 1,
    #[doc = "2: Trigger when there are 2 or more entries in the NBRFIFO."]
    Entries2 = 2,
    #[doc = "3: Trigger when there are 4 or more entries in the NBRFIFO."]
    Entries4 = 3,
    #[doc = "4: Trigger when there are 6 or more entries in the NBRFIFO."]
    Entries6 = 4,
    #[doc = "5: Trigger when there are 7 or more entries in the NBRFIFO."]
    Entries7 = 5,
    #[doc = "6: Trigger when there are 8 entries in the NBRFIFO."]
    Entries8 = 6,
}
impl From<Rdfifo> for u8 {
    #[inline(always)]
    fn from(variant: Rdfifo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdfifo {
    type Ux = u8;
}
impl crate::IsEnum for Rdfifo {}
#[doc = "Field `RDFIFO` reader - Read FIFO"]
pub type RdfifoR = crate::FieldReader<Rdfifo>;
impl RdfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rdfifo> {
        match self.bits {
            1 => Some(Rdfifo::Entries1),
            2 => Some(Rdfifo::Entries2),
            3 => Some(Rdfifo::Entries4),
            4 => Some(Rdfifo::Entries6),
            5 => Some(Rdfifo::Entries7),
            6 => Some(Rdfifo::Entries8),
            _ => None,
        }
    }
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_1(&self) -> bool {
        *self == Rdfifo::Entries1
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_2(&self) -> bool {
        *self == Rdfifo::Entries2
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_4(&self) -> bool {
        *self == Rdfifo::Entries4
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_6(&self) -> bool {
        *self == Rdfifo::Entries6
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_7(&self) -> bool {
        *self == Rdfifo::Entries7
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO."]
    #[inline(always)]
    pub fn is_entries_8(&self) -> bool {
        *self == Rdfifo::Entries8
    }
}
#[doc = "Field `RDFIFO` writer - Read FIFO"]
pub type RdfifoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rdfifo, crate::Safe>;
impl<'a, REG> RdfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries1)
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries2)
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries4)
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries6)
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries7)
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO."]
    #[inline(always)]
    pub fn entries_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfifo::Entries8)
    }
}
#[doc = "Write FIFO\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrfifo {
    #[doc = "0: Interrupt is triggered while WRFIFO is empty. It will be deasserted when not empty. This encoding is optimized for burst of 4 writes."]
    Empty = 0,
    #[doc = "2: Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2."]
    Entries2 = 2,
    #[doc = "3: Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes."]
    Entries1 = 3,
    #[doc = "4: Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO."]
    NotFull = 4,
}
impl From<Wrfifo> for u8 {
    #[inline(always)]
    fn from(variant: Wrfifo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrfifo {
    type Ux = u8;
}
impl crate::IsEnum for Wrfifo {}
#[doc = "Field `WRFIFO` reader - Write FIFO"]
pub type WrfifoR = crate::FieldReader<Wrfifo>;
impl WrfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wrfifo> {
        match self.bits {
            0 => Some(Wrfifo::Empty),
            2 => Some(Wrfifo::Entries2),
            3 => Some(Wrfifo::Entries1),
            4 => Some(Wrfifo::NotFull),
            _ => None,
        }
    }
    #[doc = "Interrupt is triggered while WRFIFO is empty. It will be deasserted when not empty. This encoding is optimized for burst of 4 writes."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Wrfifo::Empty
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2."]
    #[inline(always)]
    pub fn is_entries_2(&self) -> bool {
        *self == Wrfifo::Entries2
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes."]
    #[inline(always)]
    pub fn is_entries_1(&self) -> bool {
        *self == Wrfifo::Entries1
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Wrfifo::NotFull
    }
}
#[doc = "Field `WRFIFO` writer - Write FIFO"]
pub type WrfifoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wrfifo, crate::Safe>;
impl<'a, REG> WrfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt is triggered while WRFIFO is empty. It will be deasserted when not empty. This encoding is optimized for burst of 4 writes."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Wrfifo::Empty)
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2."]
    #[inline(always)]
    pub fn entries_2(self) -> &'a mut crate::W<REG> {
        self.variant(Wrfifo::Entries2)
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes."]
    #[inline(always)]
    pub fn entries_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrfifo::Entries1)
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO."]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(Wrfifo::NotFull)
    }
}
#[doc = "Read Stall Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rserr {
    #[doc = "0: The Read Stalled error interrupt is disabled. Reads behave as normal and are stalled until any preceding writes have completed and the read has returned a result."]
    Disable = 0,
    #[doc = "1: This bit enables the Read Stalled error interrupt (RSTALL in the EPIEISC register) to be generated when a read is attempted and the WFIFO is not empty. The read is still stalled during the time the WFIFO drains, but this error notifies the application that this excess delay has occurred."]
    Enable = 1,
}
impl From<Rserr> for bool {
    #[inline(always)]
    fn from(variant: Rserr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSERR` reader - Read Stall Error"]
pub type RserrR = crate::BitReader<Rserr>;
impl RserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rserr {
        match self.bits {
            false => Rserr::Disable,
            true => Rserr::Enable,
        }
    }
    #[doc = "The Read Stalled error interrupt is disabled. Reads behave as normal and are stalled until any preceding writes have completed and the read has returned a result."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rserr::Disable
    }
    #[doc = "This bit enables the Read Stalled error interrupt (RSTALL in the EPIEISC register) to be generated when a read is attempted and the WFIFO is not empty. The read is still stalled during the time the WFIFO drains, but this error notifies the application that this excess delay has occurred."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rserr::Enable
    }
}
#[doc = "Field `RSERR` writer - Read Stall Error"]
pub type RserrW<'a, REG> = crate::BitWriter<'a, REG, Rserr>;
impl<'a, REG> RserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Read Stalled error interrupt is disabled. Reads behave as normal and are stalled until any preceding writes have completed and the read has returned a result."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rserr::Disable)
    }
    #[doc = "This bit enables the Read Stalled error interrupt (RSTALL in the EPIEISC register) to be generated when a read is attempted and the WFIFO is not empty. The read is still stalled during the time the WFIFO drains, but this error notifies the application that this excess delay has occurred."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rserr::Enable)
    }
}
#[doc = "Write Full Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wferr {
    #[doc = "0: The Write Full error interrupt is disabled. Writes are stalled when the WFIFO is full until a space becomes available but an error is not generated. Note that the Cortex-M4 write buffer may hide that stall if no other memory transactions are attempted during that time."]
    Disable = 0,
    #[doc = "1: This bit enables the Write Full error interrupt (WTFULL in the EPIEISC register) to be generated when a write is attempted and the WFIFO is full. The write stalls until a WFIFO entry becomes available."]
    Enable = 1,
}
impl From<Wferr> for bool {
    #[inline(always)]
    fn from(variant: Wferr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFERR` reader - Write Full Error"]
pub type WferrR = crate::BitReader<Wferr>;
impl WferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wferr {
        match self.bits {
            false => Wferr::Disable,
            true => Wferr::Enable,
        }
    }
    #[doc = "The Write Full error interrupt is disabled. Writes are stalled when the WFIFO is full until a space becomes available but an error is not generated. Note that the Cortex-M4 write buffer may hide that stall if no other memory transactions are attempted during that time."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wferr::Disable
    }
    #[doc = "This bit enables the Write Full error interrupt (WTFULL in the EPIEISC register) to be generated when a write is attempted and the WFIFO is full. The write stalls until a WFIFO entry becomes available."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wferr::Enable
    }
}
#[doc = "Field `WFERR` writer - Write Full Error"]
pub type WferrW<'a, REG> = crate::BitWriter<'a, REG, Wferr>;
impl<'a, REG> WferrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Write Full error interrupt is disabled. Writes are stalled when the WFIFO is full until a space becomes available but an error is not generated. Note that the Cortex-M4 write buffer may hide that stall if no other memory transactions are attempted during that time."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wferr::Disable)
    }
    #[doc = "This bit enables the Write Full error interrupt (WTFULL in the EPIEISC register) to be generated when a write is attempted and the WFIFO is full. The write stalls until a WFIFO entry becomes available."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wferr::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn rdfifo(&self) -> RdfifoR {
        RdfifoR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn wrfifo(&self) -> WrfifoR {
        WrfifoR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn rserr(&self) -> RserrR {
        RserrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn wferr(&self) -> WferrR {
        WferrR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOLVL")
            .field("rdfifo", &self.rdfifo())
            .field("wrfifo", &self.wrfifo())
            .field("rserr", &self.rserr())
            .field("wferr", &self.wferr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn rdfifo(&mut self) -> RdfifoW<'_, FifolvlSpec> {
        RdfifoW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn wrfifo(&mut self) -> WrfifoW<'_, FifolvlSpec> {
        WrfifoW::new(self, 4)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn rserr(&mut self) -> RserrW<'_, FifolvlSpec> {
        RserrW::new(self, 16)
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn wferr(&mut self) -> WferrW<'_, FifolvlSpec> {
        WferrW::new(self, 17)
    }
}
#[doc = "EPI FIFO Level Selects\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolvlSpec;
impl crate::RegisterSpec for FifolvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolvl::R`](R) reader structure"]
impl crate::Readable for FifolvlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifolvl::W`](W) writer structure"]
impl crate::Writable for FifolvlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FIFOLVL to value 0x33"]
impl crate::Resettable for FifolvlSpec {
    const RESET_VALUE: u32 = 0x33;
}
