cmake_minimum_required(VERSION 3.5.0)
project(telegram_weather_bot VERSION 0.1.0 LANGUAGES C)

add_executable(telegram_weather_bot src/main.c)

target_include_directories(telegram_weather_bot PUBLIC include telebot/include)
target_link_directories(telegram_weather_bot PUBLIC lib telebot/Build)
target_link_libraries(telegram_weather_bot telebot weather_info)

include(CTest)
enable_testing()