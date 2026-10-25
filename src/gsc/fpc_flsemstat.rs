#[doc = "Register `FPC_FLSEMSTAT` reader"]
pub type R = crate::R<FpcFlsemstatSpec>;
#[doc = "Register `FPC_FLSEMSTAT` writer"]
pub type W = crate::W<FpcFlsemstatSpec>;
#[doc = "Field `PRIV` reader - Semaphore Privilage attribute"]
pub type PrivR = crate::BitReader;
#[doc = "Field `DBGACC` reader - Semaphore debug access"]
pub type DbgaccR = crate::BitReader;
#[doc = "Field `MATCH` reader - Match Status Bit"]
pub type MatchR = crate::BitReader;
#[doc = "Field `ASSIGNED` reader - Assigned Flag"]
pub type AssignedR = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Semaphore Privilage attribute"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Semaphore debug access"]
    #[inline(always)]
    pub fn dbgacc(&self) -> DbgaccR {
        DbgaccR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Match Status Bit"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Assigned Flag"]
    #[inline(always)]
    pub fn assigned(&self) -> AssignedR {
        AssignedR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_FLSEMSTAT")
            .field("priv_", &self.priv_())
            .field("dbgacc", &self.dbgacc())
            .field("match_", &self.match_())
            .field("assigned", &self.assigned())
            .finish()
    }
}
impl W {}
#[doc = "Flash semaphore statys registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcFlsemstatSpec;
impl crate::RegisterSpec for FpcFlsemstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_flsemstat::R`](R) reader structure"]
impl crate::Readable for FpcFlsemstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_flsemstat::W`](W) writer structure"]
impl crate::Writable for FpcFlsemstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_FLSEMSTAT to value 0"]
impl crate::Resettable for FpcFlsemstatSpec {}
