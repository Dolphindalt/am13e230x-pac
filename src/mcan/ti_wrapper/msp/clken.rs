#[doc = "Register `CLKEN` reader"]
pub type R = crate::R<ClkenSpec>;
#[doc = "Register `CLKEN` writer"]
pub type W = crate::W<ClkenSpec>;
#[doc = "Field `CLK_REQEN` reader - MCAN clock request enable"]
pub type ClkReqenR = crate::BitReader;
#[doc = "Field `CLK_REQEN` writer - MCAN clock request enable"]
pub type ClkReqenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCAN clock request enable"]
    #[inline(always)]
    pub fn clk_reqen(&self) -> ClkReqenR {
        ClkReqenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKEN")
            .field("clk_reqen", &self.clk_reqen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCAN clock request enable"]
    #[inline(always)]
    pub fn clk_reqen(&mut self) -> ClkReqenW<'_, ClkenSpec> {
        ClkReqenW::new(self, 0)
    }
}
#[doc = "MCAN module clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkenSpec;
impl crate::RegisterSpec for ClkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clken::R`](R) reader structure"]
impl crate::Readable for ClkenSpec {}
#[doc = "`write(|w| ..)` method takes [`clken::W`](W) writer structure"]
impl crate::Writable for ClkenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKEN to value 0"]
impl crate::Resettable for ClkenSpec {}
