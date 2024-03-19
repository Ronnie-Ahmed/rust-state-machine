mod balances;
fn main() {
	let name="Ronnie".to_string();
	let mut bal = balances::Pallet::new();
	bal.set_balance("Ronnie".to_string(), 125);  // Convert the string literal to a String
	println!("{:?}", bal.see_balance(&"Ronnie".to_string()));  // Convert the string literal to a String

}
