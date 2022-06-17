use core::ops::Range;

use crate::{pac, peripheral_id::get_peripheral_id};

pub struct Spu {
    spu: pac::SPU_S,
}

impl Spu {
    pub fn new(spu: pac::SPU_S) -> Self {
        Self { spu }
    }

    pub fn set_peripheral_permissions<T>(
        &mut self,
        peripheral: *const T,
        secure: bool,
        dma_secure: bool,
        lock: bool,
    ) {
        self.spu.periphid[get_peripheral_id(peripheral)]
            .perm
            .write(|w| {
                w.secattr()
                    .bit(secure)
                    .dmasec()
                    .bit(dma_secure)
                    .lock()
                    .bit(lock)
            });
    }

    pub fn set_gpio_pin_permissions(&mut self, port: usize, pin: u32, secure: bool) {
        let pin_bit_value = (secure as u32) << pin;
        self.spu.gpioport[port]
            .perm
            .modify(|r, w| unsafe { w.bits((r.bits() & !pin_bit_value) | pin_bit_value) });
    }

    pub fn set_gpio_pin_permissions_all(&mut self, port: usize, secure: bool) {
        self.spu.gpioport[port]
            .perm
            .write(|w| unsafe { w.bits(if secure { u32::MAX } else { 0 }) });
    }

    const RAM_START: u32 = 0x2000_0000;
    const RAM_REGION_SIZE: u32 = 0x0000_2000;
    const RAM_REGION_COUNT: u32 = 64;

    pub fn set_ram_region_permissions(
        &mut self,
        address_range: Range<u32>,
        execute: bool,
        write: bool,
        read: bool,
        secure: bool,
        lock: bool,
    ) {
        for address in address_range.step_by(Self::RAM_REGION_SIZE as _) {
            assert!(address >= Self::RAM_START);
            assert!(address < Self::RAM_START + Self::RAM_REGION_SIZE * Self::RAM_REGION_COUNT);
            assert!(address % Self::RAM_REGION_SIZE == 0);

            let region_index = (address - Self::RAM_START) / Self::RAM_REGION_SIZE;

            self.spu.ramregion[region_index as usize].perm.write(|w| {
                w.execute()
                    .bit(execute)
                    .write()
                    .bit(write)
                    .read()
                    .bit(read)
                    .secattr()
                    .bit(secure)
                    .lock()
                    .bit(lock)
            });
        }
    }
}
