cmake_minimum_required(VERSION 3.5.0)
project(telegram_weather_bot VERSION 0.1.0 LANGUAGES C)

add_executable(telegram_weather_bot src/main.c)

add_custom_target(weather_infoX COMMAND cargo build WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/weather_info)
add_dependencies(telegram_weather_bot weather_infoX)

target_link_directories(telegram_weather_bot PUBLIC weather_info/target/debug)
target_link_libraries(telegram_weather_bot weather_info)

target_include_directories(telegram_weather_bot PUBLIC include telebot/include)
target_link_directories(telegram_weather_bot PUBLIC telebot/Build)
target_link_libraries(telegram_weather_bot telebot)

include(CTest)
enable_testing()