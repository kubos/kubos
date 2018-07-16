import argparse
from app import *
import yaml
import requests
import json
import time

parser = argparse.ArgumentParser(description='Telemetry poller demo')
parser.add_argument('config', type=str, help='path to config file')
parser.add_argument('delay', type=int, help='delay between telemetry polls')
args = parser.parse_args()

with open(args.config) as ymlfile:
    cfg = yaml.load(ymlfile)

query_str = """
{
  payload {
    payloadOn
    uptime
    startTime
  }

  thruster {
    thrusterOn
  }
}
"""

mutate_str = """
mutation {
  createTelemetry(
    subsystem: "%s",
    param: "%s",
    value: %s,
    timestamp: %d
  ) {
    subsystem,
    param,
    value,
    timestamp
  }
}
"""

headers = {'Content-Type' : 'application/graphql'}
PAYLOAD_DEST = "http://{}:{}".format(cfg['APP_IP'], cfg['PAYLOAD_PORT'])
TELEM_DEST = "http://{}:{}".format(cfg['APP_IP'], cfg['TELEM_PORT'])

while True:
    try:
        r = requests.post(PAYLOAD_DEST, data=query_str, headers=headers, timeout=10)
        obj = json.loads(r.text)
        for subsys in obj['data']:
            for param in obj['data'][subsys]:
                milli_sec = int(round(time.time() * 1000))
                mutant = mutate_str % (subsys, param, obj['data'][subsys][param], milli_sec)
                r = requests.post(TELEM_DEST, data=mutant, headers=headers, timeout=10)
    except:
        print "Got errr...continuing..."
    finally:
        time.sleep(args.delay)
