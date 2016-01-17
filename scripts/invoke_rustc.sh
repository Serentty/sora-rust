#!/bin/bash
rustc "$@" -Z no-landing-pads -L build -L $SYSROOT_LIBS --sysroot=sysroot