#[doc = "Register `PPC_PRIVATTRIB_XBAR` reader"]
pub type R = crate::R<PpcPrivattribXbarSpec>;
#[doc = "Register `PPC_PRIVATTRIB_XBAR` writer"]
pub type W = crate::W<PpcPrivattribXbarSpec>;
#[doc = "Field `PRIV_ALL_XBAR_CONFIG` reader - Privilege access for ALL_XBAR_CONFIG"]
pub type PrivAllXbarConfigR = crate::BitReader;
#[doc = "Field `PRIV_ALL_XBAR_CONFIG` writer - Privilege access for ALL_XBAR_CONFIG"]
pub type PrivAllXbarConfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_XBAR_INPUT_FLAGS` reader - Privilege access for XBAR_INPUT_FLAGS"]
pub type PrivXbarInputFlagsR = crate::BitReader;
#[doc = "Field `PRIV_XBAR_INPUT_FLAGS` writer - Privilege access for XBAR_INPUT_FLAGS"]
pub type PrivXbarInputFlagsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_OUTPUT_XBAR_FLAGS` reader - Privilege access for OUTPUT_XBAR_FLAGS"]
pub type PrivOutputXbarFlagsR = crate::BitReader;
#[doc = "Field `PRIV_OUTPUT_XBAR_FLAGS` writer - Privilege access for OUTPUT_XBAR_FLAGS"]
pub type PrivOutputXbarFlagsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for ALL_XBAR_CONFIG"]
    #[inline(always)]
    pub fn priv_all_xbar_config(&self) -> PrivAllXbarConfigR {
        PrivAllXbarConfigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for XBAR_INPUT_FLAGS"]
    #[inline(always)]
    pub fn priv_xbar_input_flags(&self) -> PrivXbarInputFlagsR {
        PrivXbarInputFlagsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for OUTPUT_XBAR_FLAGS"]
    #[inline(always)]
    pub fn priv_output_xbar_flags(&self) -> PrivOutputXbarFlagsR {
        PrivOutputXbarFlagsR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_XBAR")
            .field("priv_all_xbar_config", &self.priv_all_xbar_config())
            .field("priv_xbar_input_flags", &self.priv_xbar_input_flags())
            .field("priv_output_xbar_flags", &self.priv_output_xbar_flags())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for ALL_XBAR_CONFIG"]
    #[inline(always)]
    pub fn priv_all_xbar_config(&mut self) -> PrivAllXbarConfigW<'_, PpcPrivattribXbarSpec> {
        PrivAllXbarConfigW::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for XBAR_INPUT_FLAGS"]
    #[inline(always)]
    pub fn priv_xbar_input_flags(&mut self) -> PrivXbarInputFlagsW<'_, PpcPrivattribXbarSpec> {
        PrivXbarInputFlagsW::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for OUTPUT_XBAR_FLAGS"]
    #[inline(always)]
    pub fn priv_output_xbar_flags(&mut self) -> PrivOutputXbarFlagsW<'_, PpcPrivattribXbarSpec> {
        PrivOutputXbarFlagsW::new(self, 2)
    }
}
#[doc = "Peripheral Privilege Access XBAR\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_xbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_xbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribXbarSpec;
impl crate::RegisterSpec for PpcPrivattribXbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_xbar::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribXbarSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_xbar::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribXbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_XBAR to value 0"]
impl crate::Resettable for PpcPrivattribXbarSpec {}
