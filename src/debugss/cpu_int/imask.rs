#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `TXIFG` reader - Masks TXIFG in MIS register"]
pub type TxifgR = crate::BitReader;
#[doc = "Field `TXIFG` writer - Masks TXIFG in MIS register"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIFG` reader - Masks RXIFG in MIS register"]
pub type RxifgR = crate::BitReader;
#[doc = "Field `RXIFG` writer - Masks RXIFG in MIS register"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPIFG` reader - Masks PWRUPIFG in MIS register"]
pub type PwrupifgR = crate::BitReader;
#[doc = "Field `PWRUPIFG` writer - Masks PWRUPIFG in MIS register"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWNIFG` reader - Masks PWRDWNIFG in MIS register"]
pub type PwrdwnifgR = crate::BitReader;
#[doc = "Field `PWRDWNIFG` writer - Masks PWRDWNIFG in MIS register"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("txifg", &self.txifg())
            .field("rxifg", &self.rxifg())
            .field("pwrupifg", &self.pwrupifg())
            .field("pwrdwnifg", &self.pwrdwnifg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Masks TXIFG in MIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, ImaskSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Masks RXIFG in MIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, ImaskSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks PWRUPIFG in MIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, ImaskSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks PWRDWNIFG in MIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, ImaskSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
