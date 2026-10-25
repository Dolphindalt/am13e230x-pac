#[doc = "Register `DTB_MUXSEL` reader"]
pub type R = crate::R<DtbMuxselSpec>;
#[doc = "Register `DTB_MUXSEL` writer"]
pub type W = crate::W<DtbMuxselSpec>;
#[doc = "Field `DTB_MUX_SEL_CPU` reader - CPU DTB MUX sel"]
pub type DtbMuxSelCpuR = crate::FieldReader;
#[doc = "Field `DTB_MUX_SEL_CPU` writer - CPU DTB MUX sel"]
pub type DtbMuxSelCpuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTB_MUX_SEL_WUC` reader - WUC DTB MUX sel"]
pub type DtbMuxSelWucR = crate::FieldReader;
#[doc = "Field `DTB_MUX_SEL_WUC` writer - WUC DTB MUX sel"]
pub type DtbMuxSelWucW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - CPU DTB MUX sel"]
    #[inline(always)]
    pub fn dtb_mux_sel_cpu(&self) -> DtbMuxSelCpuR {
        DtbMuxSelCpuR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 5:9 - WUC DTB MUX sel"]
    #[inline(always)]
    pub fn dtb_mux_sel_wuc(&self) -> DtbMuxSelWucR {
        DtbMuxSelWucR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTB_MUXSEL")
            .field("dtb_mux_sel_cpu", &self.dtb_mux_sel_cpu())
            .field("dtb_mux_sel_wuc", &self.dtb_mux_sel_wuc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU DTB MUX sel"]
    #[inline(always)]
    pub fn dtb_mux_sel_cpu(&mut self) -> DtbMuxSelCpuW<'_, DtbMuxselSpec> {
        DtbMuxSelCpuW::new(self, 0)
    }
    #[doc = "Bits 5:9 - WUC DTB MUX sel"]
    #[inline(always)]
    pub fn dtb_mux_sel_wuc(&mut self) -> DtbMuxSelWucW<'_, DtbMuxselSpec> {
        DtbMuxSelWucW::new(self, 5)
    }
}
#[doc = "Digital Test Bus MUX Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_muxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_muxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
