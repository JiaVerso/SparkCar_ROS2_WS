cmake_minimum_required(VERSION 3.8)
project(target_detector)

if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(OpenCV REQUIRED COMPONENTS core highgui imgproc videoio)
find_package(realsense2 REQUIRED)

execute_process(
  COMMAND python3 -c "import os, torch; print(os.path.dirname(torch.__file__))"
  RESULT_VARIABLE TORCH_PREFIX_RESULT
  OUTPUT_VARIABLE TORCH_ROOT
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
if(NOT TORCH_PREFIX_RESULT EQUAL 0)
  message(FATAL_ERROR "Python package 'torch' is required to build target_detector")
endif()

find_path(TORCH_INCLUDE_DIR torch/script.h
  PATHS "${TORCH_ROOT}/include"
  NO_DEFAULT_PATH
  REQUIRED
)
find_library(TORCH_LIBRARY torch
  PATHS "${TORCH_ROOT}/lib"
  NO_DEFAULT_PATH
  REQUIRED
)
find_library(TORCH_CPU_LIBRARY torch_cpu
  PATHS "${TORCH_ROOT}/lib"
  NO_DEFAULT_PATH
  REQUIRED
)
find_library(C10_LIBRARY c10
  PATHS "${TORCH_ROOT}/lib"
  NO_DEFAULT_PATH
  REQUIRED
)

add_executable(yolo_usb_detector src/yolo_usb_detector.cpp)
target_compile_features(yolo_usb_detector PUBLIC cxx_std_20)
target_include_directories(yolo_usb_detector PRIVATE ${OpenCV_INCLUDE_DIRS})
target_include_directories(yolo_usb_detector SYSTEM PRIVATE
  ${TORCH_INCLUDE_DIR}
  "${TORCH_ROOT}/include/torch/csrc/api/include"
)
ament_target_dependencies(yolo_usb_detector rclcpp geometry_msgs)
target_link_libraries(yolo_usb_detector
  ${OpenCV_LIBRARIES}
  ${TORCH_LIBRARY}
  ${TORCH_CPU_LIBRARY}
  ${C10_LIBRARY}
  realsense2::realsense2
)

set_target_properties(yolo_usb_detector PROPERTIES
  BUILD_RPATH "${TORCH_ROOT}/lib"
  INSTALL_RPATH "${TORCH_ROOT}/lib"
)

install(TARGETS yolo_usb_detector
  DESTINATION lib/${PROJECT_NAME}
)

install(DIRECTORY launch/
  DESTINATION share/${PROJECT_NAME}/launch
  FILES_MATCHING PATTERN "*.launch.py"
)

install(DIRECTORY config/
  DESTINATION share/${PROJECT_NAME}/config
  FILES_MATCHING PATTERN "*.yaml"
)

install(DIRECTORY models/
  DESTINATION share/${PROJECT_NAME}/models
  FILES_MATCHING
    PATTERN "*.torchscript"
    PATTERN "README.md"
)

install(FILES README.md
  DESTINATION share/${PROJECT_NAME}
)

ament_package()
