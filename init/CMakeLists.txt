add_executable(cpico
    main.cpp
    )

target_link_libraries(cpico
    pico_btstack_ble
    pico_btstack_cyw43
    pico_cyw43_arch_none
    pico_multicore
    pico_stdlib
    picotors
    )

target_include_directories(cpico 
    PRIVATE
    . # For btstack config
    )

# target_link_directories(cpico 
#     PRIVATE
#     ${CMAKE_SOURCE_DIR}/app/target/thumbv6m-none-eabi/debug/
#     )



pico_enable_stdio_usb(cpico 1)
pico_enable_stdio_uart(cpico 0)

# pico_btstack_make_gatt_header(cpico PRIVATE "${CMAKE_CURRENT_LIST_DIR}/picoton.gatt")

pico_add_extra_outputs(cpico)