cmake_minimum_required(VERSION 3.13)

# initialize pico-sdk from submodule
include(sdk/pico_sdk_init.cmake)

project(picoton CXX C ASM)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(PICO_BOARD pico_w)

pico_sdk_init()

add_subdirectory(init)

# Build the Rust code
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.5 # Optionally specify a commit hash, version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

set(Rust_CARGO_TARGET "thumbv6m-none-eabi")


# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH app/Cargo.toml 
    NO_STD 
    )

target_include_directories(
    picotors
    INTERFACE
    app/include
)

# add_custom_target(cargo_build_pictors)

# add_library(picotors STATIC IMPORTED GLOBAL)
# set_target_properties(picotors PROPERTIES IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/app/target/thumbv6m-none-eabi/debug/libpicotors.a")
# set_target_properties(picotors PROPERTIES INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/app/include")

# target_link_libraries(mytarget A boost_unit_test_framework C)