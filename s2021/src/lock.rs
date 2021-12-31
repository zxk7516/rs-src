use lazy_static::*;
use redis::{
    Connection,
    RedisResult,
    ToRedisArgs,
};

/// -- KEYS[1]: 锁key
/// -- ARGV[1]: 锁value,随机字符串
/// -- ARGV[2]: 过期时间
/// -- 判断锁key持有的value是否等于传入的value
/// -- 如果相等说明是再次获取锁并更新获取时间，防止重入时过期
/// -- 这里说明是“可重入锁”
/// if redis.call("GET", KEYS[1]) == ARGV[1] then
///     -- 设置
///     redis.call("SET", KEYS[1], ARGV[1], "PX", ARGV[2])
///     return "OK"
///
/// else
///     -- 锁key.value不等于传入的value则说明是第一次获取锁
///     -- SET key value NX PX timeout : 当key不存在时才设置key的值
///     -- 设置成功会自动返回“OK”，设置失败返回“NULL Bulk Reply”
///     -- 为什么这里要加“NX”呢，因为需要防止把别人的锁给覆盖了
///     return redis.call("SET", KEYS[1], ARGV[1], "NX", "PX", ARGV[2])
/// end
const LOCK_STR: &'static str = r#"if redis.call("GET", KEYS[1]) == ARGV[1] then redis.call("SET", KEYS[1], ARGV[1], "PX", ARGV[2]) return "OK" else return redis.call("SET", KEYS[1], ARGV[1], "NX", "PX", ARGV[2]) end"#;

/// -- 释放锁
/// -- 不可以释放别人的锁
/// if redis.call("GET", KEYS[1]) == ARGV[1] then
///     -- 执行成功返回“1”
///     return redis.call("DEL", KEYS[1])
/// else
///     return 0
/// end
const UNLOCK_STR: &'static str = r#"if redis.call("GET", KEYS[1]) == ARGV[1] then return redis.call("DEL", KEYS[1]) else return 0 end "#;

lazy_static! {
    /// Lock Script
    pub static ref LOCK_SCRIPT: redis::Script = redis::Script::new(LOCK_STR);
    /// UnLock Script
    pub static ref UNLOCK_SCRIPT: redis::Script = redis::Script::new(UNLOCK_STR);
}

/// Redis Lock
pub struct RedLock<K>
where
    K: ToRedisArgs,
{
    key: K,
    #[allow(dead_code)]
    ttl: u64, // milliSeconds set px
    value: [u8; 8],
}

impl<K> RedLock<K>
where
    K: ToRedisArgs,
{
    /// create a lock with ttl in milii seconds, Err if created failed
    pub fn new(
        key: K,
        ttl: u64, // milliSeconds set px
        conn: &mut Connection,
    ) -> RedisResult<Self> {
        let value: u64 = rand::random();
        let value = value.to_le_bytes();
        let s: String = LOCK_SCRIPT.key(&key).arg(&value).arg(ttl).invoke(conn)?;
        if s.eq("OK") {
            Ok(RedLock { key, ttl, value })
        } else {
            Err((redis::ErrorKind::TryAgain, "locked not ok").into())
        }
    }

    /// unlock the key
    pub fn unlock(
        &self,
        conn: &mut Connection,
    ) -> RedisResult<()> {
        UNLOCK_SCRIPT.key(&self.key).arg(&self.value).invoke(conn)
    }
}
