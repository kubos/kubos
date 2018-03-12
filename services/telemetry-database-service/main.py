import argparse
from app import *
import yaml
import models

parser = argparse.ArgumentParser(description='Radio demo')
parser.add_argument('config', type=str, help='path to config file')
parser.add_argument('database', type=str, help='telemetry database path')
args = parser.parse_args()

with open(args.config) as ymlfile:
    cfg = yaml.load(ymlfile)

models.init_db(args.database)
app = create_app()
app.run(host=cfg['APP_IP'], port=cfg['TELEM_PORT'])
