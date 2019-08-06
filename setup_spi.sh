#!/bin/sh

set -e
set -o pipefail
set -o nounset

sudo raspi-config nonint do_spi 0
