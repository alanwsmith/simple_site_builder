#!/usr/bin/env python3

import sys

for line in sys.stdin.readlines():
    line = line.replace("TEST_VAR_1", "output_from_test_var_1")
    print(line.rstrip())
