#[doc = "Register `X1CNT` reader"]
pub type R = crate::R<X1cntSpec>;
#[doc = "Register `X1CNT` writer"]
pub type W = crate::W<X1cntSpec>;
#[doc = "Field `X1CNT` reader - This counter increments on every X1 CLOCKs positive-edge. Once it reaches the values of 0x7ff, it freezes. Before switching from SYSOSC/PLL clock to X1, application must check this counter and make sure that it has saturated. This will ensure that the Crystal connected to X1/X2 is oscillating"]
pub type X1cntR = crate::FieldReader<u16>;
#[doc = "Field `CLR` writer - X1 Counter clear: A write of 1 to this bit field clears the X1CNT and makes it count from 0x0 again (provided X1 clock is ticking). Writes of 0 are ignore to this bit field"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - This counter increments on every X1 CLOCKs positive-edge. Once it reaches the values of 0x7ff, it freezes. Before switching from SYSOSC/PLL clock to X1, application must check this counter and make sure that it has saturated. This will ensure that the Crystal connected to X1/X2 is oscillating"]
    #[inline(always)]
    pub fn x1cnt(&self) -> X1cntR {
        X1cntR::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("X1CNT")
            .field("x1cnt", &self.x1cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - X1 Counter clear: A write of 1 to this bit field clears the X1CNT and makes it count from 0x0 again (provided X1 clock is ticking). Writes of 0 are ignore to this bit field"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, X1cntSpec> {
        ClrW::new(self, 16)
    }
}
#[doc = "x1cnt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`x1cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X1cntSpec;
impl crate::RegisterSpec for X1cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x1cnt::R`](R) reader structure"]
impl crate::Readable for X1cntSpec {}
#[doc = "`write(|w| ..)` method takes [`x1cnt::W`](W) writer structure"]
impl crate::Writable for X1cntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets X1CNT to value 0"]
impl crate::Resettable for X1cntSpec {}
