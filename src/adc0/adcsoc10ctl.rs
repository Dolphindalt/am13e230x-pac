#[doc = "Register `ADCSOC10CTL` reader"]
pub type R = crate::R<Adcsoc10ctlSpec>;
#[doc = "Register `ADCSOC10CTL` writer"]
pub type W = crate::W<Adcsoc10ctlSpec>;
#[doc = "Field `CHSEL` reader - SOC10 Channel Select"]
pub type ChselR = crate::FieldReader;
#[doc = "Field `CHSEL` writer - SOC10 Channel Select"]
pub type ChselW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `COMPEN` reader - SOC10 Threshold comparator enable"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - SOC10 Threshold comparator enable"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 15:19 - SOC10 Channel Select"]
    #[inline(always)]
    pub fn chsel(&self) -> ChselR {
        ChselR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - SOC10 Threshold comparator enable"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSOC10CTL")
            .field("chsel", &self.chsel())
            .field("compen", &self.compen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 15:19 - SOC10 Channel Select"]
    #[inline(always)]
    pub fn chsel(&mut self) -> ChselW<'_, Adcsoc10ctlSpec> {
        ChselW::new(self, 15)
    }
    #[doc = "Bit 25 - SOC10 Threshold comparator enable"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<'_, Adcsoc10ctlSpec> {
        CompenW::new(self, 25)
    }
}
#[doc = "ADC SOC10 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc10ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc10ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcsoc10ctlSpec;
impl crate::RegisterSpec for Adcsoc10ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsoc10ctl::R`](R) reader structure"]
impl crate::Readable for Adcsoc10ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsoc10ctl::W`](W) writer structure"]
impl crate::Writable for Adcsoc10ctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSOC10CTL to value 0"]
impl crate::Resettable for Adcsoc10ctlSpec {}
