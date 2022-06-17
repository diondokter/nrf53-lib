use crate::pac::dcnf_ns;
use core::ops::{Deref, Range};

pub struct Dcnf<DCNF: Deref<Target = dcnf_ns::RegisterBlock>> {
    peripheral: DCNF,
}

impl<DCNF: Deref<Target = dcnf_ns::RegisterBlock>> Dcnf<DCNF> {
    pub fn new(peripheral: DCNF) -> Self {
        Self { peripheral }
    }

    const RAM_START: u32 = 0x2000_0000;
    const RAM_REGION_SIZE: u32 = 0x0001_0000;
    const RAM_REGION_COUNT: u32 = 8;

    pub fn set_net_core_peripheral_access(&mut self, access: bool) {
        self.peripheral.extperi[0].protect.modify(|_, w| {
            w.slave0().variant(if access {
                nrf5340_app_pac::dcnf_ns::extperi::protect::SLAVE0_A::ALLOWED
            } else {
                nrf5340_app_pac::dcnf_ns::extperi::protect::SLAVE0_A::BLOCKED
            })
        })
    }

    pub fn set_net_core_ram_access(&mut self, address_range: Range<u32>, access: bool) {
        for address in address_range.step_by(Self::RAM_REGION_SIZE as _) {
            assert!(address >= Self::RAM_START);
            assert!(address < Self::RAM_START + Self::RAM_REGION_SIZE * Self::RAM_REGION_COUNT);
            assert!(address % Self::RAM_REGION_SIZE == 0);

            let region_index = (address - Self::RAM_START) / Self::RAM_REGION_SIZE;

            self.peripheral.extram[0].protect.modify(|r, w| unsafe {
                w.bits(r.bits() & !(0x01 << region_index) | ((!access as u32) << region_index))
            });
        }
    }

    pub fn set_net_core_flash_access(&mut self, access: bool) {
        self.peripheral.extcode[0].protect.modify(|_, w| {
            w.slave0().variant(if access {
                nrf5340_app_pac::dcnf_ns::extcode::protect::SLAVE0_A::ALLOWED
            } else {
                nrf5340_app_pac::dcnf_ns::extcode::protect::SLAVE0_A::BLOCKED
            })
        })
    }
}
