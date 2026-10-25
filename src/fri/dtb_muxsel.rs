#[doc = "Register `DTB_MUXSEL` reader"]
pub type R = crate::R<DtbMuxselSpec>;
#[doc = "Register `DTB_MUXSEL` writer"]
pub type W = crate::W<DtbMuxselSpec>;
#[doc = "Field `DTB_MUX_SEL_FRI` reader - FRI DTB Mux Select"]
pub type DtbMuxSelFriR = crate::FieldReader;
#[doc = "Field `DTB_MUX_SEL_FRI` writer - FRI DTB Mux Select"]
pub type DtbMuxSelFriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - FRI DTB Mux Select"]
    #[inline(always)]
    pub fn dtb_mux_sel_fri(&self) -> DtbMuxSelFriR {
        DtbMuxSelFriR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTB_MUXSEL")
            .field("dtb_mux_sel_fri", &self.dtb_mux_sel_fri())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FRI DTB Mux Select"]
    #[inline(always)]
    pub fn dtb_mux_sel_fri(&mut self) -> DtbMuxSelFriW<'_, DtbMuxselSpec> {
        DtbMuxSelFriW::new(self, 0)
    }
}
#[doc = "Flash Read Interface DTB Mux select\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_muxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_muxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbMuxselSpec;
impl crate::RegisterSpec for DtbMuxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtb_muxsel::R`](R) reader structure"]
impl crate::Readable for DtbMuxselSpec {}
#[doc = "`write(|w| ..)` method takes [`dtb_muxsel::W`](W) writer structure"]
impl crate::Writable for DtbMuxselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTB_MUXSEL to value 0"]
impl crate::Resettable for DtbMuxselSpec {}
