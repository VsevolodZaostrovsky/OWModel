project(CurrencyMarketMaking_FeaturesCalculator)

add_custom_target(cargo-build
    COMMAND echo "Compiling data parser using cargo\n"
    COMMAND cargo build --manifest-path ${CMAKE_SOURCE_DIR}/data_parsing/Cargo.toml --release
)

add_custom_target(build ALL)
add_dependencies(build cargo-build)

add_test(cargo-test
    COMMAND cargo test --manifest-path ${CMAKE_SOURCE_DIR}/data_parsing/Cargo.toml
)