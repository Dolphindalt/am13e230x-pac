#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Field `ATTRIBVIOL_CONFIG` reader - Register lock configuration"]
pub type AttribviolConfigR = crate::BitReader;
#[doc = "Field `ATTRIBVIOL_CONFIG` writer - Register lock configuration"]
pub type AttribviolConfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPC_AATRIB` reader - Register lock configuration"]
pub type PpcAatribR = crate::BitReader;
#[doc = "Field `PPC_AATRIB` writer - Register lock configuration"]
pub type PpcAatribW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPC_AATRIB` reader - Register lock configuration"]
pub type SpcAatribR = crate::BitReader;
#[doc = "Field `SPC_AATRIB` writer - Register lock configuration"]
pub type SpcAatribW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPPC_AATRIB` reader - Register lock configuration"]
pub type FppcAatribR = crate::BitReader;
#[doc = "Field `FPPC_AATRIB` writer - Register lock configuration"]
pub type FppcAatribW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTOR` reader - Register lock configuration"]
pub type VtorR = crate::BitReader;
#[doc = "Field `VTOR` writer - Register lock configuration"]
pub type VtorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTB_MUXSEL` reader - Register lock configuration"]
pub type DtbMuxselR = crate::BitReader;
#[doc = "Field `DTB_MUXSEL` writer - Register lock configuration"]
pub type DtbMuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write key: the write only takes effect if KEY is 0xA551\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "42321: Write 0xA551 to update the register"]
    Unlock = 42321,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xA551"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write 0xA551 to update the register"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bit 0 - Register lock configuration"]
    #[inline(always)]
    pub fn attribviol_config(&self) -> AttribviolConfigR {
        AttribviolConfigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register lock configuration"]
    #[inline(always)]
    pub fn ppc_aatrib(&self) -> PpcAatribR {
        PpcAatribR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register lock configuration"]
    #[inline(always)]
    pub fn spc_aatrib(&self) -> SpcAatribR {
        SpcAatribR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register lock configuration"]
    #[inline(always)]
    pub fn fppc_aatrib(&self) -> FppcAatribR {
        FppcAatribR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Register lock configuration"]
    #[inline(always)]
    pub fn vtor(&self) -> VtorR {
        VtorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Register lock configuration"]
    #[inline(always)]
    pub fn dtb_muxsel(&self) -> DtbMuxselR {
        DtbMuxselR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("attribviol_config", &self.attribviol_config())
            .field("ppc_aatrib", &self.ppc_aatrib())
            .field("spc_aatrib", &self.spc_aatrib())
            .field("fppc_aatrib", &self.fppc_aatrib())
            .field("vtor", &self.vtor())
            .field("dtb_muxsel", &self.dtb_muxsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Register lock configuration"]
    #[inline(always)]
    pub fn attribviol_config(&mut self) -> AttribviolConfigW<'_, LockSpec> {
        AttribviolConfigW::new(self, 0)
    }
    #[doc = "Bit 1 - Register lock configuration"]
    #[inline(always)]
    pub fn ppc_aatrib(&mut self) -> PpcAatribW<'_, LockSpec> {
        PpcAatribW::new(self, 1)
    }
    #[doc = "Bit 2 - Register lock configuration"]
    #[inline(always)]
    pub fn spc_aatrib(&mut self) -> SpcAatribW<'_, LockSpec> {
        SpcAatribW::new(self, 2)
    }
    #[doc = "Bit 3 - Register lock configuration"]
    #[inline(always)]
    pub fn fppc_aatrib(&mut self) -> FppcAatribW<'_, LockSpec> {
        FppcAatribW::new(self, 3)
    }
    #[doc = "Bit 5 - Register lock configuration"]
    #[inline(always)]
    pub fn vtor(&mut self) -> VtorW<'_, LockSpec> {
        VtorW::new(self, 5)
    }
    #[doc = "Bit 7 - Register lock configuration"]
    #[inline(always)]
    pub fn dtb_muxsel(&mut self) -> DtbMuxselW<'_, LockSpec> {
        DtbMuxselW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Write key: the write only takes effect if KEY is 0xA551"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, LockSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "GSC Lock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
