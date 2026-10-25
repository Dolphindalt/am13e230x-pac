#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `TXIFG` writer - Clears TXIFG in RIS register"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIFG` writer - Clears RXIFG in RIS register"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPIFG` writer - Clears PWRUPIFG in RIS register"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWNIFG` writer - Clears PWRDWNIFG in RIS register"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clears TXIFG in RIS register"]
    #[inline(always)]
    pub fn txifg(&mut self) -> TxifgW<'_, IclrSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears RXIFG in RIS register"]
    #[inline(always)]
    pub fn rxifg(&mut self) -> RxifgW<'_, IclrSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn pwrupifg(&mut self) -> PwrupifgW<'_, IclrSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<'_, IclrSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
