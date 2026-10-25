#[doc = "Register `TEST0` reader"]
pub type R = crate::R<Test0Spec>;
#[doc = "Register `TEST0` writer"]
pub type W = crate::W<Test0Spec>;
#[doc = "Field `DTB_MUX_SEL` reader - This bit field is used to select DTB mux digital output signals."]
pub type DtbMuxSelR = crate::FieldReader;
#[doc = "Field `DTB_MUX_SEL` writer - This bit field is used to select DTB mux digital output signals."]
pub type DtbMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - This bit field is used to select DTB mux digital output signals."]
    #[inline(always)]
    pub fn dtb_mux_sel(&self) -> DtbMuxSelR {
        DtbMuxSelR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST0")
            .field("dtb_mux_sel", &self.dtb_mux_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - This bit field is used to select DTB mux digital output signals."]
    #[inline(always)]
    pub fn dtb_mux_sel(&mut self) -> DtbMuxSelW<'_, Test0Spec> {
        DtbMuxSelW::new(self, 0)
    }
}
#[doc = "Test 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`test0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test0Spec;
impl crate::RegisterSpec for Test0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test0::R`](R) reader structure"]
impl crate::Readable for Test0Spec {}
#[doc = "`write(|w| ..)` method takes [`test0::W`](W) writer structure"]
impl crate::Writable for Test0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TEST0 to value 0"]
impl crate::Resettable for Test0Spec {}
