#![allow(non_snake_case)]
mod assetTest;
mod buttonTest;
mod gameManager;
mod nodeHandler;
mod physicsManager;
mod mapManager;
mod ECS;
mod colliderTest;

extern crate rand;

fn main() {
    // assetTest::test();
    gameManager::test();
    //colliderTest::run();


    //assetTest::test();
    //buttonTest::test();

}
