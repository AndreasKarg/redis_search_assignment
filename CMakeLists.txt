cmake_minimum_required(VERSION 4.3)
project(redis_search_assignment C)

set(CMAKE_C_STANDARD 23)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(CPM)

CPMAddPackage(
        URI "gh:corrosion-rs/corrosion@0.6.1"
)

corrosion_import_crate(MANIFEST_PATH trie_rs/Cargo.toml)
corrosion_experimental_cbindgen(
        TARGET trie_rs
        HEADER_NAME trie_rs.h
        FLAGS "--config" "cbindgen.toml"
)

add_executable(redis_search_assignment main.c)
target_link_libraries(redis_search_assignment trie_rs)
