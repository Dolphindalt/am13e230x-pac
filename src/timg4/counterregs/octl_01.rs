#[doc = "Register `OCTL_01[%s]` reader"]
pub type R = crate::R<Octl01Spec>;
#[doc = "Register `OCTL_01[%s]` writer"]
pub type W = crate::W<Octl01Spec>;
#[doc = "CCP Output Source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccpo {
    #[doc = "0: Signal generator value (for example, PWM, triggered PWM)"]
    Funcval = 0,
    #[doc = "1: Load event"]
    Load = 1,
    #[doc = "2: CCU event or CCD event"]
    Cmpval = 2,
    #[doc = "4: Zero event"]
    Zero = 4,
    #[doc = "5: Capture event"]
    Capcond = 5,
    #[doc = "6: Fault condition"]
    Faultcond = 6,
    #[doc = "8: Mirror CCP of first capture and compare register to other capture compare blocks"]
    Cc0MirrorAll = 8,
    #[doc = "9: Mirror CCP of second capture and compare register in other capture compare blocks"]
    Cc1MirrorAll = 9,
    #[doc = "12: Signal generator output after deadband insertion"]
    Deadband = 12,
    #[doc = "13: Counter direction"]
    Cntdir = 13,
}
impl From<Ccpo> for u8 {
    #[inline(always)]
    fn from(variant: Ccpo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccpo {
    type Ux = u8;
}
impl crate::IsEnum for Ccpo {}
#[doc = "Field `CCPO` reader - CCP Output Source"]
pub type CcpoR = crate::FieldReader<Ccpo>;
impl CcpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccpo> {
        match self.bits {
            0 => Some(Ccpo::Funcval),
            1 => Some(Ccpo::Load),
            2 => Some(Ccpo::Cmpval),
            4 => Some(Ccpo::Zero),
            5 => Some(Ccpo::Capcond),
            6 => Some(Ccpo::Faultcond),
            8 => Some(Ccpo::Cc0MirrorAll),
            9 => Some(Ccpo::Cc1MirrorAll),
            12 => Some(Ccpo::Deadband),
            13 => Some(Ccpo::Cntdir),
            _ => None,
        }
    }
    #[doc = "Signal generator value (for example, PWM, triggered PWM)"]
    #[inline(always)]
    pub fn is_funcval(&self) -> bool {
        *self == Ccpo::Funcval
    }
    #[doc = "Load event"]
    #[inline(always)]
    pub fn is_load(&self) -> bool {
        *self == Ccpo::Load
    }
    #[doc = "CCU event or CCD event"]
    #[inline(always)]
    pub fn is_cmpval(&self) -> bool {
        *self == Ccpo::Cmpval
    }
    #[doc = "Zero event"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Ccpo::Zero
    }
    #[doc = "Capture event"]
    #[inline(always)]
    pub fn is_capcond(&self) -> bool {
        *self == Ccpo::Capcond
    }
    #[doc = "Fault condition"]
    #[inline(always)]
    pub fn is_faultcond(&self) -> bool {
        *self == Ccpo::Faultcond
    }
    #[doc = "Mirror CCP of first capture and compare register to other capture compare blocks"]
    #[inline(always)]
    pub fn is_cc0_mirror_all(&self) -> bool {
        *self == Ccpo::Cc0MirrorAll
    }
    #[doc = "Mirror CCP of second capture and compare register in other capture compare blocks"]
    #[inline(always)]
    pub fn is_cc1_mirror_all(&self) -> bool {
        *self == Ccpo::Cc1MirrorAll
    }
    #[doc = "Signal generator output after deadband insertion"]
    #[inline(always)]
    pub fn is_deadband(&self) -> bool {
        *self == Ccpo::Deadband
    }
    #[doc = "Counter direction"]
    #[inline(always)]
    pub fn is_cntdir(&self) -> bool {
        *self == Ccpo::Cntdir
    }
}
#[doc = "Field `CCPO` writer - CCP Output Source"]
pub type CcpoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccpo, crate::Safe>;
impl<'a, REG> CcpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal generator value (for example, PWM, triggered PWM)"]
    #[inline(always)]
    pub fn funcval(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Funcval)
    }
    #[doc = "Load event"]
    #[inline(always)]
    pub fn load(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Load)
    }
    #[doc = "CCU event or CCD event"]
    #[inline(always)]
    pub fn cmpval(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Cmpval)
    }
    #[doc = "Zero event"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Zero)
    }
    #[doc = "Capture event"]
    #[inline(always)]
    pub fn capcond(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Capcond)
    }
    #[doc = "Fault condition"]
    #[inline(always)]
    pub fn faultcond(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Faultcond)
    }
    #[doc = "Mirror CCP of first capture and compare register to other capture compare blocks"]
    #[inline(always)]
    pub fn cc0_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Cc0MirrorAll)
    }
    #[doc = "Mirror CCP of second capture and compare register in other capture compare blocks"]
    #[inline(always)]
    pub fn cc1_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Cc1MirrorAll)
    }
    #[doc = "Signal generator output after deadband insertion"]
    #[inline(always)]
    pub fn deadband(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Deadband)
    }
    #[doc = "Counter direction"]
    #[inline(always)]
    pub fn cntdir(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpo::Cntdir)
    }
}
#[doc = "CCP Output Invert The output as selected by CCPO is conditionally inverted.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpoinv {
    #[doc = "0: No inversion"]
    Noinv = 0,
    #[doc = "1: Invert"]
    Inv = 1,
}
impl From<Ccpoinv> for bool {
    #[inline(always)]
    fn from(variant: Ccpoinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPOINV` reader - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type CcpoinvR = crate::BitReader<Ccpoinv>;
impl CcpoinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpoinv {
        match self.bits {
            false => Ccpoinv::Noinv,
            true => Ccpoinv::Inv,
        }
    }
    #[doc = "No inversion"]
    #[inline(always)]
    pub fn is_noinv(&self) -> bool {
        *self == Ccpoinv::Noinv
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Ccpoinv::Inv
    }
}
#[doc = "Field `CCPOINV` writer - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type CcpoinvW<'a, REG> = crate::BitWriter<'a, REG, Ccpoinv>;
impl<'a, REG> CcpoinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No inversion"]
    #[inline(always)]
    pub fn noinv(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpoinv::Noinv)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpoinv::Inv)
    }
}
#[doc = "CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpiv {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: High"]
    High = 1,
}
impl From<Ccpiv> for bool {
    #[inline(always)]
    fn from(variant: Ccpiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPIV` reader - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type CcpivR = crate::BitReader<Ccpiv>;
impl CcpivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpiv {
        match self.bits {
            false => Ccpiv::Low,
            true => Ccpiv::High,
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ccpiv::Low
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ccpiv::High
    }
}
#[doc = "Field `CCPIV` writer - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type CcpivW<'a, REG> = crate::BitWriter<'a, REG, Ccpiv>;
impl<'a, REG> CcpivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpiv::Low)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpiv::High)
    }
}
impl R {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn ccpo(&self) -> CcpoR {
        CcpoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn ccpoinv(&self) -> CcpoinvR {
        CcpoinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn ccpiv(&self) -> CcpivR {
        CcpivR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCTL_01")
            .field("ccpoinv", &self.ccpoinv())
            .field("ccpiv", &self.ccpiv())
            .field("ccpo", &self.ccpo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn ccpo(&mut self) -> CcpoW<'_, Octl01Spec> {
        CcpoW::new(self, 0)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn ccpoinv(&mut self) -> CcpoinvW<'_, Octl01Spec> {
        CcpoinvW::new(self, 4)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn ccpiv(&mut self) -> CcpivW<'_, Octl01Spec> {
        CcpivW::new(self, 5)
    }
}
#[doc = "CCP Output Control Registers 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Octl01Spec;
impl crate::RegisterSpec for Octl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl_01::R`](R) reader structure"]
impl crate::Readable for Octl01Spec {}
#[doc = "`write(|w| ..)` method takes [`octl_01::W`](W) writer structure"]
impl crate::Writable for Octl01Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OCTL_01[%s] to value 0"]
impl crate::Resettable for Octl01Spec {}
