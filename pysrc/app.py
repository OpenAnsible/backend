#coding: utf8
import tornado.ioloop
import tornado.web

import json, time


from pymongo import MongoClient

import methods
import utils

class MainHandler(tornado.web.RequestHandler):
    btime = 0.0
    etime = 0.0
    def initialize(self, database, rpc):
        self.btime    = time.time()
        self.database = database
        self.rpc      = rpc

    def prepare(self):
        pass

    def get(self):
        self.write("This is one JsonRPC2 API Service.")

    def post(self):
        self.put()

    def put(self):
        try:
            rpc_request = json.loads(self.request.body)
        except:
            rpc_request = {}
        # error = {"code": -32700, "message": "Parse error", "data": None}
        # rpc_response = {"jsonrpc": "2.0", "error": error, "id": None}
        if "params" in rpc_request:
            if type(rpc_request['params']) == dict:
                rpc_request['params']['database'] = self.database
            elif type(rpc_request['params']) == list:
                rpc_request['params'].insert(0, self.database)
            else:
                pass

        rpc_response = self.rpc(rpc_request)
        rpc_response['elapsed'] = u"%.20fms" % ((time.time() - self.btime)*1000)
        return self.write(json.dumps(rpc_response))

    def on_finish(self):
        self.etime = time.time()
        print u"时间消耗: %.20f ms" % ((self.etime - self.btime)*1000)



if __name__ == "__main__":
    # https://api.mongodb.com/python/current/faq.html
    client = MongoClient("127.0.0.1", 27017, maxPoolSize=10, 
        waitQueueTimeoutMS=5, connectTimeoutMS=5, socketTimeoutMS=5, serverSelectionTimeoutMS=5) # mongodb://localhost:27017/
    rpc = methods.init()

    app = tornado.web.Application([
        (r"/", MainHandler, dict(database=client, rpc=rpc) ),
    ])

    app.listen(8888)
    print "Server running on 127.0.0.1:8888 ..."
    tornado.ioloop.IOLoop.current().start()
