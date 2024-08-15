import sys
import time


print(f"args passed: {sys.argv[1:]}")

for i in range(2):
    print(f"forwarded stdout -- {i}")
    print(f"forwarded stderr -- {i}", file=sys.stderr)
    time.sleep(0.01)
