#![no_std]
use panic_halt as _;

use pico_w_sys as pico_sdk;

static PROF_DATA: [u8; 241] = [
    // ATT DB Version
    1, // 0x0001 PRIMARY_SERVICE-GAP_SERVICE
    0x0a, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x28, 0x00, 0x18,
    // 0x0002 CHARACTERISTIC-GAP_DEVICE_NAME - READ
    0x0d, 0x00, 0x02, 0x00, 0x02, 0x00, 0x03, 0x28, 0x02, 0x03, 0x00, 0x00, 0x2a,
    // 0x0003 VALUE CHARACTERISTIC-GAP_DEVICE_NAME - READ -'Picoton'
    // READ_ANYBODY
    0x0f, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00, 0x2a, 0x50, 0x69, 0x63, 0x6f, 0x74, 0x6f, 0x6e,
    // 0x0004 PRIMARY_SERVICE-GATT_SERVICE
    0x0a, 0x00, 0x02, 0x00, 0x04, 0x00, 0x00, 0x28, 0x01, 0x18,
    // 0x0005 CHARACTERISTIC-GATT_DATABASE_HASH - READ
    0x0d, 0x00, 0x02, 0x00, 0x05, 0x00, 0x03, 0x28, 0x02, 0x06, 0x00, 0x2a, 0x2b,
    // 0x0006 VALUE CHARACTERISTIC-GATT_DATABASE_HASH - READ -''
    // READ_ANYBODY
    0x18, 0x00, 0x02, 0x00, 0x06, 0x00, 0x2a, 0x2b, 0x70, 0xc3, 0x61, 0x96, 0xca, 0x31, 0x6d, 0xe6,
    0x4d, 0xc1, 0xbd, 0x87, 0x7c, 0x76, 0xcd, 0x51,
    // #import <cycling_power_service.gatt> -- BEGIN
    // Specification Type org.bluetooth.service.cycling_power
    // https://www.bluetooth.com/api/gatt/xmlfile?xmlFileName=org.bluetooth.service.cycling_power.xml
    // Cycling Power 1818
    // 0x0007 PRIMARY_SERVICE-ORG_BLUETOOTH_SERVICE_CYCLING_POWER
    0x0a, 0x00, 0x02, 0x00, 0x07, 0x00, 0x00, 0x28, 0x18, 0x18,
    // 0x0008 CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_MEASUREMENT - DYNAMIC | NOTIFY | BROADCAST
    0x0d, 0x00, 0x02, 0x00, 0x08, 0x00, 0x03, 0x28, 0x11, 0x09, 0x00, 0x63, 0x2a,
    // 0x0009 VALUE CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_MEASUREMENT - DYNAMIC | NOTIFY | BROADCAST
    //
    0x08, 0x00, 0x00, 0x01, 0x09, 0x00, 0x63, 0x2a,
    // 0x000a CLIENT_CHARACTERISTIC_CONFIGURATION
    // READ_ANYBODY, WRITE_ANYBODY
    0x0a, 0x00, 0x0e, 0x01, 0x0a, 0x00, 0x02, 0x29, 0x00, 0x00,
    // 0x000b SERVER_CONFIGURATION-READ | WRITE-
    // READ_ANYBODY, WRITE_ANYBODY
    0x08, 0x00, 0x0a, 0x01, 0x0b, 0x00, 0x03, 0x29,
    // 0x000c CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_FEATURE - DYNAMIC | READ
    0x0d, 0x00, 0x02, 0x00, 0x0c, 0x00, 0x03, 0x28, 0x02, 0x0d, 0x00, 0x65, 0x2a,
    // 0x000d VALUE CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_FEATURE - DYNAMIC | READ
    // READ_ANYBODY
    0x08, 0x00, 0x02, 0x01, 0x0d, 0x00, 0x65, 0x2a,
    // 0x000e CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_SENSOR_LOCATION - DYNAMIC | READ
    0x0d, 0x00, 0x02, 0x00, 0x0e, 0x00, 0x03, 0x28, 0x02, 0x0f, 0x00, 0x5d, 0x2a,
    // 0x000f VALUE CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_SENSOR_LOCATION - DYNAMIC | READ
    // READ_ANYBODY
    0x08, 0x00, 0x02, 0x01, 0x0f, 0x00, 0x5d, 0x2a,
    // 0x0010 CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_VECTOR - DYNAMIC | NOTIFY
    0x0d, 0x00, 0x02, 0x00, 0x10, 0x00, 0x03, 0x28, 0x10, 0x11, 0x00, 0x64, 0x2a,
    // 0x0011 VALUE CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_VECTOR - DYNAMIC | NOTIFY
    //
    0x08, 0x00, 0x00, 0x01, 0x11, 0x00, 0x64, 0x2a,
    // 0x0012 CLIENT_CHARACTERISTIC_CONFIGURATION
    // READ_ANYBODY, WRITE_ANYBODY
    0x0a, 0x00, 0x0e, 0x01, 0x12, 0x00, 0x02, 0x29, 0x00, 0x00,
    // 0x0013 CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_CONTROL_POINT - DYNAMIC | WRITE | INDICATE
    0x0d, 0x00, 0x02, 0x00, 0x13, 0x00, 0x03, 0x28, 0x28, 0x14, 0x00, 0x66, 0x2a,
    // 0x0014 VALUE CHARACTERISTIC-ORG_BLUETOOTH_CHARACTERISTIC_CYCLING_POWER_CONTROL_POINT - DYNAMIC | WRITE | INDICATE
    // WRITE_ANYBODY
    0x08, 0x00, 0x08, 0x01, 0x14, 0x00, 0x66, 0x2a,
    // 0x0015 CLIENT_CHARACTERISTIC_CONFIGURATION
    // READ_ANYBODY, WRITE_ANYBODY
    0x0a, 0x00, 0x0e, 0x01, 0x15, 0x00, 0x02, 0x29, 0x00, 0x00,
    // #import <cycling_power_service.gatt> -- END
    // END
    0x00, 0x00,
]; // total size 123 bytes

