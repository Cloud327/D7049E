#![allow(non_snake_case)]

use std::{iter::FilterMap, num::ParseIntError};

mod assetTest;
mod buttonTest;
mod gameManager;
mod nodeHandler;
mod physicsManager;
mod tileEnum;
mod mapManager;
mod ECS;
mod colliderTest;
mod gameStateEnum;


fn main() {

    gameManager::test();
    //colliderTest::run();


    //assetTest::test();
    //buttonTest::test();

}
