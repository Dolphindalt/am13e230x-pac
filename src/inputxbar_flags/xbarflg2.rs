#[doc = "Register `XBARFLG2` reader"]
pub type R = crate::R<Xbarflg2Spec>;
#[doc = "Register `XBARFLG2` writer"]
pub type W = crate::W<Xbarflg2Spec>;
#[doc = "Field `INPUTXBAR13` reader - Input Flag for INPUTXBAR13 Signal"]
pub type Inputxbar13R = crate::BitReader;
#[doc = "Field `INPUTXBAR14` reader - Input Flag for INPUTXBAR14 Signal"]
pub type Inputxbar14R = crate::BitReader;
#[doc = "Field `INPUTXBAR15` reader - Input Flag for INPUTXBAR15 Signal"]
pub type Inputxbar15R = crate::BitReader;
#[doc = "Field `INPUTXBAR16` reader - Input Flag for INPUTXBAR16 Signal"]
pub type Inputxbar16R = crate::BitReader;
#[doc = "Field `ECAP0_OUT` reader - Input Flag for ECAP0_OUT Signal"]
pub type Ecap0OutR = crate::BitReader;
#[doc = "Field `ECAP1_OUT` reader - Input Flag for ECAP1_OUT Signal"]
pub type Ecap1OutR = crate::BitReader;
#[doc = "Field `ADCSOCA` reader - Input Flag for ADCSOCA Signal"]
pub type AdcsocaR = crate::BitReader;
#[doc = "Field `ADCSOCB` reader - Input Flag for ADCSOCB Signal"]
pub type AdcsocbR = crate::BitReader;
#[doc = "Field `EXTSYNCOUT` reader - Input Flag for EXTSYNCOUT Signal"]
pub type ExtsyncoutR = crate::BitReader;
#[doc = "Field `MCAN_INT0` reader - Input Flag for MCAN_INT0 Signal"]
pub type McanInt0R = crate::BitReader;
#[doc = "Field `EQEP0_ERR` reader - Input Flag for EQEP0_ERR Signal"]
pub type Eqep0ErrR = crate::BitReader;
#[doc = "Field `EQEP1_ERR` reader - Input Flag for EQEP1_ERR Signal"]
pub type Eqep1ErrR = crate::BitReader;
#[doc = "Field `EQEP2_ERR` reader - Input Flag for EQEP2_ERR Signal"]
pub type Eqep2ErrR = crate::BitReader;
#[doc = "Field `EQEP0_INDEX_SYNCOUT` reader - Input Flag for EQEP0_INDEX_SYNCOUT Signal"]
pub type Eqep0IndexSyncoutR = crate::BitReader;
#[doc = "Field `EQEP0_STROBE_SYNCOUT` reader - Input Flag for EQEP0_STROBE_SYNCOUT Signal"]
pub type Eqep0StrobeSyncoutR = crate::BitReader;
#[doc = "Field `EQEP1_INDEX_SYNCOUT` reader - Input Flag for EQEP1_INDEX_SYNCOUT Signal"]
pub type Eqep1IndexSyncoutR = crate::BitReader;
#[doc = "Field `EQEP1_STROBE_SYNCOUT` reader - Input Flag for EQEP1_STROBE_SYNCOUT Signal"]
pub type Eqep1StrobeSyncoutR = crate::BitReader;
#[doc = "Field `EQEP2_INDEX_SYNCOUT` reader - Input Flag for EQEP2_INDEX_SYNCOUT Signal"]
pub type Eqep2IndexSyncoutR = crate::BitReader;
#[doc = "Field `EQEP2_STROBE_SYNCOUT` reader - Input Flag for EQEP2_STROBE_SYNCOUT Signal"]
pub type Eqep2StrobeSyncoutR = crate::BitReader;
#[doc = "Field `CMPSS0_CTRIPOUTH` reader - Input Flag for CMPSS0_CTRIPOUTH Signal"]
pub type Cmpss0CtripouthR = crate::BitReader;
#[doc = "Field `CMPSS0_CTRIPOUTL` reader - Input Flag for CMPSS0_CTRIPOUTL Signal"]
pub type Cmpss0CtripoutlR = crate::BitReader;
#[doc = "Field `CMPSS1_CTRIPOUTH` reader - Input Flag for CMPSS1_CTRIPOUTH Signal"]
pub type Cmpss1CtripouthR = crate::BitReader;
#[doc = "Field `CMPSS1_CTRIPOUTL` reader - Input Flag for CMPSS1_CTRIPOUTL Signal"]
pub type Cmpss1CtripoutlR = crate::BitReader;
#[doc = "Field `CMPSS2_CTRIPOUTH` reader - Input Flag for CMPSS2_CTRIPOUTH Signal"]
pub type Cmpss2CtripouthR = crate::BitReader;
#[doc = "Field `CMPSS2_CTRIPOUTL` reader - Input Flag for CMPSS2_CTRIPOUTL Signal"]
pub type Cmpss2CtripoutlR = crate::BitReader;
#[doc = "Field `CMPSS3_CTRIPOUTH` reader - Input Flag for CMPSS3_CTRIPOUTH Signal"]
pub type Cmpss3CtripouthR = crate::BitReader;
#[doc = "Field `CMPSS3_CTRIPOUTL` reader - Input Flag for CMPSS3_CTRIPOUTL Signal"]
pub type Cmpss3CtripoutlR = crate::BitReader;
#[doc = "Field `CPU_HALTED` reader - Input Flag for CPU_HALTED Signal"]
pub type CpuHaltedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input Flag for INPUTXBAR13 Signal"]
    #[inline(always)]
    pub fn inputxbar13(&self) -> Inputxbar13R {
        Inputxbar13R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Flag for INPUTXBAR14 Signal"]
    #[inline(always)]
    pub fn inputxbar14(&self) -> Inputxbar14R {
        Inputxbar14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Flag for INPUTXBAR15 Signal"]
    #[inline(always)]
    pub fn inputxbar15(&self) -> Inputxbar15R {
        Inputxbar15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Flag for INPUTXBAR16 Signal"]
    #[inline(always)]
    pub fn inputxbar16(&self) -> Inputxbar16R {
        Inputxbar16R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Flag for ECAP0_OUT Signal"]
    #[inline(always)]
    pub fn ecap0_out(&self) -> Ecap0OutR {
        Ecap0OutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Flag for ECAP1_OUT Signal"]
    #[inline(always)]
    pub fn ecap1_out(&self) -> Ecap1OutR {
        Ecap1OutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Flag for ADCSOCA Signal"]
    #[inline(always)]
    pub fn adcsoca(&self) -> AdcsocaR {
        AdcsocaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Flag for ADCSOCB Signal"]
    #[inline(always)]
    pub fn adcsocb(&self) -> AdcsocbR {
        AdcsocbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input Flag for EXTSYNCOUT Signal"]
    #[inline(always)]
    pub fn extsyncout(&self) -> ExtsyncoutR {
        ExtsyncoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input Flag for MCAN_INT0 Signal"]
    #[inline(always)]
    pub fn mcan_int0(&self) -> McanInt0R {
        McanInt0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Input Flag for EQEP0_ERR Signal"]
    #[inline(always)]
    pub fn eqep0_err(&self) -> Eqep0ErrR {
        Eqep0ErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input Flag for EQEP1_ERR Signal"]
    #[inline(always)]
    pub fn eqep1_err(&self) -> Eqep1ErrR {
        Eqep1ErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input Flag for EQEP2_ERR Signal"]
    #[inline(always)]
    pub fn eqep2_err(&self) -> Eqep2ErrR {
        Eqep2ErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input Flag for EQEP0_INDEX_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep0_index_syncout(&self) -> Eqep0IndexSyncoutR {
        Eqep0IndexSyncoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input Flag for EQEP0_STROBE_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep0_strobe_syncout(&self) -> Eqep0StrobeSyncoutR {
        Eqep0StrobeSyncoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input Flag for EQEP1_INDEX_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep1_index_syncout(&self) -> Eqep1IndexSyncoutR {
        Eqep1IndexSyncoutR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Flag for EQEP1_STROBE_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep1_strobe_syncout(&self) -> Eqep1StrobeSyncoutR {
        Eqep1StrobeSyncoutR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input Flag for EQEP2_INDEX_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep2_index_syncout(&self) -> Eqep2IndexSyncoutR {
        Eqep2IndexSyncoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input Flag for EQEP2_STROBE_SYNCOUT Signal"]
    #[inline(always)]
    pub fn eqep2_strobe_syncout(&self) -> Eqep2StrobeSyncoutR {
        Eqep2StrobeSyncoutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Flag for CMPSS0_CTRIPOUTH Signal"]
    #[inline(always)]
    pub fn cmpss0_ctripouth(&self) -> Cmpss0CtripouthR {
        Cmpss0CtripouthR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input Flag for CMPSS0_CTRIPOUTL Signal"]
    #[inline(always)]
    pub fn cmpss0_ctripoutl(&self) -> Cmpss0CtripoutlR {
        Cmpss0CtripoutlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input Flag for CMPSS1_CTRIPOUTH Signal"]
    #[inline(always)]
    pub fn cmpss1_ctripouth(&self) -> Cmpss1CtripouthR {
        Cmpss1CtripouthR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input Flag for CMPSS1_CTRIPOUTL Signal"]
    #[inline(always)]
    pub fn cmpss1_ctripoutl(&self) -> Cmpss1CtripoutlR {
        Cmpss1CtripoutlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input Flag for CMPSS2_CTRIPOUTH Signal"]
    #[inline(always)]
    pub fn cmpss2_ctripouth(&self) -> Cmpss2CtripouthR {
        Cmpss2CtripouthR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input Flag for CMPSS2_CTRIPOUTL Signal"]
    #[inline(always)]
    pub fn cmpss2_ctripoutl(&self) -> Cmpss2CtripoutlR {
        Cmpss2CtripoutlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input Flag for CMPSS3_CTRIPOUTH Signal"]
    #[inline(always)]
    pub fn cmpss3_ctripouth(&self) -> Cmpss3CtripouthR {
        Cmpss3CtripouthR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input Flag for CMPSS3_CTRIPOUTL Signal"]
    #[inline(always)]
    pub fn cmpss3_ctripoutl(&self) -> Cmpss3CtripoutlR {
        Cmpss3CtripoutlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input Flag for CPU_HALTED Signal"]
    #[inline(always)]
    pub fn cpu_halted(&self) -> CpuHaltedR {
        CpuHaltedR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XBARFLG2")
            .field("inputxbar13", &self.inputxbar13())
            .field("inputxbar14", &self.inputxbar14())
            .field("inputxbar15", &self.inputxbar15())
            .field("inputxbar16", &self.inputxbar16())
            .field("ecap0_out", &self.ecap0_out())
            .field("ecap1_out", &self.ecap1_out())
            .field("adcsoca", &self.adcsoca())
            .field("adcsocb", &self.adcsocb())
            .field("extsyncout", &self.extsyncout())
            .field("mcan_int0", &self.mcan_int0())
            .field("eqep0_err", &self.eqep0_err())
            .field("eqep1_err", &self.eqep1_err())
            .field("eqep2_err", &self.eqep2_err())
            .field("eqep0_index_syncout", &self.eqep0_index_syncout())
            .field("eqep0_strobe_syncout", &self.eqep0_strobe_syncout())
            .field("eqep1_index_syncout", &self.eqep1_index_syncout())
            .field("eqep1_strobe_syncout", &self.eqep1_strobe_syncout())
            .field("eqep2_index_syncout", &self.eqep2_index_syncout())
            .field("eqep2_strobe_syncout", &self.eqep2_strobe_syncout())
            .field("cmpss0_ctripouth", &self.cmpss0_ctripouth())
            .field("cmpss0_ctripoutl", &self.cmpss0_ctripoutl())
            .field("cmpss1_ctripouth", &self.cmpss1_ctripouth())
            .field("cmpss1_ctripoutl", &self.cmpss1_ctripoutl())
            .field("cmpss2_ctripouth", &self.cmpss2_ctripouth())
            .field("cmpss2_ctripoutl", &self.cmpss2_ctripoutl())
            .field("cmpss3_ctripouth", &self.cmpss3_ctripouth())
            .field("cmpss3_ctripoutl", &self.cmpss3_ctripoutl())
            .field("cpu_halted", &self.cpu_halted())
            .finish()
    }
}
impl W {}
#[doc = "X-Bar Input Flag Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarflg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarflg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xbarflg2Spec;
impl crate::RegisterSpec for Xbarflg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xbarflg2::R`](R) reader structure"]
impl crate::Readable for Xbarflg2Spec {}
#[doc = "`write(|w| ..)` method takes [`xbarflg2::W`](W) writer structure"]
impl crate::Writable for Xbarflg2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets XBARFLG2 to value 0"]
impl crate::Resettable for Xbarflg2Spec {}