static mut ADV_DATA: [u8; 18] = [
    0x02, 0x01, 0x06, 0x08, 0x09, 0x50, 0x69, 0x63, 0x6f, 0x74, 0x6f, 0x6e, 0x05, 0x03, 0x18, 0x18,
    0x0a, 0x18,
];

#[no_mangle]
pub extern "C" fn picotors_init() -> bool {
    unsafe {
        pico_sdk::cyw43_arch_init();
        pico_sdk::l2cap_init();
        // Security manager
        pico_sdk::sm_init();

        // pico_sdk::puts(CStr::from("Initializing att server\n").as_ptr());
        pico_sdk::att_server_init(PROF_DATA.as_ptr(), Some(att_read), Some(att_write));

        // setup advertisements
        let adv_int_min = 800; // interval is 800 I guess
        let adv_int_max = 800;
        let adv_type = 0; // who knows what this means

        let mut null_addr: pico_sdk::bd_addr_t = [0; 6];

        pico_sdk::gap_advertisements_set_params(
            adv_int_min,
            adv_int_max,
            adv_type,
            0,
            null_addr.as_mut_ptr(),
            0x07,
            0x00,
        );

        pico_sdk::gap_advertisements_set_data(ADV_DATA.len() as u8, ADV_DATA.as_mut_ptr());
        pico_sdk::gap_advertisements_enable(1);

        pico_sdk::hci_power_control(pico_sdk::HCI_POWER_MODE::HCI_POWER_ON);

        pico_sdk::btstack_run_loop_execute();
    }

    loop {
        unsafe {
            pico_sdk::cyw43_arch_gpio_put(0, true);
            pico_sdk::sleep_ms(100);
            pico_sdk::cyw43_arch_gpio_put(0, false);
            pico_sdk::sleep_ms(100);
        }
    }
}

extern "C" fn att_read(
    con_handle: pico_sdk::hci_con_handle_t,
    attribute_handle: u16,
    offset: u16,
    buffer: *mut u8,
    buffer_size: u16,
) -> u16 {
    0
}

extern "C" fn att_write(
    con_handle: pico_sdk::hci_con_handle_t,
    attribute_handle: u16,
    transaction_mode: u16,
    offset: u16,
    buffer: *mut u8,
    buffer_size: u16,
) -> cty::c_int {
    0
}
