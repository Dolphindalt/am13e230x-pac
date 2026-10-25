#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `Z` reader - Zero event generated an interrupt."]
pub type ZR = crate::BitReader;
#[doc = "Field `L` reader - Load event generated an interrupt."]
pub type LR = crate::BitReader;
#[doc = "Field `CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type Ccd0R = crate::BitReader;
#[doc = "Field `CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type Ccd1R = crate::BitReader;
#[doc = "Field `CCD2` reader - Capture or compare down event generated an interrupt CCP2"]
pub type Ccd2R = crate::BitReader;
#[doc = "Field `CCD3` reader - Capture or compare down event generated an interrupt CCP3"]
pub type Ccd3R = crate::BitReader;
#[doc = "Field `CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type Ccu0R = crate::BitReader;
#[doc = "Field `CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type Ccu1R = crate::BitReader;
#[doc = "Field `CCU2` reader - Capture or compare up event generated an interrupt CCP2"]
pub type Ccu2R = crate::BitReader;
#[doc = "Field `CCU3` reader - Capture or compare up event generated an interrupt CCP3"]
pub type Ccu3R = crate::BitReader;
#[doc = "Field `CCD4` reader - Compare down event generated an interrupt CCD4"]
pub type Ccd4R = crate::BitReader;
#[doc = "Field `CCD5` reader - Compare down event generated an interrupt CCD5"]
pub type Ccd5R = crate::BitReader;
#[doc = "Field `CCU4` reader - Compare up event generated an interrupt CCU4"]
pub type Ccu4R = crate::BitReader;
#[doc = "Field `CCU5` reader - Compare up event generated an interrupt CCP5"]
pub type Ccu5R = crate::BitReader;
#[doc = "Field `F` reader - Fault"]
pub type FR = crate::BitReader;
#[doc = "Field `TOV` reader - Trigger overflow"]
pub type TovR = crate::BitReader;
#[doc = "Field `REPC` reader - Repeat Counter Zero"]
pub type RepcR = crate::BitReader;
#[doc = "Field `DC` reader - Direction Change"]
pub type DcR = crate::BitReader;
#[doc = "Field `QEIERR` reader - QEIERR, set on an incorrect state transition on the encoder interface."]
pub type QeierrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ccd0(&self) -> Ccd0R {
        Ccd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ccd1(&self) -> Ccd1R {
        Ccd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or compare down event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ccd2(&self) -> Ccd2R {
        Ccd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or compare down event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ccd3(&self) -> Ccd3R {
        Ccd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ccu0(&self) -> Ccu0R {
        Ccu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ccu1(&self) -> Ccu1R {
        Ccu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or compare up event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ccu2(&self) -> Ccu2R {
        Ccu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or compare up event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ccu3(&self) -> Ccu3R {
        Ccu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare down event generated an interrupt CCD4"]
    #[inline(always)]
    pub fn ccd4(&self) -> Ccd4R {
        Ccd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare down event generated an interrupt CCD5"]
    #[inline(always)]
    pub fn ccd5(&self) -> Ccd5R {
        Ccd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare up event generated an interrupt CCU4"]
    #[inline(always)]
    pub fn ccu4(&self) -> Ccu4R {
        Ccu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare up event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn ccu5(&self) -> Ccu5R {
        Ccu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault"]
    #[inline(always)]
    pub fn f(&self) -> FR {
        FR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero"]
    #[inline(always)]
    pub fn repc(&self) -> RepcR {
        RepcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Direction Change"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - QEIERR, set on an incorrect state transition on the encoder interface."]
    #[inline(always)]
    pub fn qeierr(&self) -> QeierrR {
        QeierrR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("z", &self.z())
            .field("l", &self.l())
            .field("ccd0", &self.ccd0())
            .field("ccd1", &self.ccd1())
            .field("ccu0", &self.ccu0())
            .field("ccu1", &self.ccu1())
            .field("tov", &self.tov())
            .field("f", &self.f())
            .field("dc", &self.dc())
            .field("qeierr", &self.qeierr())
            .field("ccd2", &self.ccd2())
            .field("ccd3", &self.ccd3())
            .field("ccu2", &self.ccu2())
            .field("ccu3", &self.ccu3())
            .field("ccd4", &self.ccd4())
            .field("ccd5", &self.ccd5())
            .field("ccu4", &self.ccu4())
            .field("ccu5", &self.ccu5())
            .field("repc", &self.repc())
            .finish()
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
