#[doc = "Register `ADCSEQ3FIFOSTATUS` reader"]
pub type R = crate::R<Adcseq3fifostatusSpec>;
#[doc = "Register `ADCSEQ3FIFOSTATUS` writer"]
pub type W = crate::W<Adcseq3fifostatusSpec>;
#[doc = "Field `SEQFIFOVALID` reader - Sequencer 3 FIFO Valid"]
pub type SeqfifovalidR = crate::BitReader;
#[doc = "Field `SEQFIFOPENDREAD` reader - Sequencer 3 FIFO Pending reads"]
pub type SeqfifopendreadR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Sequencer 3 FIFO Valid"]
    #[inline(always)]
    pub fn seqfifovalid(&self) -> SeqfifovalidR {
        SeqfifovalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Sequencer 3 FIFO Pending reads"]
    #[inline(always)]
    pub fn seqfifopendread(&self) -> SeqfifopendreadR {
        SeqfifopendreadR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQ3FIFOSTATUS")
            .field("seqfifovalid", &self.seqfifovalid())
            .field("seqfifopendread", &self.seqfifopendread())
            .finish()
    }
}
impl W {}
#[doc = "ADC Sequence 3 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq3fifostatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq3fifostatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq3fifostatusSpec;
impl crate::RegisterSpec for Adcseq3fifostatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq3fifostatus::R`](R) reader structure"]
impl crate::Readable for Adcseq3fifostatusSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq3fifostatus::W`](W) writer structure"]
impl crate::Writable for Adcseq3fifostatusSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ3FIFOSTATUS to value 0"]
impl crate::Resettable for Adcseq3fifostatusSpec {}
