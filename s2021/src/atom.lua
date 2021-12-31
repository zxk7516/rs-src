local key1 = KEYS[1]
local arg1 = ARGV[1]
local arg2 = ARGV[2]


local Flag = redis.call("setnx", key1, arg1)

redis.call("expire", key1, arg2)
return Flag

