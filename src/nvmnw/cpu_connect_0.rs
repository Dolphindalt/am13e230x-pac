#[doc = "Register `CPU_CONNECT_0` reader"]
pub type R = crate::R<CpuConnect0Spec>;
#[doc = "Register `CPU_CONNECT_0` writer"]
pub type W = crate::W<CpuConnect0Spec>;
#[doc = "CPUSS0 connect bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpuss0Conn {
    #[doc = "0: The CPU is not connected."]
    Unconnected = 0,
    #[doc = "1: The CPU is connected."]
    Connected = 1,
}
impl From<Cpuss0Conn> for bool {
    #[inline(always)]
    fn from(variant: Cpuss0Conn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUSS0_CONN` reader - CPUSS0 connect bit."]
pub type Cpuss0ConnR = crate::BitReader<Cpuss0Conn>;
impl Cpuss0ConnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpuss0Conn {
        match self.bits {
            false => Cpuss0Conn::Unconnected,
            true => Cpuss0Conn::Connected,
        }
    }
    #[doc = "The CPU is not connected."]
    #[inline(always)]
    pub fn is_unconnected(&self) -> bool {
        *self == Cpuss0Conn::Unconnected
    }
    #[doc = "The CPU is connected."]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Cpuss0Conn::Connected
    }
}
#[doc = "Field `CPUSS0_CONN` writer - CPUSS0 connect bit."]
pub type Cpuss0ConnW<'a, REG> = crate::BitWriter<'a, REG, Cpuss0Conn>;
impl<'a, REG> Cpuss0ConnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CPU is not connected."]
    #[inline(always)]
    pub fn unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Cpuss0Conn::Unconnected)
    }
    #[doc = "The CPU is connected."]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Cpuss0Conn::Connected)
    }
}
impl R {
    #[doc = "Bit 1 - CPUSS0 connect bit."]
    #[inline(always)]
    pub fn cpuss0_conn(&self) -> Cpuss0ConnR {
        Cpuss0ConnR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_CONNECT_0")
            .field("cpuss0_conn", &self.cpuss0_conn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - CPUSS0 connect bit."]
    #[inline(always)]
    pub fn cpuss0_conn(&mut self) -> Cpuss0ConnW<'_, CpuConnect0Spec> {
        Cpuss0ConnW::new(self, 1)
    }
}
#[doc = "CPU Connect\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_connect_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_connect_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuConnect0Spec;
impl crate::RegisterSpec for CpuConnect0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_connect_0::R`](R) reader structure"]
impl crate::Readable for CpuConnect0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_connect_0::W`](W) writer structure"]
impl crate::Writable for CpuConnect0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CONNECT_0 to value 0"]
impl crate::Resettable for CpuConnect0Spec {}
