extern crate kiss3d;
extern crate nalgebra as na;

use ::nalgebra::{Translation3, Vector3};
use rand::Rng;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape};
use std::path::Path;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;
use crate::ECS::Components::attackRateComponent::AttackRateComponent;
use crate::ECS::Components::colliderComponent::ColliderComponent;
use crate::ECS::Components::moveComponent::MoveComponent;
use crate::ECS::Components::rigidBodyComponent::RigidBodyComponent;
use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, Components::healthComponent::HealthComponent, Components::idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, Components::typeComponent::TypeComponent, Components::attackDamageComponent::AttackDamageComponent, Components::renderableComponent::RenderableComponent};
use crate::mapManager::MapManager;
use crate::nodeHandler::NodeHandler;
use crate::physicsManager::PhysicsManager;
use kiss3d::scene::SceneNode;
use kiss3d::{window::Window, event::Action};
use kiss3d::light::Light;
use kiss3d::event::{Key, MouseButton, WindowEvent};

use rapier3d::na::{self as nalgebra};
use na::{Matrix4, vector};


/*
TODO:
make enemy move along the road

checkGame function

towerAttackEvent
takedamageEvent
spawnProjectile function
get obj file for projectile

add transformations(?) for spawnEnemy and spawnProjectile

spawnWaveOfEnemies

spawn tower with key press on random empty tile

For tomorrow:
optimize the spawn functions (create help function for repetetive code)
use help function in spawnTower, also fix scale and maybe rotation
get obj

*/



pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
    mapManager: MapManager,
    window: Window,
    physicsManager: PhysicsManager,
    nodeHandler: NodeHandler,
    towerAttackDamage: usize,
    enemyAttackDamage: usize,
    enemyHeight: f32

}

unsafe impl Sync for GameManager {}
unsafe impl Send for GameManager {}

