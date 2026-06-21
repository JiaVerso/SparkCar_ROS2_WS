/*
* 状态：x, y
* 扩展：八邻域
* 碰撞：检查一个栅格点
* 路径：栅格折线
*
*/
#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <functional>
#include <memory>
#include <queue>
#include <unordered_map>
#include <vector>

#include "geometry_msgs/msg/point.hpp"
#include "geometry_msgs/msg/pose_stamped.hpp"
#include "geometry_msgs/msg/pose_with_covariance_stamped.hpp"
#include "nav_msgs/msg/occupancy_grid.hpp"
#include "nav_msgs/msg/path.hpp"
#include "rclcpp/rclcpp.hpp"
#include "visualization_msgs/msg/marker.hpp"

namespace hybrid_astar_planner
{

// 栅格坐标点，表示地图中的一个离散格子。
// 普通 A* 只需要 x、y；Hybrid A* 后续会扩展 yaw。
struct GridIndex
{
  int x{0};
  int y{0};

  // 重载 == 运算符，用于判断两个栅格坐标是否相同。
  bool operator==(const GridIndex & other) const
  {
    return x == other.x && y == other.y;
  }
};

// 哈希函数，让 GridIndex 可以作为 unordered_map 的 key。
struct GridIndexHash
{
  // 重载 () 运算符，使 GridIndexHash 对象可以像函数一样被调用。
  std::size_t operator()(const GridIndex & index) const
  {
    const auto x_hash = std::hash<int>{}(index.x);
    const auto y_hash = std::hash<int>{}(index.y);
    // 将 x 和 y 的哈希值组合，减少哈希冲突。
    return x_hash ^ (y_hash << 1);
  }
};

// 优先队列中的节点数据，保存栅格坐标和 A* 优先级 f = g + h。
struct PriorityQueueItem
{
  GridIndex index;
  double priority{0.0};

  // std::priority_queue 默认是大顶堆。
  // 这里反向比较，让 priority 数值小的节点优先弹出。
  bool operator<(const PriorityQueueItem & other) const
  {
    return priority > other.priority;
  }
};

// 二维栅格地图，负责存储障碍物并提供邻居查询、代价计算和坐标转换。
class GridMap
{
public:
  // 构造函数：width 和 height 是格子数量，resolution 是每个格子的实际尺寸。
  GridMap(int width, int height, double resolution)
  : width_(width), height_(height), resolution_(resolution)
  {
    // occupancy_ 使用一维数组保存二维地图，false 表示空闲，true 表示障碍物。
    occupancy_.assign(static_cast<std::size_t>(width_ * height_), false);
  }

  // 设置某个栅格为障碍物。
  void setObstacle(int x, int y)
  {
    const GridIndex index{x, y};
    if (isInside(index)) {
      occupancy_[toLinearIndex(index)] = true;
    }
  }

  // 判断栅格是否位于地图范围内。
  bool isInside(const GridIndex & index) const
  {
    return index.x >= 0 && index.x < width_ && index.y >= 0 && index.y < height_;
  }

  // 判断栅格是否为障碍物。地图外部直接视为障碍物，防止路径越界。
  bool isObstacle(const GridIndex & index) const
  {
    if (!isInside(index)) {
      return true;
    }
    return occupancy_[toLinearIndex(index)];
  }

  // const 代表函数里面不能修改类成员变量值
  int width() const
  {
    return width_;
  }

  int height() const
  {
    return height_;
  }

  double resolution() const
  {
    return resolution_;
  }

  // 查询当前栅格周围可通行的邻居节点。
  std::vector<GridIndex> neighbors(const GridIndex & current) const
  {
    // 八邻域搜索：上下左右和四个斜向都允许移动。
    // 如果要改成四邻域，只保留前四个方向即可。
    static constexpr std::array<GridIndex, 8> directions{{
      {1, 0}, {-1, 0}, {0, 1}, {0, -1},
      {1, 1}, {1, -1}, {-1, 1}, {-1, -1}
    }};

    std::vector<GridIndex> result;
    // 最多只有 8 个邻居，提前分配空间，避免 vector 扩容。
    result.reserve(directions.size());

    for (const auto & direction : directions) {
      const GridIndex next{current.x + direction.x, current.y + direction.y};
      if (isInside(next) && !isObstacle(next)) {
        result.push_back(next);
      }
    }

    return result;
  }

