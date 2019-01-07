#!/usr/bin/env bash
echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots
echo cape-universal > /sys/devices/platform/bone_capemgr/slots
echo 1 > /sys/class/pwm/pwmchip6/export
