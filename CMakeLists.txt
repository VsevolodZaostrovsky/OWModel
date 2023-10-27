cmake_minimum_required(VERSION 3.11)

project(CurrencyMarketMaking)

option(BUILD_REPORT "Enable building the report" OFF)
option(BUILD_PYTHON "Enable building the optexec module" ON)
option(BUILD_RUST "Enable building the feature calculator" ON)

if(BUILD_REPORT)
    add_subdirectory(report)
endif()

if(BUILD_PYTHON)
    add_subdirectory(optexec)
endif()

if(BUILD_RUST)
    add_subdirectory(data_parsing)
endif()
