#[doc = "Register `QEPSTROBESEL` reader"]
pub type R = crate::R<QepstrobeselSpec>;
#[doc = "Register `QEPSTROBESEL` writer"]
pub type W = crate::W<QepstrobeselSpec>;
#[doc = "QMA Mode Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strobesel {
    #[doc = "0: QEP Strobe after polarity mux"]
    QsAfterPolMux = 0,
    #[doc = "2: QEP Strobe after polarity mux ORed with ADCSOCA"]
    AdcsocaAsQs = 2,
    #[doc = "3: QEP Strobe after polarity mux ORed with ADCSOCB"]
    AdcsocbAsQs = 3,
}
impl From<Strobesel> for u8 {
    #[inline(always)]
    fn from(variant: Strobesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strobesel {
    type Ux = u8;
}
impl crate::IsEnum for Strobesel {}
#[doc = "Field `STROBESEL` reader - QMA Mode Select"]
pub type StrobeselR = crate::FieldReader<Strobesel>;
impl StrobeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strobesel> {
        match self.bits {
            0 => Some(Strobesel::QsAfterPolMux),
            2 => Some(Strobesel::AdcsocaAsQs),
            3 => Some(Strobesel::AdcsocbAsQs),
            _ => None,
        }
    }
    #[doc = "QEP Strobe after polarity mux"]
    #[inline(always)]
    pub fn is_qs_after_pol_mux(&self) -> bool {
        *self == Strobesel::QsAfterPolMux
    }
    #[doc = "QEP Strobe after polarity mux ORed with ADCSOCA"]
    #[inline(always)]
    pub fn is_adcsoca_as_qs(&self) -> bool {
        *self == Strobesel::AdcsocaAsQs
    }
    #[doc = "QEP Strobe after polarity mux ORed with ADCSOCB"]
    #[inline(always)]
    pub fn is_adcsocb_as_qs(&self) -> bool {
        *self == Strobesel::AdcsocbAsQs
    }
}
#[doc = "Field `STROBESEL` writer - QMA Mode Select"]
pub type StrobeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Strobesel, crate::Safe>;
impl<'a, REG> StrobeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QEP Strobe after polarity mux"]
    #[inline(always)]
    pub fn qs_after_pol_mux(self) -> &'a mut crate::W<REG> {
        self.variant(Strobesel::QsAfterPolMux)
    }
    #[doc = "QEP Strobe after polarity mux ORed with ADCSOCA"]
    #[inline(always)]
    pub fn adcsoca_as_qs(self) -> &'a mut crate::W<REG> {
        self.variant(Strobesel::AdcsocaAsQs)
    }
    #[doc = "QEP Strobe after polarity mux ORed with ADCSOCB"]
    #[inline(always)]
    pub fn adcsocb_as_qs(self) -> &'a mut crate::W<REG> {
        self.variant(Strobesel::AdcsocbAsQs)
    }
}
impl R {
    #[doc = "Bits 0:1 - QMA Mode Select"]
    #[inline(always)]
    pub fn strobesel(&self) -> StrobeselR {
        StrobeselR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QEPSTROBESEL")
            .field("strobesel", &self.strobesel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - QMA Mode Select"]
    #[inline(always)]
    pub fn strobesel(&mut self) -> StrobeselW<'_, QepstrobeselSpec> {
        StrobeselW::new(self, 0)
    }
}
#[doc = "QEP Strobe select register\n\nYou can [`read`](crate::Reg::read) this register and get [`qepstrobesel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepstrobesel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepstrobeselSpec;
impl crate::RegisterSpec for QepstrobeselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qepstrobesel::R`](R) reader structure"]
impl crate::Readable for QepstrobeselSpec {}
#[doc = "`write(|w| ..)` method takes [`qepstrobesel::W`](W) writer structure"]
impl crate::Writable for QepstrobeselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QEPSTROBESEL to value 0"]
impl crate::Resettable for QepstrobeselSpec {}
