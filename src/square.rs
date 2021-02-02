struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) {
        self.width * self.height
    }
}