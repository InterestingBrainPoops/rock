#include <iostream>
#include <cmath>
int main()
{
    float rotation = 0.0;
    float angular_vel = 0.0;
    float min_angle = -30.0;
    float max_angle = 30.0;
    float thrust = 50.0;
    float mass = 30.0;
    float kP = 0.1;
    float kI = 0.1;
    float kD = 0.2;
    int time = 0;
    while (time < 1000)
    {
        float control_effort = 0.0;

        angular_vel += 0.1 * pow(angular_vel, 2.0);
        rotation += angular_vel;
        time += 1;
    }
    return 0;
}