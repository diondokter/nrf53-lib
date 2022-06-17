const ID_MASK: u32 = 0x000F_F000;

pub fn get_peripheral_id<T>(peripheral_pointer: *const T) -> usize {
    (((peripheral_pointer as u32) & ID_MASK) >> ID_MASK.trailing_zeros()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peripheral_ids() {
        assert_eq!(get_peripheral_id(nrf5340_app_pac::P0_NS::PTR), 66);
        assert_eq!(get_peripheral_id(nrf5340_app_pac::P1_NS::PTR), 66);
        assert_eq!(get_peripheral_id(nrf5340_app_pac::P0_S::PTR), 66);
        assert_eq!(get_peripheral_id(nrf5340_app_pac::P1_S::PTR), 66);

        assert_eq!(get_peripheral_id(nrf5340_app_pac::RTC0_NS::PTR), 20);
        assert_eq!(get_peripheral_id(nrf5340_app_pac::SPIS1_S::PTR), 9);
    }
}
