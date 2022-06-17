use crate::pac;

#[cfg(feature = "app")]
pub fn disable_approtect(
    uicr: &mut pac::UICR_S,
    ctrlap: &mut pac::CTRLAP_S,
    nvmc: &mut pac::NVMC_S,
) {
    if !uicr.approtect.read().pall().is_unprotected() {
        nvmc.config.write(|w| w.wen().wen());
        while nvmc.ready.read().bits() == 0 {}
        uicr.approtect.write(|w| w.pall().unprotected());
        while nvmc.ready.read().bits() == 0 {}
        nvmc.config.write(|w| w.wen().ren());
    }

    if !uicr.secureapprotect.read().pall().is_unprotected() {
        nvmc.config.write(|w| w.wen().wen());
        while nvmc.ready.read().bits() == 0 {}
        uicr.secureapprotect.write(|w| w.pall().unprotected());
        while nvmc.ready.read().bits() == 0 {}
        nvmc.config.write(|w| w.wen().ren());
    }

    ctrlap
        .approtect
        .disable
        .write(|w| unsafe { w.bits(0x50FA50FA) });
    ctrlap
        .secureapprotect
        .disable
        .write(|w| unsafe { w.bits(0x50FA50FA) });
}

#[cfg(feature = "net")]
pub fn disable_approtect(
    uicr: &mut pac::UICR_NS,
    ctrlap: &mut pac::CTRLAP_NS,
    nvmc: &mut pac::NVMC_NS,
) {
    if !uicr.approtect.read().pall().is_unprotected() {
        nvmc.config.write(|w| w.wen().wen());
        while nvmc.ready.read().bits() == 0 {}
        uicr.approtect.write(|w| w.pall().unprotected());
        while nvmc.ready.read().bits() == 0 {}
        nvmc.config.write(|w| w.wen().ren());
    }

    ctrlap
        .approtect
        .disable
        .write(|w| unsafe { w.bits(0x50FA50FA) });
}
