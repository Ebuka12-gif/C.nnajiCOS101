struct Total{
hp:u32, ibm:u32, toshiba:u32, dell:u32, amount:u32 
}

impl Total {
    fn sum (&self)->u32 {
        //use the . operator to fetch the value of a field via the self keyword
        self.hp * self.amount + self.ibm * self.amount + self.toshiba * self.amount + self.dell * self.amount
    }
}
fn main() {
    let cap = Total {
        hp:650000,
        ibm:755000,
        toshiba:550000,
        dell:850000,
        amount:3
    };
    
        print!("The price of 3 HP laptops is {}.\nThe price of 3 IBM laptops is {}.\nThe price of 3 Toshiba laptops is {}.\nThe price of 3 dell laptops is {}.\nYour total cost is {}",cap.hp, cap.ibm, cap.toshiba, cap.dell, cap.sum());
}
