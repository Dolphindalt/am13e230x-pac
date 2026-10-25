#[doc = "Register `INTERNALTESTCTL` reader"]
pub type R = crate::R<InternaltestctlSpec>;
#[doc = "Register `INTERNALTESTCTL` writer"]
pub type W = crate::W<InternaltestctlSpec>;
#[doc = "Test Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Testsel {
    #[doc = "1: VDDCORE"]
    Sel1 = 1,
    #[doc = "2: VDDA"]
    Sel2 = 2,
    #[doc = "3: VSSA"]
    Sel3 = 3,
    #[doc = "4: VREFLOAC"]
    Sel4 = 4,
    #[doc = "5: CDAC1H"]
    Sel5 = 5,
    #[doc = "6: CDAC1L"]
    Sel6 = 6,
    #[doc = "7: CDAC2H"]
    Sel7 = 7,
    #[doc = "8: CDAC2H"]
    Sel8 = 8,
    #[doc = "9: CDAC2H"]
    Sel9 = 9,
    #[doc = "10: CDAC2H"]
    Sel10 = 10,
    #[doc = "11: CDAC2H"]
    Sel11 = 11,
    #[doc = "12: CDAC2H"]
    Sel12 = 12,
    #[doc = "29: ENZ_CALIB_GAIN_3P3V will be made low. ADCA and ADCC will be in gain calibration mode, and 0.9xVREFHIAB pin voltage will be sampled by both ADCs through internal test-mux output"]
    Sel14 = 29,
    #[doc = "30: CMPSS1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    Sel15 = 30,
    #[doc = "31: ADCA VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    Sel16 = 31,
    #[doc = "32: COMP DAC BUFFER VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    Sel17 = 32,
    #[doc = "33: PGA1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    Sel18 = 33,
    #[doc = "34: ADCCIO_TESTANA0_INT"]
    Sel19 = 34,
    #[doc = "35: PMM/HPLL/INTOSC TESTANA0_INT"]
    Sel20 = 35,
    #[doc = "36: ADCCIO_TESTANA1_INT"]
    Sel21 = 36,
    #[doc = "37: PMM/HPLL/INTOSC TESTANA1_INT"]
    Sel22 = 37,
    #[doc = "38: Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 38: R=10k, 39: R=35k"]
    Sel23 = 38,
    #[doc = "41: USB_TESTANA0_INT"]
    Sel24 = 41,
    #[doc = "42: USB_TESTANA1_INT"]
    Sel25 = 42,
    #[doc = "43: USB_TESTANA0_INT & USB_TESTANA1_INT"]
    Sel26 = 43,
    #[doc = "44: VSS"]
    Sel27 = 44,
    #[doc = "45: Bring FLT3 & TESTPAD3 of flash on TESTANA1"]
    Sel28 = 45,
    #[doc = "46: Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 47: R=10k, 48: R=35k"]
    Sel29 = 46,
    #[doc = "49: VREFLOAC"]
    Sel30 = 49,
}
impl From<Testsel> for u8 {
    #[inline(always)]
    fn from(variant: Testsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Testsel {
    type Ux = u8;
}
impl crate::IsEnum for Testsel {}
#[doc = "Field `TESTSEL` reader - Test Select"]
pub type TestselR = crate::FieldReader<Testsel>;
impl TestselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Testsel> {
        match self.bits {
            1 => Some(Testsel::Sel1),
            2 => Some(Testsel::Sel2),
            3 => Some(Testsel::Sel3),
            4 => Some(Testsel::Sel4),
            5 => Some(Testsel::Sel5),
            6 => Some(Testsel::Sel6),
            7 => Some(Testsel::Sel7),
            8 => Some(Testsel::Sel8),
            9 => Some(Testsel::Sel9),
            10 => Some(Testsel::Sel10),
            11 => Some(Testsel::Sel11),
            12 => Some(Testsel::Sel12),
            29 => Some(Testsel::Sel14),
            30 => Some(Testsel::Sel15),
            31 => Some(Testsel::Sel16),
            32 => Some(Testsel::Sel17),
            33 => Some(Testsel::Sel18),
            34 => Some(Testsel::Sel19),
            35 => Some(Testsel::Sel20),
            36 => Some(Testsel::Sel21),
            37 => Some(Testsel::Sel22),
            38 => Some(Testsel::Sel23),
            41 => Some(Testsel::Sel24),
            42 => Some(Testsel::Sel25),
            43 => Some(Testsel::Sel26),
            44 => Some(Testsel::Sel27),
            45 => Some(Testsel::Sel28),
            46 => Some(Testsel::Sel29),
            49 => Some(Testsel::Sel30),
            _ => None,
        }
    }
    #[doc = "VDDCORE"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == Testsel::Sel1
    }
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == Testsel::Sel2
    }
    #[doc = "VSSA"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == Testsel::Sel3
    }
    #[doc = "VREFLOAC"]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == Testsel::Sel4
    }
    #[doc = "CDAC1H"]
    #[inline(always)]
    pub fn is_sel5(&self) -> bool {
        *self == Testsel::Sel5
    }
    #[doc = "CDAC1L"]
    #[inline(always)]
    pub fn is_sel6(&self) -> bool {
        *self == Testsel::Sel6
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel7(&self) -> bool {
        *self == Testsel::Sel7
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel8(&self) -> bool {
        *self == Testsel::Sel8
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel9(&self) -> bool {
        *self == Testsel::Sel9
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel10(&self) -> bool {
        *self == Testsel::Sel10
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel11(&self) -> bool {
        *self == Testsel::Sel11
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn is_sel12(&self) -> bool {
        *self == Testsel::Sel12
    }
    #[doc = "ENZ_CALIB_GAIN_3P3V will be made low. ADCA and ADCC will be in gain calibration mode, and 0.9xVREFHIAB pin voltage will be sampled by both ADCs through internal test-mux output"]
    #[inline(always)]
    pub fn is_sel14(&self) -> bool {
        *self == Testsel::Sel14
    }
    #[doc = "CMPSS1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn is_sel15(&self) -> bool {
        *self == Testsel::Sel15
    }
    #[doc = "ADCA VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn is_sel16(&self) -> bool {
        *self == Testsel::Sel16
    }
    #[doc = "COMP DAC BUFFER VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn is_sel17(&self) -> bool {
        *self == Testsel::Sel17
    }
    #[doc = "PGA1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn is_sel18(&self) -> bool {
        *self == Testsel::Sel18
    }
    #[doc = "ADCCIO_TESTANA0_INT"]
    #[inline(always)]
    pub fn is_sel19(&self) -> bool {
        *self == Testsel::Sel19
    }
    #[doc = "PMM/HPLL/INTOSC TESTANA0_INT"]
    #[inline(always)]
    pub fn is_sel20(&self) -> bool {
        *self == Testsel::Sel20
    }
    #[doc = "ADCCIO_TESTANA1_INT"]
    #[inline(always)]
    pub fn is_sel21(&self) -> bool {
        *self == Testsel::Sel21
    }
    #[doc = "PMM/HPLL/INTOSC TESTANA1_INT"]
    #[inline(always)]
    pub fn is_sel22(&self) -> bool {
        *self == Testsel::Sel22
    }
    #[doc = "Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 38: R=10k, 39: R=35k"]
    #[inline(always)]
    pub fn is_sel23(&self) -> bool {
        *self == Testsel::Sel23
    }
    #[doc = "USB_TESTANA0_INT"]
    #[inline(always)]
    pub fn is_sel24(&self) -> bool {
        *self == Testsel::Sel24
    }
    #[doc = "USB_TESTANA1_INT"]
    #[inline(always)]
    pub fn is_sel25(&self) -> bool {
        *self == Testsel::Sel25
    }
    #[doc = "USB_TESTANA0_INT & USB_TESTANA1_INT"]
    #[inline(always)]
    pub fn is_sel26(&self) -> bool {
        *self == Testsel::Sel26
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn is_sel27(&self) -> bool {
        *self == Testsel::Sel27
    }
    #[doc = "Bring FLT3 & TESTPAD3 of flash on TESTANA1"]
    #[inline(always)]
    pub fn is_sel28(&self) -> bool {
        *self == Testsel::Sel28
    }
    #[doc = "Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 47: R=10k, 48: R=35k"]
    #[inline(always)]
    pub fn is_sel29(&self) -> bool {
        *self == Testsel::Sel29
    }
    #[doc = "VREFLOAC"]
    #[inline(always)]
    pub fn is_sel30(&self) -> bool {
        *self == Testsel::Sel30
    }
}
#[doc = "Field `TESTSEL` writer - Test Select"]
pub type TestselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Testsel, crate::Safe>;
impl<'a, REG> TestselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDDCORE"]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel1)
    }
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel2)
    }
    #[doc = "VSSA"]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel3)
    }
    #[doc = "VREFLOAC"]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel4)
    }
    #[doc = "CDAC1H"]
    #[inline(always)]
    pub fn sel5(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel5)
    }
    #[doc = "CDAC1L"]
    #[inline(always)]
    pub fn sel6(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel6)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel7(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel7)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel8(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel8)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel9(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel9)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel10(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel10)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel11(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel11)
    }
    #[doc = "CDAC2H"]
    #[inline(always)]
    pub fn sel12(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel12)
    }
    #[doc = "ENZ_CALIB_GAIN_3P3V will be made low. ADCA and ADCC will be in gain calibration mode, and 0.9xVREFHIAB pin voltage will be sampled by both ADCs through internal test-mux output"]
    #[inline(always)]
    pub fn sel14(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel14)
    }
    #[doc = "CMPSS1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn sel15(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel15)
    }
    #[doc = "ADCA VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn sel16(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel16)
    }
    #[doc = "COMP DAC BUFFER VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn sel17(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel17)
    }
    #[doc = "PGA1 VDDA sense on TESTANA0,VSSA sense on TESTANA1"]
    #[inline(always)]
    pub fn sel18(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel18)
    }
    #[doc = "ADCCIO_TESTANA0_INT"]
    #[inline(always)]
    pub fn sel19(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel19)
    }
    #[doc = "PMM/HPLL/INTOSC TESTANA0_INT"]
    #[inline(always)]
    pub fn sel20(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel20)
    }
    #[doc = "ADCCIO_TESTANA1_INT"]
    #[inline(always)]
    pub fn sel21(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel21)
    }
    #[doc = "PMM/HPLL/INTOSC TESTANA1_INT"]
    #[inline(always)]
    pub fn sel22(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel22)
    }
    #[doc = "Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 38: R=10k, 39: R=35k"]
    #[inline(always)]
    pub fn sel23(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel23)
    }
    #[doc = "USB_TESTANA0_INT"]
    #[inline(always)]
    pub fn sel24(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel24)
    }
    #[doc = "USB_TESTANA1_INT"]
    #[inline(always)]
    pub fn sel25(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel25)
    }
    #[doc = "USB_TESTANA0_INT & USB_TESTANA1_INT"]
    #[inline(always)]
    pub fn sel26(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel26)
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn sel27(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel27)
    }
    #[doc = "Bring FLT3 & TESTPAD3 of flash on TESTANA1"]
    #[inline(always)]
    pub fn sel28(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel28)
    }
    #[doc = "Enable resistor for I2V conversion. The same control enables the sampling of voltage across a resistor by ADC. R=2.5k, 47: R=10k, 48: R=35k"]
    #[inline(always)]
    pub fn sel29(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel29)
    }
    #[doc = "VREFLOAC"]
    #[inline(always)]
    pub fn sel30(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::Sel30)
    }
}
impl R {
    #[doc = "Bits 0:5 - Test Select"]
    #[inline(always)]
    pub fn testsel(&self) -> TestselR {
        TestselR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNALTESTCTL")
            .field("testsel", &self.testsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Test Select"]
    #[inline(always)]
    pub fn testsel(&mut self) -> TestselW<'_, InternaltestctlSpec> {
        TestselW::new(self, 0)
    }
}
#[doc = "Internal Test Node Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`internaltestctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internaltestctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InternaltestctlSpec;
impl crate::RegisterSpec for InternaltestctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internaltestctl::R`](R) reader structure"]
impl crate::Readable for InternaltestctlSpec {}
#[doc = "`write(|w| ..)` method takes [`internaltestctl::W`](W) writer structure"]
impl crate::Writable for InternaltestctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTERNALTESTCTL to value 0"]
impl crate::Resettable for InternaltestctlSpec {}
