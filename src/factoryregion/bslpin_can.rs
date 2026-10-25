#[doc = "Register `BSLPIN_CAN` reader"]
pub type R = crate::R<BslpinCanSpec>;
#[doc = "Field `CAN_RX_PAD` reader - MCAN RX Pin used by BSL"]
pub type CanRxPadR = crate::FieldReader;
#[doc = "Field `CAN_RX_PF` reader - MCAN RX Pin Function Selection Value"]
pub type CanRxPfR = crate::FieldReader;
#[doc = "Field `CAN_TX_PAD` reader - MCAN TX Pin used by BSL"]
pub type CanTxPadR = crate::FieldReader;
#[doc = "Field `CAN_TX_PF` reader - MCAN TX Pin Function Selection Value"]
pub type CanTxPfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - MCAN RX Pin used by BSL"]
    #[inline(always)]
    pub fn can_rx_pad(&self) -> CanRxPadR {
        CanRxPadR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MCAN RX Pin Function Selection Value"]
    #[inline(always)]
    pub fn can_rx_pf(&self) -> CanRxPfR {
        CanRxPfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MCAN TX Pin used by BSL"]
    #[inline(always)]
    pub fn can_tx_pad(&self) -> CanTxPadR {
        CanTxPadR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MCAN TX Pin Function Selection Value"]
    #[inline(always)]
    pub fn can_tx_pf(&self) -> CanTxPfR {
        CanTxPfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSLPIN_CAN")
            .field("can_rx_pad", &self.can_rx_pad())
            .field("can_rx_pf", &self.can_rx_pf())
            .field("can_tx_pad", &self.can_tx_pad())
            .field("can_tx_pf", &self.can_tx_pf())
            .finish()
    }
}
#[doc = "BSL CAN Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_can::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslpinCanSpec;
impl crate::RegisterSpec for BslpinCanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslpin_can::R`](R) reader structure"]
impl crate::Readable for BslpinCanSpec {}
#[doc = "`reset()` method sets BSLPIN_CAN to value 0"]
impl crate::Resettable for BslpinCanSpec {}
