// 宏保护
#ifndef TARGET_FOLLOWER__PID_CONTROLLER_HPP_
#define TARGET_FOLLOWER__PID_CONTROLLER_HPP_

namespace target_follower
{

class PidController
{
public:
  // 无参数构造函数
  PidController() = default;

  // 构造函数重载
  PidController(
    double kp, double ki, double kd, double kf,
    double max_integral, double max_output);

  void configure(
    double kp, double ki, double kd, double kf,
    double max_integral, double max_output);

  double calculate(double input, double setpoint);
  void reset();

  double error() const;
  double integral() const;

private:
  static double limit_value(double value, double maximum);

  double kp_{0.0};
  double ki_{0.0};
  double kd_{0.0};
  double kf_{0.0};
  double max_integral_{0.0};
  double max_output_{0.0};

  double error_{0.0};
  double previous_error_{0.0};
  double previous_target_{0.0};
  double integral_{0.0};
};

}  // namespace target_follower

#endif  // TARGET_FOLLOWER__PID_CONTROLLER_HPP_
