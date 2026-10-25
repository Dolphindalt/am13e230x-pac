#[doc = "Register `RCLD` reader"]
pub type R = crate::R<RcldSpec>;
#[doc = "Register `RCLD` writer"]
pub type W = crate::W<RcldSpec>;
#[doc = "Field `RCLD` reader - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
pub type RcldR = crate::FieldReader;
#[doc = "Field `RCLD` writer - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
pub type RcldW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
    #[inline(always)]
    pub fn rcld(&self) -> RcldR {
        RcldR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCLD").field("rcld", &self.rcld()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
    #[inline(always)]
    pub fn rcld(&mut self) -> RcldW<'_, RcldSpec> {
        RcldW::new(self, 0)
    }
}
#[doc = "Repeat counter load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcldSpec;
impl crate::RegisterSpec for RcldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcld::R`](R) reader structure"]
impl crate::Readable for RcldSpec {}
#[doc = "`write(|w| ..)` method takes [`rcld::W`](W) writer structure"]
impl crate::Writable for RcldSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RCLD to value 0"]
impl crate::Resettable for RcldSpec {}
