use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {  //向区块链中添加一个新的区块
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> block::Block {  //创建一个创世块
        block::Block::new_block("This is genesis block".to_string(), String::from(""))
    }

    pub fn new_blockchain() -> BlockChain {  //创建一个新区块链
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
