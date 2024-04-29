use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {  //区块头
    pub time: i64,  //时间戳
    pub tx_hash: String,  //交易数据merkle数根hash值
    pub pre_hash: String, //前一区块（父区块）hash值
}

#[derive(Debug)]
pub struct Block {  //区块
    pub header: BlockHeader,  //区块头部信息
    pub hash: String,  //当前区块hash值
    pub data: String, //交易数据
}

impl Block {
    fn set_hash(&mut self) {  //设置hash值
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {  //创建一个新的区块
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,  //transactions data merkle root hash
                pre_hash: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };

        block.set_hash();
        block
    }
}
