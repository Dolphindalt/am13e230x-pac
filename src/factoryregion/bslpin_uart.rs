#[doc = "Register `BSLPIN_UART` reader"]
pub type R = crate::R<BslpinUartSpec>;
#[doc = "Field `UART_RXD_PAD` reader - UART RXD Pad used by BSL"]
pub type UartRxdPadR = crate::FieldReader;
#[doc = "Field `UART_RXD_PF` reader - UART RXD Pin Function Selection Value"]
pub type UartRxdPfR = crate::FieldReader;
#[doc = "Field `UART_TXD_PAD` reader - UART TXD Pin used by BSL"]
pub type UartTxdPadR = crate::FieldReader;
#[doc = "Field `UART_TXD_PF` reader - UART TXD Pin Function Selection Value"]
pub type UartTxdPfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART RXD Pad used by BSL"]
    #[inline(always)]
    pub fn uart_rxd_pad(&self) -> UartRxdPadR {
        UartRxdPadR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UART RXD Pin Function Selection Value"]
    #[inline(always)]
    pub fn uart_rxd_pf(&self) -> UartRxdPfR {
        UartRxdPfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UART TXD Pin used by BSL"]
    #[inline(always)]
    pub fn uart_txd_pad(&self) -> UartTxdPadR {
        UartTxdPadR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UART TXD Pin Function Selection Value"]
    #[inline(always)]
    pub fn uart_txd_pf(&self) -> UartTxdPfR {
        UartTxdPfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSLPIN_UART")
            .field("uart_rxd_pad", &self.uart_rxd_pad())
            .field("uart_rxd_pf", &self.uart_rxd_pf())
            .field("uart_txd_pad", &self.uart_txd_pad())
            .field("uart_txd_pf", &self.uart_txd_pf())
            .finish()
    }
}
#[doc = "BSL UART Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_uart::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslpinUartSpec;
impl crate::RegisterSpec for BslpinUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslpin_uart::R`](R) reader structure"]
impl crate::Readable for BslpinUartSpec {}
#[doc = "`reset()` method sets BSLPIN_UART to value 0"]
impl crate::Resettable for BslpinUartSpec {}
