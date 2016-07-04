#coding: utf8

from jsonrpc2 import JsonRpc
import catalog

def hello(database, *args, **kwargs):
    db   = database.test
    coll = db.foo

    for record in coll.find():
        print record
    return "hello world."


def init():
    rpc = JsonRpc()
    rpc['hello'] = hello

    rpc['catalog.get']    = catalog.get
    rpc['catalog.append'] = catalog.append
    rpc['catalog.update'] = catalog.update
    rpc['catalog.remove'] = catalog.remove
    rpc['catalog.drop']   = catalog.drop
    return rpc
