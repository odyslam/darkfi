from ouroboros.logger import Logger

'''
Non-forkable Blockchain for simplicity
#TODO implement forkable chain
'''
class Blockchain(object):
    def __init__(self, R, genesis_time):
        self.blocks = []
        self.log = Logger(self, genesis_time)
        self.R = R # how many slots in single epoch
    
    @property
    def epoch_length(self):
        return self.R

    def __repr__(self):
        buff=''
        for i in range(len(self.blocks)):
            buff+=str(self.blocks[i])
        return buff
    
    def __getitem__(self, i):
        return self.blocks[i]

    def __len__(self):
        return len(self.blocks)

    def __add_block(self, block):
        self.blocks.append(block)

    def add_epoch(self, epoch):
        assert epoch!=None, 'epoch cant be None'
        assert len(epoch)>0 , 'epoch cant be zero-sized'
        for idx, block in enumerate(epoch):
            if not block.empty:
                self.__add_block(block)
            else:
                self.log.warn(f"an empty block at index of index: {block.index},\nrelative slot:{idx}\nabsolute slot: {self.length*idx+block.slot}")
