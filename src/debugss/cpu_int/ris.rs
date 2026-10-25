#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `TXIFG` reader - Raw interrupt status for TXIFG"]
pub type TxifgR = crate::BitReader;
#[doc = "Field `RXIFG` reader - Raw interrupt status for RXIFG"]
pub type RxifgR = crate::BitReader;
#[doc = "Field `PWRUPIFG` reader - Raw interrupt status for PWRUPIFG"]
pub type PwrupifgR = crate::BitReader;
#[doc = "Field `PWRDWNIFG` reader - Raw interrupt status for PWRDWNIFG"]
pub type PwrdwnifgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw interrupt status for TXIFG"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status for RXIFG"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status for PWRDWNIFG"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("txifg", &self.txifg())
            .field("rxifg", &self.rxifg())
            .field("pwrupifg", &self.pwrupifg())
            .field("pwrdwnifg", &self.pwrdwnifg())
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
