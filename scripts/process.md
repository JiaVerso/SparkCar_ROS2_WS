Livox MID360 驱动 ros2 launch livox_ros_driver2 msg_MID360_launch.py
                  ros2 topic hz /livox/lidar
                  ros2 topic hz /livox/imu
-> 启动 ros2 bag record 录包
-> PGO 建图       ros2 launch pgo pgo_launch.py
-> PGO 内部自动启动 FAST-LIO2
-> 可手动保存 map.pcd       mkdir -p /home/jiaverso/Desktop/Save_Map
                            ros2 service call /pgo/save_maps interface/srv/SaveMaps "{file_path: '/home/jiaverso/Desktop/Save_Map', save_patches: true}"
-> 可用 pcd2pgm 转 pgm      source /opt/ros/humble/setup.bash
                            source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash

                            mkdir -p /home/jiaverso/Desktop/bags

                            ros2 bag record \
                            /livox/lidar \
                            /livox/imu \
                            /fastlio2/lio_odom \
                            /fastlio2/body_cloud \
                            -o /home/jiaverso/Desktop/bags/mapping_001
-> 保存地图
-> 停止录包
-> 结束pgo建图