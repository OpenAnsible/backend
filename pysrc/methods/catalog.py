#coding: utf8

import time
from utils.mongo import result_to_json, result_to_dict, ObjectId

database_name   = "open_ansible"
collection_name = "catalog"

_get_collection = lambda d: d[database_name][collection_name]


def get(database, object_id, *args, **kwargs):
    coll   = _get_collection(database)
    oid    = ObjectId(object_id)
    result = coll.find_one({"_id": oid})
    if result is None:
        catalog = None
    else:
        catalog = result_to_json(list(result))
    return result

def append(database, *args, **kwargs):
    coll = _get_collection(database)
    doc  = { "name": "这是一个目录",
             "descp": "这是目录描述",
             "ctime": time.time(),
             "utime": time.time()
    }
    coll.insert_one(doc)
    return result_to_json(doc)

def update(database, *args, **kwargs):
    return None

def remove(database, object_id, *args, **kwargs):
    coll = _get_collection(database)
    oid  = ObjectId(object_id)
    result = coll.delete_one({"_id": oid})
    return result.deleted_count == 1

def search (database, *args, **kwargs):
    pass

def drop(database, *args, **kwargs):
    # coll   = _get_collection(database)
    # coll.drop()
    raise Exception("unsupport")
