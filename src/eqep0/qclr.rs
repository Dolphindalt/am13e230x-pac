#[doc = "Register `QCLR` reader"]
pub type R = crate::R<QclrSpec>;
#[doc = "Register `QCLR` writer"]
pub type W = crate::W<QclrSpec>;
#[doc = "Field `INT` reader - Global interrupt clear flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Global interrupt clear flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Clear position counter error interrupt flag"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Clear position counter error interrupt flag"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHE` reader - Clear quadrature phase error interrupt flag"]
pub type PheR = crate::BitReader;
#[doc = "Field `PHE` writer - Clear quadrature phase error interrupt flag"]
pub type PheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDC` reader - Clear quadrature direction change interrupt flag"]
pub type QdcR = crate::BitReader;
#[doc = "Field `QDC` writer - Clear quadrature direction change interrupt flag"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTO` reader - Clear watchdog timeout interrupt flag"]
pub type WtoR = crate::BitReader;
#[doc = "Field `WTO` writer - Clear watchdog timeout interrupt flag"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCU` reader - Clear position counter underflow interrupt flag"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCU` writer - Clear position counter underflow interrupt flag"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCO` reader - Clear position counter overflow interrupt flag"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCO` writer - Clear position counter overflow interrupt flag"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - Clear position-compare ready interrupt flag"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCR` writer - Clear position-compare ready interrupt flag"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - Clear eQEP compare match event interrupt flag"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - Clear eQEP compare match event interrupt flag"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Clear strobe event latch interrupt flag"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Clear strobe event latch interrupt flag"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - Clear index event latch interrupt flag"]
pub type IelR = crate::BitReader;
#[doc = "Field `IEL` writer - Clear index event latch interrupt flag"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - Clear unit time out interrupt flag"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - Clear unit time out interrupt flag"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QMAE` reader - Clear QMA error interrupt flag"]
pub type QmaeR = crate::BitReader;
#[doc = "Field `QMAE` writer - Clear QMA error interrupt flag"]
pub type QmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt clear flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear position counter error interrupt flag"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear quadrature phase error interrupt flag"]
    #[inline(always)]
    pub fn phe(&self) -> PheR {
        PheR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear quadrature direction change interrupt flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear watchdog timeout interrupt flag"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear position counter underflow interrupt flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear position counter overflow interrupt flag"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear position-compare ready interrupt flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear eQEP compare match event interrupt flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear strobe event latch interrupt flag"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear index event latch interrupt flag"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear unit time out interrupt flag"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clear QMA error interrupt flag"]
    #[inline(always)]
    pub fn qmae(&self) -> QmaeR {
        QmaeR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QCLR")
            .field("int", &self.int())
            .field("pce", &self.pce())
            .field("phe", &self.phe())
            .field("qdc", &self.qdc())
            .field("wto", &self.wto())
            .field("pcu", &self.pcu())
            .field("pco", &self.pco())
            .field("pcr", &self.pcr())
            .field("pcm", &self.pcm())
            .field("sel", &self.sel())
            .field("iel", &self.iel())
            .field("uto", &self.uto())
            .field("qmae", &self.qmae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt clear flag"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, QclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear position counter error interrupt flag"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, QclrSpec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear quadrature phase error interrupt flag"]
    #[inline(always)]
    pub fn phe(&mut self) -> PheW<'_, QclrSpec> {
        PheW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear quadrature direction change interrupt flag"]
    #[inline(always)]
    pub fn qdc(&mut self) -> QdcW<'_, QclrSpec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear watchdog timeout interrupt flag"]
    #[inline(always)]
    pub fn wto(&mut self) -> WtoW<'_, QclrSpec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear position counter underflow interrupt flag"]
    #[inline(always)]
    pub fn pcu(&mut self) -> PcuW<'_, QclrSpec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear position counter overflow interrupt flag"]
    #[inline(always)]
    pub fn pco(&mut self) -> PcoW<'_, QclrSpec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear position-compare ready interrupt flag"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<'_, QclrSpec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear eQEP compare match event interrupt flag"]
    #[inline(always)]
    pub fn pcm(&mut self) -> PcmW<'_, QclrSpec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear strobe event latch interrupt flag"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, QclrSpec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear index event latch interrupt flag"]
    #[inline(always)]
    pub fn iel(&mut self) -> IelW<'_, QclrSpec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear unit time out interrupt flag"]
    #[inline(always)]
    pub fn uto(&mut self) -> UtoW<'_, QclrSpec> {
        UtoW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear QMA error interrupt flag"]
    #[inline(always)]
    pub fn qmae(&mut self) -> QmaeW<'_, QclrSpec> {
        QmaeW::new(self, 12)
    }
}
#[doc = "QEP Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`qclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QclrSpec;
impl crate::RegisterSpec for QclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qclr::R`](R) reader structure"]
impl crate::Readable for QclrSpec {}
#[doc = "`write(|w| ..)` method takes [`qclr::W`](W) writer structure"]
impl crate::Writable for QclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCLR to value 0"]
impl crate::Resettable for QclrSpec {}
