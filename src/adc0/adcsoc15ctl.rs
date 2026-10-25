#[doc = "Register `ADCSOC15CTL` reader"]
pub type R = crate::R<Adcsoc15ctlSpec>;
#[doc = "Register `ADCSOC15CTL` writer"]
pub type W = crate::W<Adcsoc15ctlSpec>;
#[doc = "Field `CHSEL` reader - SOC15 Channel Select"]
pub type ChselR = crate::FieldReader;
#[doc = "Field `CHSEL` writer - SOC15 Channel Select"]
pub type ChselW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `COMPEN` reader - SOC15 Threshold comparator enable"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - SOC15 Threshold comparator enable"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 15:19 - SOC15 Channel Select"]
    #[inline(always)]
    pub fn chsel(&self) -> ChselR {
        ChselR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - SOC15 Threshold comparator enable"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOC15CTL")
            .field("chsel", &self.chsel())
            .field("compen", &self.compen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 15:19 - SOC15 Channel Select"]
    #[inline(always)]
    pub fn chsel(&mut self) -> ChselW<'_, Adcsoc15ctlSpec> {
        ChselW::new(self, 15)
    }
    #[doc = "Bit 25 - SOC15 Threshold comparator enable"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<'_, Adcsoc15ctlSpec> {
        CompenW::new(self, 25)
    }
}
#[doc = "ADC SOC15 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc15ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc15ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcsoc15ctlSpec;
impl crate::RegisterSpec for Adcsoc15ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsoc15ctl::R`](R) reader structure"]
impl crate::Readable for Adcsoc15ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsoc15ctl::W`](W) writer structure"]
impl crate::Writable for Adcsoc15ctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOC15CTL to value 0"]
impl crate::Resettable for Adcsoc15ctlSpec {}
