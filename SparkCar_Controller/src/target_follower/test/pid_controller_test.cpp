#include <cassert>
#include <cmath>

#include "target_follower/pid_controller.hpp"

int main()
{
  target_follower::PidController controller(0.5, 0.0, 0.0, 0.0, 0.0, 1.0);

  assert(std::abs(controller.calculate(0.0, 1.0) - 0.5) < 1e-9);
  assert(std::abs(controller.calculate(2.0, 1.0) + 0.5) < 1e-9);
  assert(std::abs(controller.calculate(-10.0, 1.0) - 1.0) < 1e-9);

  controller.reset();
  assert(std::abs(controller.error()) < 1e-9);
  assert(std::abs(controller.integral()) < 1e-9);
  return 0;
}
