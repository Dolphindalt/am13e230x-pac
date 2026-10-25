#[doc = "Register `BAUD2` reader"]
pub type R = crate::R<Baud2Spec>;
#[doc = "Register `BAUD2` writer"]
pub type W = crate::W<Baud2Spec>;
#[doc = "Field `COUNT0` reader - CS2n Baud Rate Counter 0"]
pub type Count0R = crate::FieldReader<u16>;
#[doc = "Field `COUNT0` writer - CS2n Baud Rate Counter 0"]
pub type Count0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
#[doc = "Field `COUNT1` reader - CS3n Baud Rate Counter 1"]
pub type Count1R = crate::FieldReader<u16>;
#[doc = "Field `COUNT1` writer - CS3n Baud Rate Counter 1"]
pub type Count1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn count0(&self) -> Count0R {
        Count0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn count1(&self) -> Count1R {
        Count1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BAUD2")
            .field("count0", &self.count0())
            .field("count1", &self.count1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn count0(&mut self) -> Count0W<'_, Baud2Spec> {
        Count0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn count1(&mut self) -> Count1W<'_, Baud2Spec> {
        Count1W::new(self, 16)
    }
}
#[doc = "EPI Main Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Baud2Spec;
impl crate::RegisterSpec for Baud2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud2::R`](R) reader structure"]
impl crate::Readable for Baud2Spec {}
#[doc = "`write(|w| ..)` method takes [`baud2::W`](W) writer structure"]
impl crate::Writable for Baud2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BAUD2 to value 0"]
impl crate::Resettable for Baud2Spec {}
