project(CurrencyMarketMaking_Optimal_Execution_Module)

find_package(Python COMPONENTS Interpreter REQUIRED)
set(OPTIMAL_EXECUTION_DIR ${CMAKE_SOURCE_DIR}/optexec)

add_test(NAME Execution
    COMMAND ${Python_EXECUTABLE} -m pytest -v optexec/*.py
)