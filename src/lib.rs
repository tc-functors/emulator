mod event;
mod function;
mod mutation;
mod route;
mod state;

use authorizer::Auth;
use composer::Entity;
use kit as u;
use std::str::FromStr;

pub async fn emulate(auth: Auth, entity: Option<String>, dev: bool, shell: bool) {
    let entity_str = u::maybe_string(entity, "function");
    let entity = Entity::from_str(&entity_str).unwrap();
    match entity {
        Entity::Function => function::run(&auth, dev, shell).await,
        Entity::State => state::run(&auth).await,
        Entity::Route => route::run(&auth).await,
        Entity::Mutation => mutation::run(&auth).await,
        Entity::Event => event::run(&auth).await,
        _ => todo!(),
    }
}

pub async fn init(profile: Option<String>, assume_role: Option<String>) -> Auth {
    match std::env::var("TC_ASSUME_ROLE") {
        Ok(_) => {
            let role = match assume_role {
                Some(r) => Some(r),
                None => {
                    let config = composer::config(&kit::pwd());
                    let p = u::maybe_string(profile.clone(), "default");
                    config.ci.roles.get(&p).cloned()
                }
            };
            Auth::new(profile.clone(), role).await
        }
        Err(_) => Auth::new(profile.clone(), assume_role).await,
    }
}
