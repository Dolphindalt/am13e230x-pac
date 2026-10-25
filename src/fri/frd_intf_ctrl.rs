#[doc = "Register `FRD_INTF_CTRL` reader"]
pub type R = crate::R<FrdIntfCtrlSpec>;
#[doc = "Register `FRD_INTF_CTRL` writer"]
pub type W = crate::W<FrdIntfCtrlSpec>;
#[doc = "Field `DATA_CACHE_EN` reader - Data Cache Enable"]
pub type DataCacheEnR = crate::BitReader;
#[doc = "Field `DATA_CACHE_EN` writer - Data Cache Enable"]
pub type DataCacheEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE_CACHE_EN` reader - Code Cache Enable"]
pub type CodeCacheEnR = crate::BitReader;
#[doc = "Field `CODE_CACHE_EN` writer - Code Cache Enable"]
pub type CodeCacheEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Data Cache Enable"]
    #[inline(always)]
    pub fn data_cache_en(&self) -> DataCacheEnR {
        DataCacheEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Code Cache Enable"]
    #[inline(always)]
    pub fn code_cache_en(&self) -> CodeCacheEnR {
        CodeCacheEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRD_INTF_CTRL")
            .field("data_cache_en", &self.data_cache_en())
            .field("code_cache_en", &self.code_cache_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Data Cache Enable"]
    #[inline(always)]
    pub fn data_cache_en(&mut self) -> DataCacheEnW<'_, FrdIntfCtrlSpec> {
        DataCacheEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Code Cache Enable"]
    #[inline(always)]
    pub fn code_cache_en(&mut self) -> CodeCacheEnW<'_, FrdIntfCtrlSpec> {
        CodeCacheEnW::new(self, 2)
    }
}
#[doc = "Flash Read Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frd_intf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frd_intf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrdIntfCtrlSpec;
impl crate::RegisterSpec for FrdIntfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frd_intf_ctrl::R`](R) reader structure"]
impl crate::Readable for FrdIntfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`frd_intf_ctrl::W`](W) writer structure"]
impl crate::Writable for FrdIntfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRD_INTF_CTRL to value 0"]
impl crate::Resettable for FrdIntfCtrlSpec {}
