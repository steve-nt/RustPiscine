#[derive(Debug, Clone, PartialEq)]
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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n_free = prices.len() / 3;
        let total: f32 = prices.iter().sum();
        let free_sum: f32 = prices.iter().take(n_free).sum();

        let ratio = if total > 0.0 { (total - free_sum) / total } else { 1.0 };

        let receipt: Vec<f32> = prices.iter()
            .map(|&p| (p * ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt = receipt.clone();
        receipt
    }
}
