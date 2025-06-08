#!/usr/bin/env python3

import json
from datetime import datetime

def write_json_to_file(data, file):
    with open(file, 'w', encoding='utf-8') as _out:
        json.dump(data, _out, sort_keys=True, indent=4)

now = datetime.now()
current_time = now.strftime("%Y-%m-%d %H:%M:%S")

d = {
        "updated": current_time
    }


file_path = "../data/auto.json"
write_json_to_file(d, file_path)


