#[doc = "Register `EISC` reader"]
pub type R = crate::R<EiscSpec>;
#[doc = "Register `EISC` writer"]
pub type W = crate::W<EiscSpec>;
#[doc = "Timeout Error.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tout {
    #[doc = "0: No timeout error has occurred."]
    ToutClear = 0,
    #[doc = "1: A timeout error has occurred."]
    Tout = 1,
}
impl From<Tout> for bool {
    #[inline(always)]
    fn from(variant: Tout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUT` reader - Timeout Error."]
pub type ToutR = crate::BitReader<Tout>;
impl ToutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tout {
        match self.bits {
            false => Tout::ToutClear,
            true => Tout::Tout,
        }
    }
    #[doc = "No timeout error has occurred."]
    #[inline(always)]
    pub fn is_tout_clear(&self) -> bool {
        *self == Tout::ToutClear
    }
    #[doc = "A timeout error has occurred."]
    #[inline(always)]
    pub fn is_tout(&self) -> bool {
        *self == Tout::Tout
    }
}
#[doc = "Field `TOUT` writer - Timeout Error."]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG, Tout>;
impl<'a, REG> ToutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timeout error has occurred."]
    #[inline(always)]
    pub fn tout_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tout::ToutClear)
    }
    #[doc = "A timeout error has occurred."]
    #[inline(always)]
    pub fn tout(self) -> &'a mut crate::W<REG> {
        self.variant(Tout::Tout)
    }
}
#[doc = "Read Stalled Error.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstall {
    #[doc = "0: The RSERR bit is not enabled or no pending reads are stalled."]
    RstallClear = 0,
    #[doc = "1: The RSERR bit is enabled and a pending read is stalled due to writes in the WFIFO."]
    Rstall = 1,
}
impl From<Rstall> for bool {
    #[inline(always)]
    fn from(variant: Rstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTALL` reader - Read Stalled Error."]
pub type RstallR = crate::BitReader<Rstall>;
impl RstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstall {
        match self.bits {
            false => Rstall::RstallClear,
            true => Rstall::Rstall,
        }
    }
    #[doc = "The RSERR bit is not enabled or no pending reads are stalled."]
    #[inline(always)]
    pub fn is_rstall_clear(&self) -> bool {
        *self == Rstall::RstallClear
    }
    #[doc = "The RSERR bit is enabled and a pending read is stalled due to writes in the WFIFO."]
    #[inline(always)]
    pub fn is_rstall(&self) -> bool {
        *self == Rstall::Rstall
    }
}
#[doc = "Field `RSTALL` writer - Read Stalled Error."]
pub type RstallW<'a, REG> = crate::BitWriter<'a, REG, Rstall>;
impl<'a, REG> RstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RSERR bit is not enabled or no pending reads are stalled."]
    #[inline(always)]
    pub fn rstall_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rstall::RstallClear)
    }
    #[doc = "The RSERR bit is enabled and a pending read is stalled due to writes in the WFIFO."]
    #[inline(always)]
    pub fn rstall(self) -> &'a mut crate::W<REG> {
        self.variant(Rstall::Rstall)
    }
}
#[doc = "Write FIFO Full Error.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtfull {
    #[doc = "0: The WFERR bit is not enabled or no writes are stalled."]
    WtfullClear = 0,
    #[doc = "1: The WFERR bit is enabled and a write is stalled due to the WFIFO being full."]
    Wtfull = 1,
}
impl From<Wtfull> for bool {
    #[inline(always)]
    fn from(variant: Wtfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFULL` reader - Write FIFO Full Error."]
pub type WtfullR = crate::BitReader<Wtfull>;
impl WtfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtfull {
        match self.bits {
            false => Wtfull::WtfullClear,
            true => Wtfull::Wtfull,
        }
    }
    #[doc = "The WFERR bit is not enabled or no writes are stalled."]
    #[inline(always)]
    pub fn is_wtfull_clear(&self) -> bool {
        *self == Wtfull::WtfullClear
    }
    #[doc = "The WFERR bit is enabled and a write is stalled due to the WFIFO being full."]
    #[inline(always)]
    pub fn is_wtfull(&self) -> bool {
        *self == Wtfull::Wtfull
    }
}
#[doc = "Field `WTFULL` writer - Write FIFO Full Error."]
pub type WtfullW<'a, REG> = crate::BitWriter<'a, REG, Wtfull>;
impl<'a, REG> WtfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WFERR bit is not enabled or no writes are stalled."]
    #[inline(always)]
    pub fn wtfull_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfull::WtfullClear)
    }
    #[doc = "The WFERR bit is enabled and a write is stalled due to the WFIFO being full."]
    #[inline(always)]
    pub fn wtfull(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfull::Wtfull)
    }
}
#[doc = "Field `DMARDIC` reader - Read uDMA Interrupt Clear."]
pub type DmardicR = crate::BitReader;
#[doc = "Field `DMARDIC` writer - Read uDMA Interrupt Clear."]
pub type DmardicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWRIC` reader - Write uDMA Interrupt Clear."]
pub type DmawricR = crate::BitReader;
#[doc = "Field `DMAWRIC` writer - Write uDMA Interrupt Clear."]
pub type DmawricW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timeout Error."]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Stalled Error."]
    #[inline(always)]
    pub fn rstall(&self) -> RstallR {
        RstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Full Error."]
    #[inline(always)]
    pub fn wtfull(&self) -> WtfullR {
        WtfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear."]
    #[inline(always)]
    pub fn dmardic(&self) -> DmardicR {
        DmardicR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear."]
    #[inline(always)]
    pub fn dmawric(&self) -> DmawricR {
        DmawricR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EISC")
            .field("tout", &self.tout())
            .field("rstall", &self.rstall())
            .field("wtfull", &self.wtfull())
            .field("dmardic", &self.dmardic())
            .field("dmawric", &self.dmawric())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Error."]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<'_, EiscSpec> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Stalled Error."]
    #[inline(always)]
    pub fn rstall(&mut self) -> RstallW<'_, EiscSpec> {
        RstallW::new(self, 1)
    }
    #[doc = "Bit 2 - Write FIFO Full Error."]
    #[inline(always)]
    pub fn wtfull(&mut self) -> WtfullW<'_, EiscSpec> {
        WtfullW::new(self, 2)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear."]
    #[inline(always)]
    pub fn dmardic(&mut self) -> DmardicW<'_, EiscSpec> {
        DmardicW::new(self, 3)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear."]
    #[inline(always)]
    pub fn dmawric(&mut self) -> DmawricW<'_, EiscSpec> {
        DmawricW::new(self, 4)
    }
}
#[doc = "EPI Error and Interrupt Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`eisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EiscSpec;
impl crate::RegisterSpec for EiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eisc::R`](R) reader structure"]
impl crate::Readable for EiscSpec {}
#[doc = "`write(|w| ..)` method takes [`eisc::W`](W) writer structure"]
impl crate::Writable for EiscSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EISC to value 0"]
impl crate::Resettable for EiscSpec {}
