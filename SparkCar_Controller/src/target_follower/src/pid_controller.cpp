/*
# @file pid_controller.cpp
# @author JiaVerso
# @brief  A C++ implementation of a PID controller
# @version 0.1
# @date 2026-07-13
#
# @copyright JiaVerso (c) 2026
*/


#include "target_follower/pid_controller.hpp"

#include <algorithm>
#include <cmath>
#include <stdexcept>

namespace target_follower
{

PidController::PidController(
  double kp, double ki, double kd, double kf,
  double max_integral, double max_output)
{
  configure(kp, ki, kd, kf, max_integral, max_output);
}

void PidController::configure(
  double kp, double ki, double kd, double kf,
  double max_integral, double max_output)
{
  if (!std::isfinite(kp) || !std::isfinite(ki) || !std::isfinite(kd) ||
    !std::isfinite(kf) || max_integral < 0.0 || max_output <= 0.0)
  {
    throw std::invalid_argument("invalid PID parameters");
  }

  kp_ = kp;
  ki_ = ki;
  kd_ = kd;
  kf_ = kf;
  max_integral_ = max_integral;
  max_output_ = max_output;
  reset();
}

double PidController::calculate(double input, double setpoint)
{
  // Check for NaN or infinite values
  if (!std::isfinite(input) || !std::isfinite(setpoint)) {
    reset();
    return 0.0;
  }

  previous_error_ = error_;
  error_ = setpoint - input;

  const double proportional_output = error_ * kp_;
  const double derivative_output = (error_ - previous_error_) * kd_;
  const double feedforward_output = setpoint * kf_;

  integral_ += error_ * ki_;
  integral_ = limit_value(integral_, max_integral_);

  previous_target_ = setpoint;
  return limit_value(
    proportional_output + integral_ + derivative_output + feedforward_output,
    max_output_);
}

void PidController::reset()
{
  error_ = 0.0;
  previous_error_ = 0.0;
  previous_target_ = 0.0;
  integral_ = 0.0;
}

double PidController::error() const
{
  return error_;
}

double PidController::integral() const
{
  return integral_;
}

double PidController::limit_value(double value, double maximum)
{
  if (maximum <= 0.0) {
    return 0.0;
  }
  return std::clamp(value, -maximum, maximum);
}

}  // namespace target_follower
