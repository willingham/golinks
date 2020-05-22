use crate::schema::routes;

// Route as it's represented in the database
#[derive(Serialize, Deserialize, Queryable)]
pub struct Route {
    pub id: i32,
    pub slug: String,
    pub destination: String,
}

// Route as it's represented before insertion into database
#[derive(Deserialize, Insertable)]
#[table_name = "routes"]
pub struct InsertableRoute {
    pub slug: String,
    pub destination: String,
}