impl GameManager{
    pub fn new() -> Self {
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
            mapManager: MapManager::new(),
            window: Window::new("Game"),
            physicsManager: PhysicsManager::new(),
            nodeHandler: NodeHandler::new(),
            towerAttackDamage: 0,
            enemyAttackDamage: 0,
            enemyHeight: 0.0,
        }
    }


    pub fn initialize(&mut self){

        // Set parameters
        self.towerAttackDamage = 10;
        self.enemyAttackDamage = 1;
        self.enemyHeight = 1.0;

        // Create nodes for tower and enemy
        self.nodeHandler.addNodes(TypeEnum::towerType, Path::new("src/resources/mushroom.obj"), Path::new("src/resources/mushroom.mtl"));
        self.nodeHandler.addNodes(TypeEnum::enemyType, Path::new("src/resources/bird.obj"), Path::new("src/resources/bird.mtl"));
        self.nodeHandler.addNodes(TypeEnum::baseType, Path::new("src/resources/castle-tower.obj"), Path::new("src/resources/castle-tower.mtl"));
        // TODO: Add projectile here too
        
        // Initialize map
        self.mapManager.parseMap();
        self.mapManager.drawMap(&mut self.window);
        self.createBase();

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.5,0.7,1.0)
    }

    /* PSEUDOCODE
    initialize(){
        createHome/Base()       // Create the object home/base, place at End tile and add health component, if hp = 0 -> lose game
        loadMap()       
    }

    loadMap(){
        mapManager.initialize()
    }

    // Will load the map matrix, map texture(?) and do various init stuff like spawn towers, enemies, etc
    mapmanager.initialize(){
        do stuff...
        createObjects()
    }

    gameLoop(){
        while(True){

            doEvent()       // Executes events from event buffer

            physicsManager.step()
            for renderable in entityManager.componentVec[renderableComponents]{
                renderable.update()     // Updates the sceneNodes after rigidBody which has been altered by physics stuff
                                        // Add movement for enemies?
                renderable.render()     // Draw in window?
            }
            
            checkGame()         // Check win/lose/wave conditions, maybe start new wave? 

            collisionCheck()        // Check collision events

        }
    }
    */ 


    pub fn gameloop(&mut self){

        // thread for bird spawning
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let val = String::from("spawn bird now please :)");
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        });

        // actual gameloop
        loop {
            let received = rx.try_recv();
            match received {
                Ok(_) => self.spawnEnemy(),
                Err(_) => (),
            }

            self.physicsManager.step();

            //self.updateNodes();
            
            // Adds functionality to close window by pressing escape
            let escape = self.window.get_key(Key::Escape);
            let space = self.window.get_key(Key::Space);
            if matches!(escape, Action::Press){
                break;
            }   
			
			
            // On some key press, spawn tower on random empty tile
            if matches!(space, Action::Press) {
                let nextTowerLocation = self.mapManager.nextTowerLocation();
                match nextTowerLocation {
                    Ok(n) => self.spawnTower(n.0 ,0.3,n.1),
                    Err(n) => println!("{}",n),
                }                
            }

            self.checkGame();
            
            // while !self.eventManager.eventBufferIsEmpty(){
            //     self.doEvent();
            // }

            



            self.window.render();



            self.window.render();
            
        }

            //while self.window.render() {}
    }


    fn checkGame(&mut self){
        let mut enemyCount = 0;

        //let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
        let mut rigidBodyCompList = self.entityManager.borrowComponentVecMut::<RigidBodyComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let zip = rigidBodyCompList.iter_mut().zip(typeCompList.iter_mut());

        let iter = zip.filter_map
        (|(rigidBodyComp, typeComp)| Some((rigidBodyComp.as_mut()?, typeComp.as_mut()?)));
        for (rigidBodyComp, typeComp) in iter {
            if matches!(typeComp.getType(), TypeEnum::enemyType){
                let position = self.physicsManager.getRigidBody(rigidBodyComp.getRigidBodyHandle()).translation();
                // send towerAttackEvent or call attack function from here?
                self.eventManager.sendEvent(EventEnum::towerAttackEvent { xTarget: position[0], yTarget: position[1], zTarget: position[2] });
                enemyCount += 1;
            }
        }

        if enemyCount == 0{
            // Spawn new wave or won game
        }

        // if base's healthComponent = 0:  // Make sure we don't delete base object before saying game over
        //     game over

    }


    fn doEvent(&mut self){
        let event = self.eventManager.readEvent();

        if let EventEnum::takeDamageEvent{id, damage} = event {
            let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = healthCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(healthComp, idComp)| Some((healthComp.as_mut()?, idComp.as_mut()?)));
            for (healthComp, idComp) in iter {
                if idComp.getId() == id {
                    healthComp.decreaseHealth(damage);
                } else {
                    println!("No such id");
                }
            }
        }

        // Do attack with all object of type = towers
        // Do attack = create projectile object with heading towards x, y, z
        if let EventEnum::towerAttackEvent{xTarget, yTarget, zTarget} = event {
            let coordList = self.mapManager.getTowerLocations();
            for coords in coordList{
                self.spawnProjectile(xTarget as f32, yTarget as f32, zTarget as f32, 
                                        coords.0 as f32, self.enemyHeight as f32, coords.1 as f32);
                println!("Spawn projectile!")

            }
        }


        // Create the necessary components for a tower
        if let EventEnum::spawnTowerEvent{x, y, z} = event {
            self.spawnTower(x as f32, y as f32, z as f32);
        }


        // Create the necessary components for an enemy and sets translation at start point of map
        if let EventEnum::spawnEnemyEvent = event {
            self.spawnEnemy();
        }


        // All events here
    }


    fn createRenderComponents(&mut self, id: usize, objectType: TypeEnum, x: f32, y: f32, z: f32, scale: f32){
        let temp = self.nodeHandler.getNames(TypeEnum::baseType).unwrap();
        let names = temp.clone();

        let meshManager = self.nodeHandler.getMeshManager(objectType).unwrap();

        let mut groupNode = self.window.add_group();

        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            groupNode.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
        }
        groupNode.set_local_translation(Translation3::new(x, y, z));
        groupNode.set_local_scale(scale, scale, scale);
        // random color
        groupNode.set_color(rand::thread_rng().gen_range(0.0..0.2), rand::thread_rng().gen_range(0.0..0.2), rand::thread_rng().gen_range(0.0..0.2));

        self.entityManager.addComponentToObject(id, RenderableComponent::new(groupNode));

        // Add RigidBody to PhysicsManager and RigidBodyHandle to RigidBodyComponent (like an index) with a translation 
        let body = RigidBodyBuilder::new(RigidBodyType::Dynamic);
        let rigidBodyHandle = self.physicsManager.addRigidBody(body.translation(vector![x, y, z]).build());
        self.entityManager.addComponentToObject(id, RigidBodyComponent::new(rigidBodyHandle));

        // Add Collider to PhysicsManager and ColliderHandle to ColliderComponent (like an index) with a translation 
        let collider = ColliderBuilder::new(ColliderShape::ball(1.0));
        let collider = collider.translation(vector![x, y, z]).build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        self.entityManager.addComponentToObject(id, ColliderComponent::new(colliderHandle));
    }


    fn createBase(&mut self){
        let endCoords = self.mapManager.getEnd();

        let base = self.entityManager.newObject();
        self.entityManager.addComponentToObject(base, TypeComponent::new(TypeEnum::baseType));
        self.entityManager.addComponentToObject(base, HealthComponent::new(20));

        self.createRenderComponents(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, 0.007);

    }


    fn spawnProjectile(&mut self, xTarget:  f32, yTarget: f32, zTarget: f32, xOrigin: f32, yOrigin: f32, zOrigin: f32){
        //TODO: Add ability to move
        let projectile = self.entityManager.newObject();
        self.entityManager.addComponentToObject(projectile, TypeComponent::new(TypeEnum::projectileType));
        self.entityManager.addComponentToObject(projectile, AttackDamageComponent::new(self.towerAttackDamage));
        self.entityManager.addComponentToObject(projectile, MoveComponent::new(5));

        self.createRenderComponents(projectile, TypeEnum::projectileType,  xOrigin, yOrigin, zOrigin, 1.0);

    }


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(self.towerAttackDamage));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(1));

    
        let temp = self.nodeHandler.getNames(TypeEnum::towerType).unwrap();
        let names = temp.clone();

        let meshManager = self.nodeHandler.getMeshManager(TypeEnum::towerType).unwrap();

        let mut groupNode = self.window.add_group();

        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            let scale = rand::thread_rng().gen_range(0.3..0.7);
            groupNode.add_mesh(mesh, Vector3::new(scale, scale, scale));
        }
        groupNode.set_local_translation(Translation3::new(x, y, z));
        groupNode.set_color(rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0));
        self.entityManager.addComponentToObject(tower, RenderableComponent::new(groupNode));
    }



    fn spawnEnemy(&mut self){
        // TODO: Add ability to move
        let enemy = self.entityManager.newObject();
        self.entityManager.addComponentToObject(enemy, TypeComponent::new(TypeEnum::enemyType));
        self.entityManager.addComponentToObject(enemy, AttackDamageComponent::new(self.enemyAttackDamage));
        self.entityManager.addComponentToObject(enemy, AttackRateComponent::new(1));
        self.entityManager.addComponentToObject(enemy, HealthComponent::new(30));
        self.entityManager.addComponentToObject(enemy, MoveComponent::new(2));

        let startCoords = self.mapManager.getStart();
        // let startCoords = (rand::thread_rng().gen_range(0.0..15.0),rand::thread_rng().gen_range(0.0..15.0));

        self.createRenderComponents(enemy, TypeEnum::enemyType, startCoords.0, self.enemyHeight, startCoords.1, 0.5);
    }
}


pub fn test(){

    let mut gm = GameManager::new();
    gm.initialize();

    gm.spawnEnemy();
    // gm.spawnTower(2.0, 0.4, 4.0);

    gm.gameloop();

    

    // gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z: 2});
    // gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    
    // gm.doEvent();
    // gm.doEvent();
}



// pub fn waveTest(){
//     let mut gm = GameManager::new();
//     gm.initialize();



// }