-- KEYS[1]: 锁key
-- ARGV[1]: 锁value,随机字符串
-- ARGV[2]: 过期时间
-- 判断锁key持有的value是否等于传入的value
-- 如果相等说明是再次获取锁并更新获取时间，防止重入时过期
-- 这里说明是“可重入锁”
if redis.call("GET", KEYS[1]) == ARGV[1] then
    -- 设置
    redis.call("SET", KEYS[1], ARGV[1], "PX", ARGV[2])
    return "OK"

else
    -- 锁key.value不等于传入的value则说明是第一次获取锁
    -- SET key value NX PX timeout : 当key不存在时才设置key的值
    -- 设置成功会自动返回“OK”，设置失败返回“NULL Bulk Reply”
    -- 为什么这里要加“NX”呢，因为需要防止把别人的锁给覆盖了
    return redis.call("SET", KEYS[1], ARGV[1], "NX", "PX", ARGV[2])
end
