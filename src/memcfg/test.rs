#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `TEST_SRAM0` reader - Selects the different test modes for ULL RAM"]
pub type TestSram0R = crate::FieldReader;
#[doc = "Field `TEST_SRAM0` writer - Selects the different test modes for ULL RAM"]
pub type TestSram0W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `TEST_SRAM1` reader - Selects the different test modes for GLXMP_0 RAM"]
pub type TestSram1R = crate::FieldReader;
#[doc = "Field `TEST_SRAM1` writer - Selects the different test modes for GLXMP_0 RAM"]
pub type TestSram1W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `TEST_SRAM2` reader - Selects the different test modes for GLXMP_1 RAM"]
pub type TestSram2R = crate::FieldReader;
#[doc = "Field `TEST_SRAM2` writer - Selects the different test modes for GLXMP_1 RAM"]
pub type TestSram2W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `TEST_SRAM3` reader - Selects the different test modes for UGLXMP_2 RAM"]
pub type TestSram3R = crate::FieldReader;
#[doc = "Field `TEST_SRAM3` writer - Selects the different test modes for UGLXMP_2 RAM"]
pub type TestSram3W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Selects the different test modes for ULL RAM"]
    #[inline(always)]
    pub fn test_sram0(&self) -> TestSram0R {
        TestSram0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Selects the different test modes for GLXMP_0 RAM"]
    #[inline(always)]
    pub fn test_sram1(&self) -> TestSram1R {
        TestSram1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Selects the different test modes for GLXMP_1 RAM"]
    #[inline(always)]
    pub fn test_sram2(&self) -> TestSram2R {
        TestSram2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Selects the different test modes for UGLXMP_2 RAM"]
    #[inline(always)]
    pub fn test_sram3(&self) -> TestSram3R {
        TestSram3R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("test_sram0", &self.test_sram0())
            .field("test_sram1", &self.test_sram1())
            .field("test_sram2", &self.test_sram2())
            .field("test_sram3", &self.test_sram3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the different test modes for ULL RAM"]
    #[inline(always)]
    pub fn test_sram0(&mut self) -> TestSram0W<'_, TestSpec> {
        TestSram0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Selects the different test modes for GLXMP_0 RAM"]
    #[inline(always)]
    pub fn test_sram1(&mut self) -> TestSram1W<'_, TestSpec> {
        TestSram1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Selects the different test modes for GLXMP_1 RAM"]
    #[inline(always)]
    pub fn test_sram2(&mut self) -> TestSram2W<'_, TestSpec> {
        TestSram2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Selects the different test modes for UGLXMP_2 RAM"]
    #[inline(always)]
    pub fn test_sram3(&mut self) -> TestSram3W<'_, TestSpec> {
        TestSram3W::new(self, 6)
    }
}
#[doc = "RAM TEST Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
