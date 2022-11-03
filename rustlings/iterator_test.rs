use std::collections::HashMap;

fn get_vec_map() -> Vec<HashMap<String, i32>> {
 
	let mut other1 = HashMap::new();
	other1.insert(String::from("other1_1"), 1);
	other1.insert(String::from("other1_2"), 2);

	let mut other2 = HashMap::new();
	other2.insert(String::from("other2_3"), 3);
	other2.insert(String::from("other2_4"), 4);

	let mut other3 = HashMap::new();
	other3.insert(String::from("other3_5"), 4);
	other3.insert(String::from("other3_6"), 4);
	
	vec![other1, other2, other3]
}

fn main() {
 
	let collection = get_vec_map();
	//统计出value是4的个数
	//Vec<HashMap>这是两层集合，因此需要两次iterator
	let ret:usize = collection.iter()
                            .map(|hashmap| {
                                              hashmap.iter()
                                                     .filter(|(_k, &v)| v==4)
                                                     .collect::<Vec<_>>().len()
                                           }
                               )
                            .sum();
	
	println!("{:?}",ret);
}
