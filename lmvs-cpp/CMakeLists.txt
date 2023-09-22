cmake_minimum_required(VERSION 3.4.0)
project(
        LMVS_CPP 
        VERSION 1.0.0
        DESCRIPTION "An example project with CMake"
        LANGUAGES CXX )
add_subdirectory(src)




install(FILES 
    ${CMAKE_CURRENT_SOURCE_DIR}/include/lmvs/add.h
    ${CMAKE_CURRENT_SOURCE_DIR}/include/lmvs/div.h
    DESTINATION 
    ${CMAKE_INSTALL_PREFIX}/include/lmvs 
)
install(TARGETS lmvs EXPORT lmvs DESTINATION ${CMAKE_INSTALL_PREFIX}/lib/lmvs)

#install(EXPORT lmvs FILE math-config.cmake  DESTINATION ${CMAKE_INSTALL_PREFIX}/lib/lmvs)



add_custom_target("uninstall" 
    COMMENT "Uninstall files with install_manifest.txt"
    COMMAND xargs rm -vf < install_manifest.txt 
         && rm -rf ${CMAKE_INSTALL_PREFIX}/include/lmvs
         || echo Nothing in install_manifest.txt to be uninstalled!
)



# default  /usr/local
# Header Files  /usr/local/include/<package_name>
# Targets:      /usr/local/lib/<package_name>
message("CMAKE_INSTALL_PREFIX = ${CMAKE_INSTALL_PREFIX}")