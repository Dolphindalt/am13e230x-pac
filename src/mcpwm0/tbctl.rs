#[doc = "Register `TBCTL` reader"]
pub type R = crate::R<TbctlSpec>;
#[doc = "Register `TBCTL` writer"]
pub type W = crate::W<TbctlSpec>;
#[doc = "Field `CTRMODE` reader - Counter Mode"]
pub type CtrmodeR = crate::FieldReader;
#[doc = "Field `CTRMODE` writer - Counter Mode"]
pub type CtrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CLKDIV` reader - Time Base Clock Pre-scaler"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Time Base Clock Pre-scaler"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `PRDLD` reader - Shadow to active load of TBPRD"]
pub type PrdldR = crate::BitReader;
#[doc = "Field `PRDLD` writer - Shadow to active load of TBPRD"]
pub type PrdldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHSEN` reader - Phase Load Enable"]
pub type PhsenR = crate::BitReader;
#[doc = "Field `PHSEN` writer - Phase Load Enable"]
pub type PhsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHSDIR` reader - Phase Direction Bit"]
pub type PhsdirR = crate::BitReader;
#[doc = "Field `PHSDIR` writer - Phase Direction Bit"]
pub type PhsdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYNC` reader - Software Force Sync Pulse"]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Software Force Sync Pulse"]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOSEL` reader - Sync Output Select"]
pub type SyncoselR = crate::FieldReader;
#[doc = "Field `SYNCOSEL` writer - Sync Output Select"]
pub type SyncoselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `FREE_SOFT` reader - Emulation Mode Bits"]
pub type FreeSoftR = crate::FieldReader;
#[doc = "Field `FREE_SOFT` writer - Emulation Mode Bits"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `SYNCPERSEL` reader - Sync peripheral Select"]
pub type SyncperselR = crate::FieldReader;
#[doc = "Field `SYNCPERSEL` writer - Sync peripheral Select"]
pub type SyncperselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SYNCISEL` reader - Sync Input Select"]
pub type SynciselR = crate::FieldReader;
#[doc = "Field `SYNCISEL` writer - Sync Input Select"]
pub type SynciselW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Counter Mode"]
    #[inline(always)]
    pub fn ctrmode(&self) -> CtrmodeR {
        CtrmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Time Base Clock Pre-scaler"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Shadow to active load of TBPRD"]
    #[inline(always)]
    pub fn prdld(&self) -> PrdldR {
        PrdldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Phase Load Enable"]
    #[inline(always)]
    pub fn phsen(&self) -> PhsenR {
        PhsenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Phase Direction Bit"]
    #[inline(always)]
    pub fn phsdir(&self) -> PhsdirR {
        PhsdirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Force Sync Pulse"]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Sync Output Select"]
    #[inline(always)]
    pub fn syncosel(&self) -> SyncoselR {
        SyncoselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Emulation Mode Bits"]
    #[inline(always)]
    pub fn free_soft(&self) -> FreeSoftR {
        FreeSoftR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:24 - Sync peripheral Select"]
    #[inline(always)]
    pub fn syncpersel(&self) -> SyncperselR {
        SyncperselR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:29 - Sync Input Select"]
    #[inline(always)]
    pub fn syncisel(&self) -> SynciselR {
        SynciselR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBCTL")
            .field("ctrmode", &self.ctrmode())
            .field("clkdiv", &self.clkdiv())
            .field("prdld", &self.prdld())
            .field("phsen", &self.phsen())
            .field("phsdir", &self.phsdir())
            .field("swsync", &self.swsync())
            .field("syncosel", &self.syncosel())
            .field("free_soft", &self.free_soft())
            .field("syncpersel", &self.syncpersel())
            .field("syncisel", &self.syncisel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter Mode"]
    #[inline(always)]
    pub fn ctrmode(&mut self) -> CtrmodeW<'_, TbctlSpec> {
        CtrmodeW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Time Base Clock Pre-scaler"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, TbctlSpec> {
        ClkdivW::new(self, 2)
    }
    #[doc = "Bit 8 - Shadow to active load of TBPRD"]
    #[inline(always)]
    pub fn prdld(&mut self) -> PrdldW<'_, TbctlSpec> {
        PrdldW::new(self, 8)
    }
    #[doc = "Bit 10 - Phase Load Enable"]
    #[inline(always)]
    pub fn phsen(&mut self) -> PhsenW<'_, TbctlSpec> {
        PhsenW::new(self, 10)
    }
    #[doc = "Bit 11 - Phase Direction Bit"]
    #[inline(always)]
    pub fn phsdir(&mut self) -> PhsdirW<'_, TbctlSpec> {
        PhsdirW::new(self, 11)
    }
    #[doc = "Bit 15 - Software Force Sync Pulse"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SwsyncW<'_, TbctlSpec> {
        SwsyncW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Sync Output Select"]
    #[inline(always)]
    pub fn syncosel(&mut self) -> SyncoselW<'_, TbctlSpec> {
        SyncoselW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Emulation Mode Bits"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FreeSoftW<'_, TbctlSpec> {
        FreeSoftW::new(self, 20)
    }
    #[doc = "Bits 22:24 - Sync peripheral Select"]
    #[inline(always)]
    pub fn syncpersel(&mut self) -> SyncperselW<'_, TbctlSpec> {
        SyncperselW::new(self, 22)
    }
    #[doc = "Bits 25:29 - Sync Input Select"]
    #[inline(always)]
    pub fn syncisel(&mut self) -> SynciselW<'_, TbctlSpec> {
        SynciselW::new(self, 25)
    }
}
#[doc = "Time base control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctlSpec;
impl crate::RegisterSpec for TbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctl::R`](R) reader structure"]
impl crate::Readable for TbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctl::W`](W) writer structure"]
impl crate::Writable for TbctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBCTL to value 0x02"]
impl crate::Resettable for TbctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
