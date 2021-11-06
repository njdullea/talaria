use ta::DataItem;

pub trait Trade {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction>;
}

#[derive(Debug)]
pub enum MarketAction {
    Buy,
    Sell,
}
