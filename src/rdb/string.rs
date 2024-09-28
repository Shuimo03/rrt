use std::io::{Cursor,Result};
use crate::{Pair, read_key_string};



pub fn parse_string(cursor: &mut Cursor<&[u8]>) -> Result<Pair> {
    let vt = String::from("String");
    let key = read_key_string(cursor)?;
    let mut value = read_key_string(cursor)?; //这里是因为redis-string编码也提供给了key，所以这里是通用的。

    Ok(Pair {
        key_name: key,
        val_type:vt,
        val:value,
        size: cursor.position() as usize, //TODO Bug
        expiry: None,
    })
}

