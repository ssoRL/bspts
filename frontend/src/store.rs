use crate::data_store::store_item::StoreItem;
use data::user::User;

#[derive(Clone)]
pub struct Store {
    pub user: StoreItem<User>,
}