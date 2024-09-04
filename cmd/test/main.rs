use template::hello_world;

fn main() {
    let name = "test";
    hello_world(name);
}

#[derive(GraphQLFilter)]
struct Collection {
    #[graphql:filter]
    id: Uuid,
    #[graphql:filter]
    name: String,
    #[graphql:filter,ref:name:Field,ref:type:OnToMany]
    fields: Vec<String>,
}

#[derive(GraphQLFilter)]
struct Field {
    #[graphql:filter]
    id: Uuid,
    collection_id: Uuid,
    value_type: String,
    name: String,
}
