#!/bin/bash

g_color_red='\033[31m'     # 红色
g_color_green='\033[32m'  # 黄色
g_color_normal='\033[0m'   # 正常色

log() {
    echo -e "[$(date +%F-%T)] $*"
}

log_info() {
    log "[info]$*"
}

log_succ() {
    log "${g_color_green}[success]${g_color_normal}$*"
}

log_error() {
    log "${g_color_red}[error]${g_color_normal}$*"
    exit 1
}

sudo_run() {
  sudo -u root -H sh -c "$1"
}
