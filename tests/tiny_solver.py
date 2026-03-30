#!/usr/bin/env python3
import sys
import time
import signal

idigest = None
for line in sys.stdin:
    if line.startswith("#s idigest "):
        idigest = line.split(" ")[2].replace('"', '').strip()

if idigest == "008f20e0af36c203ffdd3182439a814e":
    # tiny05
    def signal_handler(sig, frame):
        while True:
            pass


    signal.signal(signal.SIGINT, signal_handler)
    while True:
        time.sleep(1)

if idigest == "0094f6a3b54554767e43e70faadfd361":
    # tiny05
    def signal_handler(sig, frame):
        print("""(3,1);
4;
((2,5),6);
""")
        sys.exit(0)


    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    while True:
        time.sleep(1)

answers = {
    "0010b172a28d0664d5521e1296fc3586": "(5,3);\n6;\n4;\n(1,2);\n",  # tiny01: opt
    "10fc2300fae9577ad4296f1c8b8b16d9": "((2,3),(((8,5),(6,7)),4));\n1;\n",  # tiny02: subopt (+1)
    "10cc91161ca13497efa530c5239d2446": """1;\n3;\n4;\n(5,7);\n6;\n8;\n""",  # tiny03: leaf 2 is missing
    "004be79895dc5bfdb2b22e5dde781804": "",  # tiny04: no answer
    # tiny05: timeout - see above
    # tiny06: report after sigterm - see above
    "01e632be9ef4c8f9ecadb9c83815d966": "7\n6\n5;\n((((11,2),12),9),8);\n4;\n10;\n3;\n1;",  # tiny07: missing semicolons
    "10c76d3ec96d89397f4e78d0f1cec803": "foo bar\n17;\ntest;\n10;\n15;\n(16,8);\n6;\n14;\n4;\n12;\n9;\n5;\n7;\n((((3,11),2),13),1);"
    # tiny08: syntax error
    # tiny09: key missing -- exit(1)
}

print(answers[idigest])
