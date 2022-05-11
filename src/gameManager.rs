extern crate kiss3d;
extern crate nalgebra as na;

use ::nalgebra::{Translation3, Vector3};
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape};
use std::path::Path;
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
add transformations(?) for spawnEnemy and spawnProjectile

spawnWaveOfEnemies

spawn tower with key press on random empty tile

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

}


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
        }
    }


    pub fn initialize(&mut self){

        // Set parameters
        self.towerAttackDamage = 10;
        self.enemyAttackDamage = 1;

        // Create nodes for tower and enemy
        self.nodeHandler.addNodes(TypeEnum::towerType, Path::new("src/resources/mushroom.obj"), Path::new("src/resources/mushroom.mtl"));
        self.nodeHandler.addNodes(TypeEnum::enemyType, Path::new("src/resources/bird.obj"), Path::new("src/resources/bird.mtl"));
        // TODO: Add projectile here too
        
        // Initialize map
        self.mapManager.parseMap();
        self.mapManager.drawMap(&mut self.window);

        self.window.set_light(Light::StickToCamera);
        
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

        while true {
            self.physicsManager.step();

            //self.updateNodes();
            
            // Adds functionality to close window by pressing escape
            let escape = self.window.get_key(Key::Escape);
            if matches!(escape, Action::Press){
                break;
            }   

            // On some key press, spawn tower on random empty tile

            self.checkGame();

            self.window.render();
            
        }

            //while self.window.render() {}
    }


    fn checkGame(&self){
        /*
        enemyCount = 0
        Loop through all objects with renderable/rigidbody/collision components
            if object type = enemy:
                Send attackEvent/takeDamageEvent ? 
                if enemy is at end:
                    send takeTamageEvent to base
                enemyCount += 1

        if enemyCount = 0:
            Spawn new wave or won game
        
        if base's healthComponent = 0:  // Make sure we don't delete base object before saying game over
            game over
        
        */
 
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
        // Do attack = create projectile object with heading towards x, y
        if let EventEnum::towerAttackEvent{xTarget, yTarget, zTarget} = event {

            //Collect coords of all towers from map
            // loop through that list and spawn projectiles
                // Spawn projectile from x,y,z with heading towords xTarget, yTarget, zTarget
                //self.spawnProjectile(0 as f32, 0 as f32, 0 as f32);

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

    fn spawnProjectile(&mut self, x:  f32, y: f32, z: f32){
        //TODO: Add ability to move
        let projectile = self.entityManager.newObject();
        self.entityManager.addComponentToObject(projectile, TypeComponent::new(TypeEnum::projectileType));
        self.entityManager.addComponentToObject(projectile, AttackDamageComponent::new(self.towerAttackDamage));
        self.entityManager.addComponentToObject(projectile, MoveComponent::new(5));

        let temp = self.nodeHandler.getNames(TypeEnum::enemyType).unwrap();
        let names = temp.clone();

        let meshManager = self.nodeHandler.getMeshManager(TypeEnum::projectileType).unwrap();

        let mut sceneNodes: Vec<SceneNode> = Vec::new();
        let tup = self.mapManager.getStart();

        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            let mut temp = self.window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
            temp.set_local_translation(Translation3::new(x, y, z));
            sceneNodes.push(temp);
        }
        self.entityManager.addComponentToObject(projectile, RenderableComponent::new(sceneNodes));

        // Add RigidBody to PhysicsManager and RigidBodyHandle to RigidBodyComponent (like an index) with a translation 
        let body = RigidBodyBuilder::new(RigidBodyType::Dynamic);
        let rigidBodyHandle = self.physicsManager.addRigidBody(body.translation(vector![x, y, z]).build());
        self.entityManager.addComponentToObject(projectile, RigidBodyComponent::new(rigidBodyHandle));

        // Add Collider to PhysicsManager and ColliderHandle to ColliderComponent (like an index) with a translation 
        let collider = ColliderBuilder::new(ColliderShape::ball(1.0));
        let collider = collider.translation(vector![x, y, z]).build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        self.entityManager.addComponentToObject(projectile, ColliderComponent::new(colliderHandle));


    }


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(self.towerAttackDamage));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(1));

    
        let temp = self.nodeHandler.getNames(TypeEnum::towerType).unwrap();
        let names = temp.clone();

        let meshManager = self.nodeHandler.getMeshManager(TypeEnum::towerType).unwrap();

        let mut sceneNodes: Vec<SceneNode> = Vec::new();
        
        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            let mut temp = self.window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
            temp.set_local_translation(Translation3::new(x, y, z));
            sceneNodes.push(temp);
        }
        self.entityManager.addComponentToObject(tower, RenderableComponent::new(sceneNodes));
    }



    fn spawnEnemy(&mut self){
        // TODO: Add ability to move
        let enemy = self.entityManager.newObject();
        self.entityManager.addComponentToObject(enemy, TypeComponent::new(TypeEnum::enemyType));
        self.entityManager.addComponentToObject(enemy, AttackDamageComponent::new(self.enemyAttackDamage));
        self.entityManager.addComponentToObject(enemy, AttackRateComponent::new(1));
        self.entityManager.addComponentToObject(enemy, HealthComponent::new(30));
        self.entityManager.addComponentToObject(enemy, MoveComponent::new(2));

        let temp = self.nodeHandler.getNames(TypeEnum::enemyType).unwrap();
        let names = temp.clone();

        let meshManager = self.nodeHandler.getMeshManager(TypeEnum::enemyType).unwrap();

        let mut sceneNodes: Vec<SceneNode> = Vec::new();
        let tup = self.mapManager.getStart();

        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            let mut temp = self.window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
            temp.set_local_translation(Translation3::new(tup.0, 1.0, tup.1));
            sceneNodes.push(temp);
        }
        self.entityManager.addComponentToObject(enemy, RenderableComponent::new(sceneNodes));

        // Add RigidBody to PhysicsManager and RigidBodyHandle to RigidBodyComponent (like an index) with a translation 
        let body = RigidBodyBuilder::new(RigidBodyType::Dynamic);
        let rigidBodyHandle = self.physicsManager.addRigidBody(body.translation(vector![tup.0, 1.0, tup.1]).build());
        self.entityManager.addComponentToObject(enemy, RigidBodyComponent::new(rigidBodyHandle));

        // Add Collider to PhysicsManager and ColliderHandle to ColliderComponent (like an index) with a translation 
        let collider = ColliderBuilder::new(ColliderShape::ball(1.0));
        let collider = collider.translation(vector![tup.0, 1.0, tup.1]).build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        self.entityManager.addComponentToObject(enemy, ColliderComponent::new(colliderHandle));
    }
}


pub fn test(){

    let mut gm = GameManager::new();
    gm.initialize();

    gm.spawnEnemy();
    gm.spawnTower(2.0, 0.4, 4.0);

    gm.gameloop();

    // gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z: 2});
    // gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    
    // gm.doEvent();
    // gm.doEvent();
}