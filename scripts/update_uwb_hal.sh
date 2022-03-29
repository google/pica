#!/bin/bash

# /!\ This script shall be run in top of the Android Source /!\
# The first time, you will need to reboot right after the remount.

HAL_BIN="out/target/product/vsoc_x86_64/vendor/bin/hw/android.hardware.uwb-service.mock "
HAL_RC="out/target/product/vsoc_x86_64/vendor/etc/init/mock-uwb-service.rc"
HAL_VINTF="out/target/product/vsoc_x86_64/vendor/etc/vintf/manifest/mock-uwb-service.xml"

adb root
sleep 1
adb remount
sleep 1
adb push ${HAL_BIN} /vendor/bin/hw
sleep 0.5
adb push ${HAL_BIN} /system/vendor/bin/hw
sleep 0.5
adb push ${HAL_VINTF} /vendor/etc/vintf/manifest/
sleep 0.5
adb push ${HAL_VINTF} /system/vendor/etc/vintf/manifest/
sleep 0.5
adb push ${HAL_RC} /vendor/etc/init/
sleep 0.5
adb push ${HAL_RC} /system/vendor/etc/init/
sleep 1
adb reboot