  // 计算两个相邻栅格之间的移动代价。
  double moveCost(const GridIndex & from, const GridIndex & to) const
  {
    const int dx = std::abs(to.x - from.x);
    const int dy = std::abs(to.y - from.y);
    // 斜向移动距离为 sqrt(2)，直线移动距离为 1。
    return (dx == 1 && dy == 1) ? std::sqrt(2.0) : 1.0;
  }

  // 将栅格坐标转换为 ROS/RViz 中使用的 map 坐标。
  geometry_msgs::msg::Point gridToWorldPoint(const GridIndex & index) const
  {
    geometry_msgs::msg::Point point;
    // 加 0.5 是为了把坐标放在栅格中心，而不是栅格左下角。
    point.x = (static_cast<double>(index.x) + 0.5) * resolution_;
    point.y = (static_cast<double>(index.y) + 0.5) * resolution_;
    point.z = 0.0;
    return point;
  }

  // 将 RViz 中点击的世界坐标转换为栅格坐标。
  GridIndex worldToGridIndex(double x, double y) const
  {
    return {
      static_cast<int>(std::floor(x / resolution_)),
      static_cast<int>(std::floor(y / resolution_))
    };
  }

  // 遍历整张地图，返回所有障碍物栅格，用于 RViz Marker 显示。
  std::vector<GridIndex> obstacles() const
  {
    std::vector<GridIndex> result;
    for (int y = 0; y < height_; ++y) {
      for (int x = 0; x < width_; ++x) {
        const GridIndex index{x, y};
        if (isObstacle(index)) {
          result.push_back(index);
        }
      }
    }
    return result;
  }

private:
  // 二维栅格坐标转一维数组下标：index = y * width + x。
  std::size_t toLinearIndex(const GridIndex & index) const
  {
    return static_cast<std::size_t>(index.y * width_ + index.x);
  }

  int width_{0};
  int height_{0};
  double resolution_{1.0};
  std::vector<bool> occupancy_;
};

// 标准二维 A* 规划器。
class AStarPlanner
{
public:
  // 输入地图、起点和终点，返回从 start 到 goal 的栅格路径。
  std::vector<GridIndex> search(
    const GridMap & map,
    const GridIndex & start,
    const GridIndex & goal)
  {
    std::priority_queue<PriorityQueueItem> frontier;
    // came_from[当前节点] = 父节点，用于最后回溯路径。
    std::unordered_map<GridIndex, GridIndex, GridIndexHash> came_from;
    // cost_so_far[节点] = 从起点到该节点的最小已知代价。
    std::unordered_map<GridIndex, double, GridIndexHash> cost_so_far;

    // 将起点加入开放列表，起点代价为 0。
    frontier.push({start, 0.0});
    came_from[start] = start;
    cost_so_far[start] = 0.0;

    while (!frontier.empty()) {
      // 每次取出 f = g + h 最小的节点进行扩展。
      const GridIndex current = frontier.top().index;
      frontier.pop();

      // 如果扩展到目标点，则通过 came_from 回溯路径。
      if (current == goal) {
        return reconstructPath(came_from, start, goal);
      }

      // 遍历当前节点的所有可通行邻居。
      for (const auto & next : map.neighbors(current)) {
        const double new_cost = cost_so_far[current] + map.moveCost(current, next);
        const auto old_cost = cost_so_far.find(next);

        // 如果 next 没访问过，或者发现了更便宜的路径，则更新它。
        if (old_cost == cost_so_far.end() || new_cost < old_cost->second) {
          cost_so_far[next] = new_cost;
          frontier.push({next, new_cost + heuristic(next, goal)});
          came_from[next] = current;
        }
      }
    }

    return {};
  }

private:
  // 启发式函数 h(n)：估计 from 到 to 的距离。
  // 当前使用欧几里得距离，适合八邻域栅格。
  double heuristic(const GridIndex & from, const GridIndex & to) const
  {
    const double dx = static_cast<double>(to.x - from.x);
    const double dy = static_cast<double>(to.y - from.y);
    return std::hypot(dx, dy);
  }

