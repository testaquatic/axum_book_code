pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub struct CategoryModel {
    pub name: String,
}

pub struct ProductModel {
    pub id: i32,
    pub title: String,
    pub price: i32,
    pub category: String,
}
