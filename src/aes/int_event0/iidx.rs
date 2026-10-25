#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    Outputrdy = 1,
    #[doc = "2: This indicates that the engine can take new input. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    Inputrdy = 2,
    #[doc = "3: This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
    Savedcntxtrdy = 3,
    #[doc = "4: This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write new context."]
    Cntxtrdy = 4,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Outputrdy),
            2 => Some(Stat::Inputrdy),
            3 => Some(Stat::Savedcntxtrdy),
            4 => Some(Stat::Cntxtrdy),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn is_outputrdy(&self) -> bool {
        *self == Stat::Outputrdy
    }
    #[doc = "This indicates that the engine can take new input. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn is_inputrdy(&self) -> bool {
        *self == Stat::Inputrdy
    }
    #[doc = "This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
    #[inline(always)]
    pub fn is_savedcntxtrdy(&self) -> bool {
        *self == Stat::Savedcntxtrdy
    }
    #[doc = "This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write new context."]
    #[inline(always)]
    pub fn is_cntxtrdy(&self) -> bool {
        *self == Stat::Cntxtrdy
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDX").field("stat", &self.stat()).finish()
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
