use sled::Result;

fn main() -> Result<()> {
    let tree = sled::open("/tmp/welcome-to-sled").expect("open");

    let key = "name";
    let value = "junhui";
    // tree.insert(key, value)?;
    // tree.flush()?;
    let value = tree.get(key)?.unwrap();
    // convert IVec to String
    let value = String::from_utf8(value.to_vec()).unwrap();
    println!("{:?}: {:?}", key, value);
    // println!("{:?}", sled::IVec::from(value));
    // // insert and get, similar to std's BTreeMap
    // tree.insert("KEY1", "VAL1");
    // assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

    // // range queries
    // for kv in tree.range("KEY1".."KEY9") {}

    // // deletion
    // tree.remove(&"KEY1");

    // // atomic compare and swap
    // tree.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"));

    // // block until all operations are stable on disk
    // // (flush_async also available to get a Future)
    // tree.flush();

    Ok(())
}
