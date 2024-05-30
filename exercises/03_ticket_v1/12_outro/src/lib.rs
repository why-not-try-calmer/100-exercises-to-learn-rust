// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
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
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        let mut order = Order {
            product_name: String::from(""),
            quantity: 0,
            unit_price: 0,
        };
        order.set_product_name(product_name);
        order.set_quantity(quantity);
        order.set_unit_price(unit_price);
        order

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

    pub fn set_product_name(&mut self, product_name: String) {
        if product_name.trim().is_empty() {
            panic!("Name cannot be empty")
        }
        if product_name.len() > 300 {
            panic!("Name cannot be longer than 300 bytes")
        }
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        if (quantity) <= 0 {
            panic!("Quantity cannot be zero or below")
        }
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, price: u32) {
        if (price) <= 0 {
            panic!("Unit price cannot be zero or below")
        }
        self.unit_price = price;
    }
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
