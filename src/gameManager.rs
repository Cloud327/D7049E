extern crate kiss3d;
extern crate nalgebra as na;

use ::nalgebra::{Translation3, Vector3};
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape, RigidBody};
use std::borrow::Borrow;
use std::iter::{FilterMap, Zip};
use std::path::Path;
use crate::ECS::attackRateComponent::AttackRateComponent;
use crate::ECS::colliderComponent::ColliderComponent;
use crate::ECS::moveComponent::MoveComponent;
use crate::ECS::rigidBodyComponent::RigidBodyComponent;
use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, typeComponent::TypeComponent, attackDamageComponent::AttackDamageComponent, renderableComponent::RenderableComponent};
use crate::mapManager::MapManager;
use crate::nodeHandler::NodeHandler;
use crate::physicsManager::{PhysicsManager, self};
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
        }
    }

    pub fn initialize(&mut self){

        // Create nodes for tower and enemy
        self.nodeHandler.addNodes(TypeEnum::towerType, Path::new("src/resources/mushroom.obj"), Path::new("src/resources/mushroom.mtl"));
        self.nodeHandler.addNodes(TypeEnum::enemyType, Path::new("src/resources/bird.obj"), Path::new("src/resources/bird.mtl"));
        
        // Initialize map
        self.mapManager.parseMap();
        self.mapManager.drawMap(&mut self.window);

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.5, 0.7, 1.0);
        
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

            self.window.render();
            
        }

            //while self.window.render() {}
    }

    /* AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA FUCK
    fn getComponentVecIter<T: 'static, F: 'static>(&mut self) -> FilterMap<Zip<std::slice::IterMut<'_, Option<T>>, std::slice::IterMut<'_, Option<F>>>, Box::Option<(&mut T, &mut F)>>{
        let mut compList1 = self.entityManager.borrowComponentVecMut::<T>().unwrap();
        let mut compList2 = self.entityManager.borrowComponentVecMut::<F>().unwrap();
        let zip = compList1.iter_mut().zip(compList2.iter_mut());

        let iter = zip.filter_map(|(compList1, compList2)| Some((compList1.as_mut()?, compList2.as_mut()?)));
        return iter;
    } */

    /*

     

    */

    fn moveEnemies(&mut self){
        let mut renderCompList = self.entityManager.borrowComponentVecMut::<RenderableComponent>().unwrap();
        let mut rigidCompList = self.entityManager.borrowComponentVecMut::<RigidBodyComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let mut moveCompList = self.entityManager.borrowComponentVecMut::<MoveComponent>().unwrap();
        let zip = renderCompList.iter_mut().zip(rigidCompList.iter_mut().zip(typeCompList.iter_mut().zip(moveCompList.iter_mut())));
        let iter = zip.filter_map(|(renderComp, (rigidComp, (typeComp, moveComp))),
                                                                            |Some((renderComp.as_mut()?, rigidComp.as_mut()?, typeComp.as_mut()?, moveComp.as_mut()?)));

        /* Loop through all objects and if it's an enemy then move it */
        for (renderComp, rigidComp, typeComp, moveComp) in iter {
            if matches!(typeComp.getType(), TypeEnum::enemyType){
                //moveEnemy()

                let node = renderComp.getSceneNode();

                
                let rigidBody = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).unwrap();
                //self.moveEnemy(rigidComp, moveComp);

                // Retrieves the rigidBody coordinates
                let t = GameManager::moveEnemy(rigidBody, moveComp);

                //let t = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).translation();
                // Sets the renderableComponent node coordinates to the rigidBody coordinates
                node.write().unwrap().set_local_translation(Translation3::new(t.0, t.1, t.2));
            }
            
        }
        
    }

    fn moveEnemy(rigidBody: &mut RigidBody, moveComp: &mut MoveComponent) -> (f32, f32, f32){
        
        let mut nextPoint = moveComp.getNextPoint();
        let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
        // If the enemy is located near enough the next point, then remove it and use the new next point in the sequence
        if  (nextPoint.0 - 0.3) < t.0 && t.0 < (nextPoint.0 + 0.3) && (nextPoint.1 - 0.3) < t.2 && t.2 < (nextPoint.1 + 0.3){
            nextPoint = moveComp.popAndGetNextPoint();
        }

        let mut velocity = (0.0, 0.0, 0.0);
        if t.0 < nextPoint.0{
            velocity.0 = 0.01;
            
        } else if t.0 > nextPoint.0{
            velocity.0 = -0.01;
        }

        if t.2 < nextPoint.1 {
            velocity.2 = 0.01;
        } else if t.2 > nextPoint.1 {
            velocity.2 = -0.01;
        }

        rigidBody.set_linvel(vector![velocity.0, velocity.1, velocity.2], true);

        return (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);

    }

    fn doEvent(&mut self){
        // TODO: Make into loop??
        let event = self.eventManager.readEvent();

        // Makes the object 
        if let EventEnum::takeDamageEvent{id, damage} = event {
            let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = healthCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(healthComp, idComp)| Some((healthComp.as_mut()?, idComp.as_mut()?)));
            for (healthComp, idComp) in iter {
                if idComp.getId() == id {
                    println!("health {} at id: {}", healthComp.getHealth(), id);
                    healthComp.decreaseHealth(damage);
                    println!("health {} at id: {}", healthComp.getHealth(), id);
                } else {
                    println!("No such id");
                }
            }
        }

        // Do attack with all object of type = towers
        // Do attack = create projectile object with heading towards x, y
        if let EventEnum::towerAttackEvent{x, y, z} = event {
            let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = typeCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(typeComp, idComp)| Some((typeComp.as_mut()?, idComp.as_mut()?)));
            for (typeComp, idComp) in iter {
                if matches!(typeComp.getType(), TypeEnum::towerType{}){
                    println!("found tower at id: {}", idComp.getId());
                    // TODO: Do attack
                }
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


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
            self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
            self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(10));
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
        let enemy = self.entityManager.newObject();
        self.entityManager.addComponentToObject(enemy, TypeComponent::new(TypeEnum::enemyType));
        self.entityManager.addComponentToObject(enemy, AttackDamageComponent::new(1));
        self.entityManager.addComponentToObject(enemy, AttackRateComponent::new(1));
        self.entityManager.addComponentToObject(enemy, HealthComponent::new(30));
        self.entityManager.addComponentToObject(enemy, MoveComponent::new(2, self.mapManager.findPath().unwrap()));

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