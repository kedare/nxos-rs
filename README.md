# Introduction

Rust helpers when running code directly on NX-OS.

This scope of this project is to make rust useable on Cisco NX-OS 7 and 9 (to not have to use Python 2 on NX-OS 7).
It will reimplemented the featuers (probably not all) from the Cisco embedded python library (it is not meant to be a 1-to-1 mapping of the existing functions or features)

# How to use rust on NX-OS

As NX-OS is building using older versions of Linux and libc,
the rust application you are building need to be compiled with the following target : `x86_64-unknown-linux-musl`.
Make sure you other libraries have no specific dynamic dependencies (openssl, etc...)