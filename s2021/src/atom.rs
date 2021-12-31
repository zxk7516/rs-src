use lazy_static::*;
// local key1 = KEYS[1]
// local arg1 = ARGV[1]
// local arg2 = ARGV[2]
//
//
// local Flag = redis.call("setnx", key1, arg1)
//
// redis.call("expire", key1, arg2)
// return Flag

const ATOM_STR: &'static str = r#" local flag = redis.call("setnx", KEYS[1], ARGV[1]) redis.call("expire", KEYS[1], ARGV[2]) return flag "#;

lazy_static! {
    static ref ATOM_SCRIPT: redis::Script = redis::Script::new(ATOM_STR);
}
