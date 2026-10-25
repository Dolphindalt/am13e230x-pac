#[doc = "Register `FRDCNTL` reader"]
pub type R = crate::R<FrdcntlSpec>;
#[doc = "Register `FRDCNTL` writer"]
pub type W = crate::W<FrdcntlSpec>;
#[doc = "Field `WS0_MODE` reader - Wait state 0 Mode"]
pub type Ws0ModeR = crate::BitReader;
#[doc = "Field `WS0_MODE` writer - Wait state 0 Mode"]
pub type Ws0ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWAIT` reader - Random Read Waitstate"]
pub type RwaitR = crate::FieldReader;
#[doc = "Field `RWAIT` writer - Random Read Waitstate"]
pub type RwaitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Wait state 0 Mode"]
    #[inline(always)]
    pub fn ws0_mode(&self) -> Ws0ModeR {
        Ws0ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Random Read Waitstate"]
    #[inline(always)]
    pub fn rwait(&self) -> RwaitR {
        RwaitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDCNTL")
            .field("ws0_mode", &self.ws0_mode())
            .field("rwait", &self.rwait())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Wait state 0 Mode"]
    #[inline(always)]
    pub fn ws0_mode(&mut self) -> Ws0ModeW<'_, FrdcntlSpec> {
        Ws0ModeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Random Read Waitstate"]
    #[inline(always)]
    pub fn rwait(&mut self) -> RwaitW<'_, FrdcntlSpec> {
        RwaitW::new(self, 8)
    }
}
#[doc = "Flash Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frdcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frdcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrdcntlSpec;
impl crate::RegisterSpec for FrdcntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frdcntl::R`](R) reader structure"]
impl crate::Readable for FrdcntlSpec {}
#[doc = "`write(|w| ..)` method takes [`frdcntl::W`](W) writer structure"]
impl crate::Writable for FrdcntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRDCNTL to value 0x0200_0201"]
impl crate::Resettable for FrdcntlSpec {
    const RESET_VALUE: u32 = 0x0200_0201;
}
