use std::io::{Cursor,Result};
use crate::{Pair, read_key_string};



pub fn parse_string(cursor: &mut Cursor<&[u8]>) -> Result<Pair> {
    /**
    这里进来的是String Encoding,最好当作是字节数组，一共有三种:
        Length prefixed strings 长度前缀字符串
        An 8, 16 or 32 bit integer   8、16 或 32 位整数
        A LZF compressed string LZF 压缩字符串
     */


    let vt = String::from("String");
    let key = read_key_string(cursor)?;
    let mut value = read_key_string(cursor)?; //这里是因为redis-string编码也提供给了key，所以这里是通用的。


    Ok(Pair {
        key_name: key,
        val_type:vt,
        val:value,
        size: cursor.position() as usize,
        expiry: None,
    })
}

