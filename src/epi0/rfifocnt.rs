#[doc = "Register `RFIFOCNT` reader"]
pub type R = crate::R<RfifocntSpec>;
#[doc = "Register `RFIFOCNT` writer"]
pub type W = crate::W<RfifocntSpec>;
#[doc = "Field `COUNT` reader - FIFO Count."]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - FIFO Count."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - FIFO Count."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFIFOCNT")
            .field("count", &self.count())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Count."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RfifocntSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "EPI Read FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifocnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifocnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifocntSpec;
impl crate::RegisterSpec for RfifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifocnt::R`](R) reader structure"]
impl crate::Readable for RfifocntSpec {}
#[doc = "`write(|w| ..)` method takes [`rfifocnt::W`](W) writer structure"]
impl crate::Writable for RfifocntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RFIFOCNT to value 0"]
impl crate::Resettable for RfifocntSpec {}
