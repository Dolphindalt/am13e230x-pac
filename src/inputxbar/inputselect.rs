#[doc = "Register `INPUTSELECT[%s]` reader"]
pub type R = crate::R<InputselectSpec>;
#[doc = "Register `INPUTSELECT[%s]` writer"]
pub type W = crate::W<InputselectSpec>;
#[doc = "Field `SELECT` reader - Select GPIO for INPUT{#} signal"]
pub type SelectR = crate::FieldReader;
#[doc = "Field `SELECT` writer - Select GPIO for INPUT{#} signal"]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Select GPIO for INPUT{#} signal"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUTSELECT")
            .field("select", &self.select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Select GPIO for INPUT{#} signal"]
    #[inline(always)]
    pub fn select(&mut self) -> SelectW<'_, InputselectSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "Input Select Register (GPIO0 to x)\n\nYou can [`read`](crate::Reg::read) this register and get [`inputselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputselectSpec;
impl crate::RegisterSpec for InputselectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputselect::R`](R) reader structure"]
impl crate::Readable for InputselectSpec {}
#[doc = "`write(|w| ..)` method takes [`inputselect::W`](W) writer structure"]
impl crate::Writable for InputselectSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INPUTSELECT[%s] to value 0xfe"]
impl crate::Resettable for InputselectSpec {
    const RESET_VALUE: u32 = 0xfe;
}
