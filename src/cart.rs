pub struct Product {
    pub name: String,
    pub quantity: u64,
}

pub struct Cart {
    pub(crate) products: Vec<Product>,
}

pub trait ICart {
    fn tambah_produk(&mut self, i_name: String, i_quantity: u64);
    fn hapus_produk(&mut self, i_name: String);
    fn tampilkan_cart(&mut self) -> String;
}

impl ICart for Cart {
    fn tambah_produk(&mut self, i_name: String, i_quantity: u64) {
        let new_product: Product = Product { name: i_name, quantity: i_quantity };

        self.products.push(new_product);
    }

    fn hapus_produk(&mut self, i_name: String) {
        self.products.remove(i_name.parse().unwrap());
    }

    fn tampilkan_cart(&mut self) -> String {
        for product in &self.products { // iterate over a copy of self.products
            println!("{} ({})", product.name, product.quantity);
        };

        "List of products".to_string()

        // self.products.iter().map(ToString::to_string).collect()

        // format!("{} ({})", product.name, product.quantity)
    }
}