  // 根据 came_from 从 goal 倒推到 start，恢复完整路径。
  std::vector<GridIndex> reconstructPath(
    const std::unordered_map<GridIndex, GridIndex, GridIndexHash> & came_from,
    const GridIndex & start,
    const GridIndex & goal) const
  {
    std::vector<GridIndex> path;
    GridIndex current = goal;
    path.push_back(current);

    // came_from 记录的是“当前点来自哪个父节点”，所以需要从终点倒着找。
    while (!(current == start)) {
      const auto parent = came_from.find(current);
      if (parent == came_from.end()) {
        return {};
      }

      current = parent->second;
      path.push_back(current);
    }

    // 倒推得到的是 goal -> start，需要反转成 start -> goal。
    std::reverse(path.begin(), path.end());
    return path;
  }
};

// ROS2 演示节点：创建测试地图，运行 A*，发布地图、路径和障碍物给 RViz。
class AStarDemoNode : public rclcpp::Node
{
public:
  AStarDemoNode()
  : Node("a_star_demo_node"), map_(40, 30, 0.1)
  {
    // 消息队列深度为1，保存上一次发布的消息。
    const auto latched_qos = rclcpp::QoS(1).transient_local().reliable();

    map_pub_ = create_publisher<nav_msgs::msg::OccupancyGrid>("a_star_map", latched_qos);
    path_pub_ = create_publisher<nav_msgs::msg::Path>("a_star_path", latched_qos);
    obstacle_pub_ = create_publisher<visualization_msgs::msg::Marker>(
      "a_star_obstacles",
      latched_qos);

    initial_pose_sub_ = create_subscription<geometry_msgs::msg::PoseWithCovarianceStamped>(
      "initialpose",
      10,
      std::bind(&AStarDemoNode::initialPoseCallback, this, std::placeholders::_1));

    goal_pose_sub_ = create_subscription<geometry_msgs::msg::PoseStamped>(
      "goal_pose",
      10,
      std::bind(&AStarDemoNode::goalPoseCallback, this, std::placeholders::_1));

    // 生成演示障碍物。
    addDemoObstacles(map_);

    // 周期发布地图和路径
    publish_timer_ = create_wall_timer(
      std::chrono::milliseconds(500),
      std::bind(&AStarDemoNode::publishVisualization, this));

    // 启动后先规划一条默认路径，便于 RViz 立即看到效果。
    planAndPublish({2, 2}, {35, 25});
    RCLCPP_INFO(get_logger(), "A* demo node started.");
    RCLCPP_INFO(get_logger(), "RViz start: 2D Pose Estimate, goal: Nav2 Goal.");
  }

private:
  // 周期发布地图、障碍物和最近一次规划路径。
  void publishVisualization()
  {
    map_pub_->publish(makeOccupancyGridMsg(map_));
    obstacle_pub_->publish(makeObstacleMarker(map_));

    if (!last_path_.empty()) {
      path_pub_->publish(makePathMsg(map_, last_path_));
    }
  }

  // 执行 A* 搜索并发布路径。
  void planAndPublish(const GridIndex & start, const GridIndex & goal)
  {
    AStarPlanner planner;
    last_path_ = planner.search(map_, start, goal);

    if (last_path_.empty()) {
      RCLCPP_WARN(get_logger(), "A* failed to find a path.");
      return;
    }

    path_pub_->publish(makePathMsg(map_, last_path_));
    RCLCPP_INFO(get_logger(), "A* found a path with %zu points.", last_path_.size());
  }

  // RViz 的 2D Pose Estimate 回调，用来设置起点。
  void initialPoseCallback(const geometry_msgs::msg::PoseWithCovarianceStamped::SharedPtr msg)
  {
    start_ = map_.worldToGridIndex(msg->pose.pose.position.x, msg->pose.pose.position.y);
    has_start_ = true;
    RCLCPP_INFO(get_logger(), "Start set to grid (%d, %d).", start_.x, start_.y);
    tryPlanFromRvizInput();
  }

  // RViz 的 Nav2 Goal 回调，用来设置终点。
  void goalPoseCallback(const geometry_msgs::msg::PoseStamped::SharedPtr msg)
  {
    goal_ = map_.worldToGridIndex(msg->pose.position.x, msg->pose.position.y);
    has_goal_ = true;
    RCLCPP_INFO(get_logger(), "Goal set to grid (%d, %d).", goal_.x, goal_.y);
    tryPlanFromRvizInput();
  }

