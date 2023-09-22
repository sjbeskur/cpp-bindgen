# Optionally glob, but only for CMake 3.12 or later:

file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${ModernCMakeExample_SOURCE_DIR}/include/lmvs/*.h")
#set(HEADER_LIST "${LMVS_CPP_SOURCE_DIR}/include/lmvs/math.h")

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${LMVS_CPP_SOURCE_DIR}/src/*.cpp")

# Make an automatic library - will be static or dynamic based on user setting
add_library(lmvs ${SOURCE_LIST} ${HEADER_LIST})

# We need this directory, and users of our library will need it too
target_include_directories(lmvs PUBLIC ../include)

# All users of this library will need at least C++11
target_compile_features(lmvs PUBLIC cxx_std_11)

# IDEs should put the headers in a nice place
source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST})
