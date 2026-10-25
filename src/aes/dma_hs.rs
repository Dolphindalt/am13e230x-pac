#[doc = "Register `DMA_HS` reader"]
pub type R = crate::R<DmaHsSpec>;
#[doc = "Register `DMA_HS` writer"]
pub type W = crate::W<DmaHsSpec>;
#[doc = "When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDataAck {
    #[doc = "0: Disable DMA based data handshake"]
    Disable = 0,
    #[doc = "1: Enables DMA based handshake"]
    Enable = 1,
}
impl From<DmaDataAck> for bool {
    #[inline(always)]
    fn from(variant: DmaDataAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DATA_ACK` reader - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
pub type DmaDataAckR = crate::BitReader<DmaDataAck>;
impl DmaDataAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDataAck {
        match self.bits {
            false => DmaDataAck::Disable,
            true => DmaDataAck::Enable,
        }
    }
    #[doc = "Disable DMA based data handshake"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DmaDataAck::Disable
    }
    #[doc = "Enables DMA based handshake"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DmaDataAck::Enable
    }
}
#[doc = "Field `DMA_DATA_ACK` writer - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
pub type DmaDataAckW<'a, REG> = crate::BitWriter<'a, REG, DmaDataAck>;
impl<'a, REG> DmaDataAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA based data handshake"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDataAck::Disable)
    }
    #[doc = "Enables DMA based handshake"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDataAck::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
    #[inline(always)]
    pub fn dma_data_ack(&self) -> DmaDataAckR {
        DmaDataAckR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_HS")
            .field("dma_data_ack", &self.dma_data_ack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is 0b, input and output data acknowledge is I/O register based, as specified in the description of the AES_DATA_IN_n / AES_DATA_OUT_n registers. When this bit is 1b, input and ouput data acknowledge is based on DMA handshake signals."]
    #[inline(always)]
    pub fn dma_data_ack(&mut self) -> DmaDataAckW<'_, DmaHsSpec> {
        DmaDataAckW::new(self, 0)
    }
}
#[doc = "Control register for DMA handshaking\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaHsSpec;
impl crate::RegisterSpec for DmaHsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hs::R`](R) reader structure"]
impl crate::Readable for DmaHsSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_hs::W`](W) writer structure"]
impl crate::Writable for DmaHsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DMA_HS to value 0"]
impl crate::Resettable for DmaHsSpec {}
