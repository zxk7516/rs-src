-- 释放锁
-- 不可以释放别人的锁
if redis.call("GET", KEYS[1]) == ARGV[1] then
    -- 执行成功返回“1”
    return redis.call("DEL", KEYS[1])
else
    return 0
end
