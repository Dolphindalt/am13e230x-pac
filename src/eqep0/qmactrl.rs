#[doc = "Register `QMACTRL` reader"]
pub type R = crate::R<QmactrlSpec>;
#[doc = "Register `QMACTRL` writer"]
pub type W = crate::W<QmactrlSpec>;
#[doc = "Field `MODE` reader - QMA Mode Select"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - QMA Mode Select"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - QMA Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMACTRL")
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - QMA Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, QmactrlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "QMA Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`qmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QmactrlSpec;
impl crate::RegisterSpec for QmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qmactrl::R`](R) reader structure"]
impl crate::Readable for QmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`qmactrl::W`](W) writer structure"]
impl crate::Writable for QmactrlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QMACTRL to value 0"]
impl crate::Resettable for QmactrlSpec {}
