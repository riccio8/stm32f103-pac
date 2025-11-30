#[doc = "Register `DCTRL` reader"]
pub type R = crate::R<DctrlSpec>;
#[doc = "Register `DCTRL` writer"]
pub type W = crate::W<DctrlSpec>;
#[doc = "Field `DTEN` reader - DTEN"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDIR` reader - DTDIR"]
pub type DtdirR = crate::BitReader;
#[doc = "Field `DTDIR` writer - DTDIR"]
pub type DtdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - DTMODE"]
pub type DtmodeR = crate::BitReader;
#[doc = "Field `DTMODE` writer - DTMODE"]
pub type DtmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub type DblocksizeR = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub type DblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWSTART` reader - PWSTART"]
pub type PwstartR = crate::BitReader;
#[doc = "Field `PWSTART` writer - PWSTART"]
pub type PwstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSTOP` reader - PWSTOP"]
pub type PwstopR = crate::BitReader;
#[doc = "Field `PWSTOP` writer - PWSTOP"]
pub type PwstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMOD` reader - RWMOD"]
pub type RwmodR = crate::BitReader;
#[doc = "Field `RWMOD` writer - RWMOD"]
pub type RwmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SdioenR = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SdioenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&self) -> DtdirR {
        DtdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DtmodeR {
        DtmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DblocksizeR {
        DblocksizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&self) -> PwstartR {
        PwstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&self) -> PwstopR {
        PwstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&self) -> RwmodR {
        RwmodR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<'_, DctrlSpec> {
        DtenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DtdirW<'_, DctrlSpec> {
        DtdirW::new(self, 1)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DtmodeW<'_, DctrlSpec> {
        DtmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DctrlSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DblocksizeW<'_, DctrlSpec> {
        DblocksizeW::new(self, 4)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&mut self) -> PwstartW<'_, DctrlSpec> {
        PwstartW::new(self, 8)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&mut self) -> PwstopW<'_, DctrlSpec> {
        PwstopW::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RwmodW<'_, DctrlSpec> {
        RwmodW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SdioenW<'_, DctrlSpec> {
        SdioenW::new(self, 11)
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctrlSpec;
impl crate::RegisterSpec for DctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctrl::R`](R) reader structure"]
impl crate::Readable for DctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctrl::W`](W) writer structure"]
impl crate::Writable for DctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DctrlSpec {}
