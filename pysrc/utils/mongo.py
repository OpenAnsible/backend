#coding: utf8

import json
from bson import ObjectId

class MongoResultEncoder(json.JSONEncoder):
	def default(self, obj):
		if isinstance(obj, ObjectId):
			return str(obj)
		return json.JSONEncoder.default(self, obj)

def result_to_json(mongo_result):
	return json.dumps(mongo_result, cls=MongoResultEncoder)

def result_to_dict(mongo_result):
	return json.loads(result_to_json(mongo_result))
