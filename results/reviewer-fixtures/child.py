#!/usr/bin/env python3
import json,sys
mode=sys.argv[1]
if mode=='large': print('x'*65537)
elif mode=='malformed': print('{')
else: print(json.dumps({'allocation_calls':1,'allocated_bytes':1024,'peak_live_bytes':1024,'checksum':2 if mode=='valid' else 3}))
