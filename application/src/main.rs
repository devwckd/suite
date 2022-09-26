#![feature(is_some_with)]
#![feature(drain_filter)]

mod actors;
mod domains;
mod handlers;
mod maestro;
mod repositories;
mod routes;

use std::{net::SocketAddr, sync::Arc};

use axum::Extension;
use log::LevelFilter;

use crate::{
    actors::deploy_instance::DeployUnitHandle,
    handlers::{
        blueprints::{DefaultBlueprintHandler, DynBlueprintHandler},
        env::{DefaultEnvHandler, DynEnvHandler},
        instance::{DefaultInstanceHandler, DynInstanceHandler},
        unit::DockerUnitHandler,
        version::{DefaultVersionHandler, DynVersionHandler},
    },
    repositories::{
        blueprint::MemoryBlueprintRepository, env::MemoryEnvRepository,
        instance::MemoryInstanceRepository, unit::MemoryUnitRepository,
        version::MemoryVersionRepository,
    },
};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let docker_api = bollard::Docker::connect_with_local_defaults().unwrap();

    let blueprint_repository = Arc::new(MemoryBlueprintRepository::default());
    let env_repository = Arc::new(MemoryEnvRepository::default());
    let instance_repository = Arc::new(MemoryInstanceRepository::default());
    let unit_repository = Arc::new(MemoryUnitRepository::default());
    let version_repository = Arc::new(MemoryVersionRepository::default());

    let unit_handler = Arc::new(DockerUnitHandler {
        unit_repository: unit_repository.clone(),
        docker_api: docker_api.clone(),
    });

    let deploy_unit_handle = DeployUnitHandle::new(unit_handler.clone());

    let blueprint_handler = Arc::new(DefaultBlueprintHandler {
        blueprint_repository: blueprint_repository.clone(),
    }) as DynBlueprintHandler;

    let env_handler = Arc::new(DefaultEnvHandler {
        env_repository: env_repository.clone(),
    }) as DynEnvHandler;

    let instance_handler = Arc::new(DefaultInstanceHandler {
        blueprint_repository: blueprint_repository.clone(),
        deploy_unit_handle: deploy_unit_handle.clone(),
        env_repository: env_repository.clone(),
        instance_repository: instance_repository.clone(),
        version_repository: version_repository.clone(),
    }) as DynInstanceHandler;

    let version_handler = Arc::new(DefaultVersionHandler {
        blueprint_repository: blueprint_repository.clone(),
        env_repository: env_repository.clone(),
        version_repository: version_repository.clone(),
    }) as DynVersionHandler;

    let router = axum::Router::new()
        .nest("/blueprints", routes::blueprint::router())
        .nest("/envs", routes::env::router())
        .nest("/instances", routes::instance::router())
        .nest("/versions", routes::version::router())
        .layer(Extension(blueprint_handler))
        .layer(Extension(env_handler))
        .layer(Extension(instance_handler))
        .layer(Extension(version_handler));

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    log::info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
