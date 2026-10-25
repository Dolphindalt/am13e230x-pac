#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `TXIFG` writer - Sets TXIFG in RIS register"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIFG` writer - Sets RXIFG in RIS register"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPIFG` writer - Sets PWRUPIFG in RIS register"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWNIFG` writer - Sets PWRDWNIFG in RIS register"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Sets TXIFG in RIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, IsetSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets RXIFG in RIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, IsetSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Sets PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, IsetSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Sets PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, IsetSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {}
