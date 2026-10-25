#[doc = "Register `ADCSEQ4FIFOSTATUS` reader"]
pub type R = crate::R<Adcseq4fifostatusSpec>;
#[doc = "Register `ADCSEQ4FIFOSTATUS` writer"]
pub type W = crate::W<Adcseq4fifostatusSpec>;
#[doc = "Field `SEQFIFOVALID` reader - Sequencer 4 FIFO Valid"]
pub type SeqfifovalidR = crate::BitReader;
#[doc = "Field `SEQFIFOPENDREAD` reader - Sequencer 4 FIFO Pending reads"]
pub type SeqfifopendreadR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Sequencer 4 FIFO Valid"]
    #[inline(always)]
    pub fn seqfifovalid(&self) -> SeqfifovalidR {
        SeqfifovalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Sequencer 4 FIFO Pending reads"]
    #[inline(always)]
    pub fn seqfifopendread(&self) -> SeqfifopendreadR {
        SeqfifopendreadR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQ4FIFOSTATUS")
            .field("seqfifovalid", &self.seqfifovalid())
            .field("seqfifopendread", &self.seqfifopendread())
            .finish()
    }
}
impl W {}
#[doc = "ADC Sequence 4 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq4fifostatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq4fifostatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcseq4fifostatusSpec;
impl crate::RegisterSpec for Adcseq4fifostatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseq4fifostatus::R`](R) reader structure"]
impl crate::Readable for Adcseq4fifostatusSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseq4fifostatus::W`](W) writer structure"]
impl crate::Writable for Adcseq4fifostatusSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQ4FIFOSTATUS to value 0"]
impl crate::Resettable for Adcseq4fifostatusSpec {}
