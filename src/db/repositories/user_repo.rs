use mongodb::{
    bson::{doc, to_bson}, 
    Collection, Database
};
use crate::model::model::User;
use tracing::{debug, error, info};

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        debug!("Creating new UserRepository");
        let collection = db.collection("users");
        Self { collection }
    }

    pub async fn create_user(&self, user: User) -> mongodb::error::Result<User> {
        info!("Creating new user with email: {}", user.email);
        match self.collection.insert_one(user.clone()).await {
            Ok(_) => {
                debug!("User created successfully: id={}", user.id);
                Ok(user)
            },
            Err(e) => {
                error!("Failed to create user: {}", e);
                Err(e)
            }
        }
    }

    pub async fn find_by_id(&self, id: u64) -> mongodb::error::Result<Option<User>> {
        debug!("Finding user by id: {}", id);
        let id_bson = to_bson(&id).unwrap();
        let result = self.collection
            .find_one(doc! { "id": id_bson })
            .await;
        
        match &result {
            Ok(Some(_)) => debug!("User with id {} found", id),
            Ok(None) => debug!("User with id {} not found", id),
            Err(e) => error!("Error finding user by id {}: {}", id, e),
        }
        
        result
    }

    pub async fn find_by_email(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        debug!("Finding user by email: {}", email);
        let result = self.collection
            .find_one(doc! { "email": email })
            .await;
            
        match &result {
            Ok(Some(_)) => debug!("User with email {} found", email),
            Ok(None) => debug!("User with email {} not found", email),
            Err(e) => error!("Error finding user by email {}: {}", email, e),
        }
        
        result
    }
    
    pub async fn update_user(&self, user: User) -> mongodb::error::Result<User> {
        info!("Updating user with id: {}", user.id);
        let id_bson = to_bson(&user.id).unwrap();
        match self.collection
            .replace_one(doc! { "id": id_bson }, user.clone())
            .await
        {
            Ok(result) => {
                if result.matched_count > 0 {
                    debug!("User with id {} updated successfully", user.id);
                } else {
                    debug!("No user with id {} found to update", user.id);
                }
                Ok(user)
            },
            Err(e) => {
                error!("Failed to update user with id {}: {}", user.id, e);
                Err(e)
            }
        }
    }
    
    pub async fn delete_user(&self, id: u64) -> mongodb::error::Result<bool> {
        info!("Deleting user with id: {}", id);
        let id_bson = to_bson(&id).unwrap();
        match self.collection
            .delete_one(doc! { "id": id_bson })
            .await
        {
            Ok(result) => {
                let deleted = result.deleted_count > 0;
                if deleted {
                    debug!("User with id {} deleted successfully", id);
                } else {
                    debug!("No user with id {} found to delete", id);
                }
                Ok(deleted)
            },
            Err(e) => {
                error!("Failed to delete user with id {}: {}", id, e);
                Err(e)
            }
        }
    }
} 