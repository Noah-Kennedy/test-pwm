#!/usr/bin/env bash
sudo bash -c "echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots"
sudo bash -c "echo cape-universal > /sys/devices/platform/bone_capemgr/slots"

config-pin P8_13 pwm

#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip0/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip1/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip2/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip3/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip4/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip5/export"
#sudo bash -c "echo 1 > /sys/class/pwm/pwmchip6/export"