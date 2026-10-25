#[doc = "Register `ERR_STAT1` reader"]
pub type R = crate::R<ErrStat1Spec>;
#[doc = "Register `ERR_STAT1` writer"]
pub type W = crate::W<ErrStat1Spec>;
#[doc = "Field `ECC_SEC` reader - Single Bit Error Corrected Status"]
pub type EccSecR = crate::FieldReader;
#[doc = "Field `ECC_SEC` writer - Single Bit Error Corrected Status"]
pub type EccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `ECC_DED` reader - Double Bit Error Detected Status"]
pub type EccDedR = crate::FieldReader;
#[doc = "Field `ECC_DED` writer - Double Bit Error Detected Status"]
pub type EccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `ECC_OTHER` reader - SEC While Writeback Error Status"]
pub type EccOtherR = crate::BitReader;
#[doc = "Field `ECC_OTHER` writer - SEC While Writeback Error Status"]
pub type EccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_REG_ERROR` reader - Control Register Error"]
pub type CtrlRegErrorR = crate::BitReader;
#[doc = "Field `CTRL_REG_ERROR` writer - Control Register Error"]
pub type CtrlRegErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ECC_SEC` reader - Clear ECC_SEC"]
pub type ClrEccSecR = crate::FieldReader;
#[doc = "Field `CLR_ECC_SEC` writer - Clear ECC_SEC"]
pub type ClrEccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CLR_ECC_DED` reader - Clear ECC_DED"]
pub type ClrEccDedR = crate::FieldReader;
#[doc = "Field `CLR_ECC_DED` writer - Clear ECC_DED"]
pub type ClrEccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CLR_ECC_OTHER` reader - Clear ECC_OTHER"]
pub type ClrEccOtherR = crate::BitReader;
#[doc = "Field `CLR_ECC_OTHER` writer - Clear ECC_OTHER"]
pub type ClrEccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CTRL_REG_ERROR` reader - Clear Control Register Error"]
pub type ClrCtrlRegErrorR = crate::BitReader;
#[doc = "Field `CLR_CTRL_REG_ERROR` writer - Clear Control Register Error"]
pub type ClrCtrlRegErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_BIT1` reader - ECC Error Bit Position"]
pub type EccBit1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Single Bit Error Corrected Status"]
    #[inline(always)]
    pub fn ecc_sec(&self) -> EccSecR {
        EccSecR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Double Bit Error Detected Status"]
    #[inline(always)]
    pub fn ecc_ded(&self) -> EccDedR {
        EccDedR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SEC While Writeback Error Status"]
    #[inline(always)]
    pub fn ecc_other(&self) -> EccOtherR {
        EccOtherR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Register Error"]
    #[inline(always)]
    pub fn ctrl_reg_error(&self) -> CtrlRegErrorR {
        CtrlRegErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clear ECC_SEC"]
    #[inline(always)]
    pub fn clr_ecc_sec(&self) -> ClrEccSecR {
        ClrEccSecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clear ECC_DED"]
    #[inline(always)]
    pub fn clr_ecc_ded(&self) -> ClrEccDedR {
        ClrEccDedR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Clear ECC_OTHER"]
    #[inline(always)]
    pub fn clr_ecc_other(&self) -> ClrEccOtherR {
        ClrEccOtherR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear Control Register Error"]
    #[inline(always)]
    pub fn clr_ctrl_reg_error(&self) -> ClrCtrlRegErrorR {
        ClrCtrlRegErrorR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - ECC Error Bit Position"]
    #[inline(always)]
    pub fn ecc_bit1(&self) -> EccBit1R {
        EccBit1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_STAT1")
            .field("ecc_sec", &self.ecc_sec())
            .field("ecc_ded", &self.ecc_ded())
            .field("ecc_other", &self.ecc_other())
            .field("ctrl_reg_error", &self.ctrl_reg_error())
            .field("clr_ecc_sec", &self.clr_ecc_sec())
            .field("clr_ecc_ded", &self.clr_ecc_ded())
            .field("clr_ecc_other", &self.clr_ecc_other())
            .field("clr_ctrl_reg_error", &self.clr_ctrl_reg_error())
            .field("ecc_bit1", &self.ecc_bit1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Single Bit Error Corrected Status"]
    #[inline(always)]
    pub fn ecc_sec(&mut self) -> EccSecW<'_, ErrStat1Spec> {
        EccSecW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Double Bit Error Detected Status"]
    #[inline(always)]
    pub fn ecc_ded(&mut self) -> EccDedW<'_, ErrStat1Spec> {
        EccDedW::new(self, 2)
    }
    #[doc = "Bit 4 - SEC While Writeback Error Status"]
    #[inline(always)]
    pub fn ecc_other(&mut self) -> EccOtherW<'_, ErrStat1Spec> {
        EccOtherW::new(self, 4)
    }
    #[doc = "Bit 7 - Control Register Error"]
    #[inline(always)]
    pub fn ctrl_reg_error(&mut self) -> CtrlRegErrorW<'_, ErrStat1Spec> {
        CtrlRegErrorW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clear ECC_SEC"]
    #[inline(always)]
    pub fn clr_ecc_sec(&mut self) -> ClrEccSecW<'_, ErrStat1Spec> {
        ClrEccSecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Clear ECC_DED"]
    #[inline(always)]
    pub fn clr_ecc_ded(&mut self) -> ClrEccDedW<'_, ErrStat1Spec> {
        ClrEccDedW::new(self, 10)
    }
    #[doc = "Bit 12 - Clear ECC_OTHER"]
    #[inline(always)]
    pub fn clr_ecc_other(&mut self) -> ClrEccOtherW<'_, ErrStat1Spec> {
        ClrEccOtherW::new(self, 12)
    }
    #[doc = "Bit 15 - Clear Control Register Error"]
    #[inline(always)]
    pub fn clr_ctrl_reg_error(&mut self) -> ClrCtrlRegErrorW<'_, ErrStat1Spec> {
        ClrCtrlRegErrorW::new(self, 15)
    }
}
#[doc = "MCAN ECC Error Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat1Spec;
impl crate::RegisterSpec for ErrStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat1::R`](R) reader structure"]
impl crate::Readable for ErrStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat1::W`](W) writer structure"]
impl crate::Writable for ErrStat1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ERR_STAT1 to value 0"]
impl crate::Resettable for ErrStat1Spec {}
