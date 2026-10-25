#[doc = "Register `SYNCSOCLOCK` reader"]
pub type R = crate::R<SyncsoclockSpec>;
#[doc = "Register `SYNCSOCLOCK` writer"]
pub type W = crate::W<SyncsoclockSpec>;
#[doc = "Field `SYNCSELECT` reader - SYNCSEL Register Lock bit"]
pub type SyncselectR = crate::BitReader;
#[doc = "Field `SYNCSELECT` writer - SYNCSEL Register Lock bit"]
pub type SyncselectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSOCOUTSELECT` reader - ADCSOCOUTSELECT Register Lock bit"]
pub type AdcsocoutselectR = crate::BitReader;
#[doc = "Field `ADCSOCOUTSELECT` writer - ADCSOCOUTSELECT Register Lock bit"]
pub type AdcsocoutselectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNCSEL Register Lock bit"]
    #[inline(always)]
    pub fn syncselect(&self) -> SyncselectR {
        SyncselectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADCSOCOUTSELECT Register Lock bit"]
    #[inline(always)]
    pub fn adcsocoutselect(&self) -> AdcsocoutselectR {
        AdcsocoutselectR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCSOCLOCK")
            .field("syncselect", &self.syncselect())
            .field("adcsocoutselect", &self.adcsocoutselect())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYNCSEL Register Lock bit"]
    #[inline(always)]
    pub fn syncselect(&mut self) -> SyncselectW<'_, SyncsoclockSpec> {
        SyncselectW::new(self, 0)
    }
    #[doc = "Bit 1 - ADCSOCOUTSELECT Register Lock bit"]
    #[inline(always)]
    pub fn adcsocoutselect(&mut self) -> AdcsocoutselectW<'_, SyncsoclockSpec> {
        AdcsocoutselectW::new(self, 1)
    }
}
#[doc = "SYNCSEL and EXTADCSOC Select Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncsoclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncsoclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncsoclockSpec;
impl crate::RegisterSpec for SyncsoclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncsoclock::R`](R) reader structure"]
impl crate::Readable for SyncsoclockSpec {}
#[doc = "`write(|w| ..)` method takes [`syncsoclock::W`](W) writer structure"]
impl crate::Writable for SyncsoclockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYNCSOCLOCK to value 0"]
impl crate::Resettable for SyncsoclockSpec {}
