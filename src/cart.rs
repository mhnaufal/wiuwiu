pub struct Product {
    pub name: String,
    pub quantity: u64,
}

pub struct Cart {
    pub(crate) Products: Vec<Product>,
}

pub trait ICart {
    fn tambah_produk(&mut self, i_name: String, i_quantity: u64);
    fn tampilkan_cart(&mut self);
}

impl ICart for Cart {
    fn tambah_produk(&mut self, i_name: String, i_quantity: u64) {
        let new_product: Product = Product { name: i_name, quantity: i_quantity };

        self.Products.push(new_product);
    }

    fn tampilkan_cart(&mut self) {
        for product in &self.Products { // iterate over a copy of self.Products
            println!("{} ({})", product.name, product.quantity);
            format!("{} ({})", product.name, product.quantity);
        }
    }
}