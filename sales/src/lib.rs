#[derive(Debug,Default, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items:Vec<(String, f32)>,
    pub receipt:Vec<f32>
}
impl Cart {
    pub fn new() -> Cart {
           Cart{
            items:Vec::new(),
            receipt:Vec::new(),
           }
        }
    
    pub fn insert_item(&mut self, s: &Store, ele: String) {
            let mut price:f32=0.0;
        for pr in &s.products{
            if &ele==&pr.0{
                price=pr.1;
                break;
            }
            //  price =match ele{
            //     pr.0=>pr.1,
            //     _=>0.0,
            // }
            
        }
        self.items.push((ele,price));
        self.generate_receipt();
    }
 pub fn get_prices(&self) -> Vec<f32> {
        self.items.iter().map(|(_, p)| *p).collect()
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.get_prices();
        let cal = self.items.len() / 3; // number of "free items"
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // exclude cheapest "free" items
        let v: Vec<f32> = prices[cal..].to_vec();

        // compute scaling factor
        let percentage: f32 = v.iter().sum::<f32>() * 100.0 / prices.iter().sum::<f32>();

        // apply factor to all items
        self.receipt = prices
            .iter()
            .map(|price| round_two(price * percentage / 100.0))
            .collect();

        self.receipt.clone()
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}


// 1.23*((0.0+3.12 + 23.1)/(1.23, 3.12, 23.1))=1.17;