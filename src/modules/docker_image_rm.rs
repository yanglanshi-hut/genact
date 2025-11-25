//! Pretend to delete Docker images
use async_trait::async_trait;
use rand::seq::IndexedRandom;
use rand::{Rng, rng};
use rust_i18n::t;

use crate::args::AppConfig;
use crate::data::{DOCKER_PACKAGES_LIST, DOCKER_TAGS_LIST};
use crate::generators::gen_hex_string;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct DockerImageRm;

#[async_trait(?Send)]
impl Module for DockerImageRm {
    fn name(&self) -> &'static str {
        "docker_image_rm"
    }

    fn signature(&self) -> String {
        "docker image rm 04880416f1bf".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();
        let num_packages = rng.random_range(20..100);
        // Choose `num_packages` packages, non-repeating and in random order
        let chosen_packages: Vec<_> = DOCKER_PACKAGES_LIST
            .choose_multiple(&mut rng, num_packages)
            .collect();

        for &package_name in &chosen_packages {
            let sleep_length = rng.random_range(500..5000);
            let package_tag: &&str = DOCKER_TAGS_LIST.choose(&mut rng).unwrap();

            print(format!("{}: {package_name}:{package_tag}", t!("modules.docker_image_rm.untagged"))).await;
            newline().await;
            print(format!(
                "{}: {package_name}:{package_tag}@sha256:{hash}",
                t!("modules.docker_image_rm.untagged"),
                hash = gen_hex_string(&mut rng, 64)
            ))
            .await;
            newline().await;

            let num_hashes = rng.random_range(10..30);
            let mut index = 0;
            while index < num_hashes {
                print(format!(
                    "{}: sha256:{hash}",
                    t!("modules.docker_image_rm.deleted"),
                    hash = gen_hex_string(&mut rng, 64)
                ))
                .await;
                newline().await;

                index += 1;
            }

            csleep(sleep_length).await;

            if appconfig.should_exit() {
                return;
            }
        }
    }
}
