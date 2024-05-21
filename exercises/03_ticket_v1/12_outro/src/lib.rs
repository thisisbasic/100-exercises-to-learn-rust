// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::validate_product_name(&product_name);
        Self::validate_quantity(&quantity);
        Self::validate_unit_price(&unit_price);
        
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, value: String) {
        Self::validate_product_name(&value);
        self.product_name = value;
    }

    pub fn set_quantity(&mut self, value: u32) {
        Self::validate_quantity(&value);
        self.quantity = value;
    }

    pub fn set_unit_price(&mut self, value: u32) {
        Self::validate_unit_price(&value);
        self.unit_price = value;
    }

    fn validate_product_name(value: &String) {
        let max_value_size: u16 = 300;
        if value.is_empty() {
            panic!("Product can not have empty name!");
        }
        if value.len() > 300 as usize {
            panic!("Product name can not be greater than {max_value_size}");
        }
    }
    
    fn validate_quantity(value: &u32){
        if value <= &0 {
            panic!("Quantity must be greater than zero.");
        }
    }

    fn validate_unit_price(value: &u32){
        if value <= &0 {
            panic!("Unit price must be greater than zero.");
        }
    }

}