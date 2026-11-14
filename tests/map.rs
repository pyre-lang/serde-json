use serde_json::{from_str, Map, Value};

#[test]
fn test_preserve_order() {
    // Sorted order
    #[cfg(not(feature = "preserve_order"))]
    const EXPECTED: &[&str] = &["a", "b", "c"];

    // Insertion order
    #[cfg(feature = "preserve_order")]
    const EXPECTED: &[&str] = &["b", "a", "c"];

    let v: Value = from_str(r#"{"b":null,"a":null,"c":null}"#).unwrap();
    let keys: Vec<_> = v.as_object().unwrap().keys().collect();
    assert_eq!(keys, EXPECTED);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_shift_insert() {
    let mut v: Value = from_str(r#"{"b":null,"a":null,"c":null}"#).unwrap();
    let val = v.as_object_mut().unwrap();
    val.shift_insert(0, "d".to_owned(), Value::Null);

    let keys: Vec<_> = val.keys().collect();
    assert_eq!(keys, &["d", "b", "a", "c"]);
}

#[test]
fn test_append() {
    // Sorted order
    #[cfg(not(feature = "preserve_order"))]
    const EXPECTED: &[&str] = &["a", "b", "c"];

    // Insertion order
    #[cfg(feature = "preserve_order")]
    const EXPECTED: &[&str] = &["b", "a", "c"];

    let mut v: Value = from_str(r#"{"b":null,"a":null,"c":null}"#).unwrap();
    let val = v.as_object_mut().unwrap();
    let mut m = Map::new();
    m.append(val);
    let keys: Vec<_> = m.keys().collect();

    assert_eq!(keys, EXPECTED);
    assert!(val.is_empty());
}

#[test]
fn test_retain() {
    let mut v: Value = from_str(r#"{"b":null,"a":null,"c":null}"#).unwrap();
    let val = v.as_object_mut().unwrap();
    val.retain(|k, _| k.as_str() != "b");

    let keys: Vec<_> = val.keys().collect();
    assert_eq!(keys, &["a", "c"]);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_sorted_iteration() {
    let mut map = Map::new();
    assert_eq!(map.insert("b".to_owned(), 1.into()), None);
    assert_eq!(map.insert("a".to_owned(), 2.into()), None);
    assert_eq!(map.insert("c".to_owned(), 3.into()), None);
    assert_eq!(map.insert("a".to_owned(), 4.into()), Some(2.into()));

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, &["a", "b", "c"]);
    assert_eq!(map["a"], 4);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_matches_btreemap() {
    use std::collections::BTreeMap;

    let mut json_map = Map::new();
    let mut btree = BTreeMap::new();
    let mut rng = 0x9E37_79B9_7F4A_7C15u64;

    fn xorshift(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    fn key(n: u64) -> String {
        format!("k{}", n % 41)
    }

    for _ in 0..5000 {
        let op = xorshift(&mut rng) % 4;
        match op {
            0 => {
                let k = key(xorshift(&mut rng));
                let v = xorshift(&mut rng);
                assert_eq!(json_map.insert(k.clone(), v.into()), btree.insert(k, v.into()));
            }
            1 => {
                let k = key(xorshift(&mut rng));
                assert_eq!(json_map.remove(&k), btree.remove(&k));
            }
            2 => {
                let k = key(xorshift(&mut rng));
                let v = xorshift(&mut rng);
                json_map.entry(k.clone()).or_insert(v.into());
                btree.entry(k).or_insert(v.into());
            }
            _ => {
                json_map.retain(|_, v| v.as_u64().unwrap() % 2 == 0);
                btree.retain(|_, v| v.as_u64().unwrap() % 2 == 0);
            }
        }
        assert_eq!(json_map.len(), btree.len());
        assert!(json_map.iter().eq(btree.iter()));
    }
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_parse_duplicate_and_unsorted_keys() {
    let v: Value = serde_json::from_str(r#"{"b":1,"a":2,"b":3}"#).unwrap();
    assert_eq!(v.to_string(), r#"{"a":2,"b":3}"#);
}