  // 当起点和终点都有效时，重新执行规划。
  void tryPlanFromRvizInput()
  {
    if (!has_start_ || !has_goal_) {
      return;
    }

    if (!map_.isInside(start_) || map_.isObstacle(start_)) {
      RCLCPP_WARN(get_logger(), "Start is outside the map or inside an obstacle.");
      return;
    }

    if (!map_.isInside(goal_) || map_.isObstacle(goal_)) {
      RCLCPP_WARN(get_logger(), "Goal is outside the map or inside an obstacle.");
      return;
    }

    planAndPublish(start_, goal_);
  }

  // 构造演示地图中的障碍物。
  void addDemoObstacles(GridMap & map)
  {
    // 一堵竖墙，中间留出缺口。
    for (int y = 0; y < 25; ++y) {
      if (y >= 12 && y <= 15) {
        continue;
      }
      map.setObstacle(18, y);
    }

    // 一段横向障碍物。
    for (int x = 22; x < 30; ++x) {
      map.setObstacle(x, 18);
    }
  }

  // 将 A* 栅格路径转换为 nav_msgs/Path，供 RViz 显示。
  nav_msgs::msg::Path makePathMsg(
    const GridMap & map,
    const std::vector<GridIndex> & path)
  {
    nav_msgs::msg::Path path_msg;
    path_msg.header.frame_id = "map";
    path_msg.header.stamp = now();

    for (const auto & index : path) {
      geometry_msgs::msg::PoseStamped pose;
      pose.header = path_msg.header;
      pose.pose.position = map.gridToWorldPoint(index);
      pose.pose.orientation.w = 1.0;
      path_msg.poses.push_back(pose);
    }

    return path_msg;
  }

  // 将内部 GridMap 转换为 OccupancyGrid，供 RViz Map 插件显示。
  nav_msgs::msg::OccupancyGrid makeOccupancyGridMsg(const GridMap & map)
  {
    nav_msgs::msg::OccupancyGrid map_msg;
    map_msg.header.frame_id = "map";
    map_msg.header.stamp = now();
    map_msg.info.resolution = map.resolution();
    map_msg.info.width = static_cast<std::uint32_t>(map.width());
    map_msg.info.height = static_cast<std::uint32_t>(map.height());
    map_msg.info.origin.orientation.w = 1.0;
    map_msg.data.resize(static_cast<std::size_t>(map.width() * map.height()), 0);

    for (int y = 0; y < map.height(); ++y) {
      for (int x = 0; x < map.width(); ++x) {
        const GridIndex index{x, y};
        const auto linear_index = static_cast<std::size_t>(y * map.width() + x);
        map_msg.data[linear_index] = map.isObstacle(index) ? 100 : 0;
      }
    }

    return map_msg;
  }

  // 将障碍物栅格转换为 CUBE_LIST Marker，供 RViz Marker 插件显示。
  visualization_msgs::msg::Marker makeObstacleMarker(const GridMap & map)
  {
    visualization_msgs::msg::Marker marker;
    marker.header.frame_id = "map";
    marker.header.stamp = now();
    marker.ns = "a_star";
    marker.id = 0;
    marker.type = visualization_msgs::msg::Marker::CUBE_LIST;
    marker.action = visualization_msgs::msg::Marker::ADD;
    marker.scale.x = map.resolution();
    marker.scale.y = map.resolution();
    marker.scale.z = 0.02;
    marker.color.r = 0.2;
    marker.color.g = 0.2;
    marker.color.b = 0.2;
    marker.color.a = 1.0;

    for (const auto & obstacle : map.obstacles()) {
      marker.points.push_back(map.gridToWorldPoint(obstacle));
    }

    return marker;
  }

  rclcpp::Publisher<nav_msgs::msg::OccupancyGrid>::SharedPtr map_pub_;
  rclcpp::Publisher<nav_msgs::msg::Path>::SharedPtr path_pub_;
  rclcpp::Publisher<visualization_msgs::msg::Marker>::SharedPtr obstacle_pub_;

  rclcpp::Subscription<geometry_msgs::msg::PoseWithCovarianceStamped>::SharedPtr initial_pose_sub_;
  rclcpp::Subscription<geometry_msgs::msg::PoseStamped>::SharedPtr goal_pose_sub_;
  rclcpp::TimerBase::SharedPtr publish_timer_;

  GridMap map_;
  GridIndex start_;
  GridIndex goal_;
  std::vector<GridIndex> last_path_;
  bool has_start_{false};
  bool has_goal_{false};
};

}  // namespace hybrid_astar_planner

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<hybrid_astar_planner::AStarDemoNode>());
  rclcpp::shutdown();
  return 0;
}
