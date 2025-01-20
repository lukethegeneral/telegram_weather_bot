cmake_minimum_required(VERSION 3.5.0)
project(telegram_weather_bot VERSION 0.1.0 LANGUAGES C)

add_executable(telegram_weather_bot src/main.c)

add_custom_target(weather_infoX COMMAND cargo build WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/weather_info)
add_dependencies(telegram_weather_bot weather_infoX)

#target_link_directories(telegram_weather_bot PUBLIC lib telebot/Build)
target_link_directories(telegram_weather_bot PUBLIC weather_info/target/debug)
#target_link_libraries(telegram_weather_bot telebot weather_info)
target_link_libraries(telegram_weather_bot weather_info)

target_include_directories(telegram_weather_bot PUBLIC include telebot/include)
target_link_directories(telegram_weather_bot PUBLIC telebot/Build)
target_link_libraries(telegram_weather_bot telebot)

#include(FetchContent)
#
#FetchContent_Declare(
#    Corrosion
#    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
#    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
#)
#FetchContent_MakeAvailable(Corrosion)
#
## Import targets defined in a package or workspace manifest `Cargo.toml` file
#corrosion_import_crate(
#	MANIFEST_PATH weather_info/Cargo.toml
#	#PROFILE release
#	PROFILE dev
#)
#target_link_libraries(telegram_weather_bot weather_info)

include(CTest)
enable_testing()