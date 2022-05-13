extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::text::Font;
use ::nalgebra::{Translation3, Vector3};
use rand::Rng;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape, RigidBody};
use std::collections::HashMap;
use std::ops::Not;
use std::path::Path;
use crate::ECS::Components::attackRateComponent::AttackRateComponent;
use crate::ECS::Components::colliderComponent::ColliderComponent;
use crate::ECS::Components::moveComponent::MoveComponent;
use crate::ECS::Components::rigidBodyComponent::RigidBodyComponent;
use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, Components::healthComponent::HealthComponent, Components::idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, Components::typeComponent::TypeComponent, Components::attackDamageComponent::AttackDamageComponent, Components::renderableComponent::RenderableComponent};
use crate::gameStateEnum::GameStateEnum;
use crate::mapManager::MapManager;
use crate::nodeHandler::NodeHandler;
use crate::physicsManager::PhysicsManager;
use kiss3d::scene::SceneNode;
use kiss3d::{window::Window, event::Action};
use kiss3d::light::Light;
use kiss3d::event::{Key, MouseButton, WindowEvent};

use rapier3d::na::{self as nalgebra};
use na::{Matrix4, vector, UnitQuaternion, Point2, Point3};

use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};

/*
TODO:
fix rotations for enemies along the road 

collision events for:
    enemy - projectile
        send takeDamageEvent to enemy and remove the projectile
    enemy - base
        send takeDamageEvent to base and remove the enemy

fix timer or boundries for projectiles?

checkGame function

spawnWaveOfEnemies

make game parameters into a single global variable

if base's healthComponent = 0:      // Make sure we don't delete base object before saying game over
    game over

*/



pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
    mapManager: MapManager,
    window: Window,
    physicsManager: PhysicsManager,
    nodeHandler: NodeHandler,
    gameState: GameStateEnum,
    gameParameters: HashMap<&'static str, f32>,

}


impl GameManager{
    pub fn new() -> Self {
        let gameParameters = HashMap::from([
            ("towerAttackDamage", 10.0),
            ("enemyAttackDamage", 1.0),
            ("enemyHeight", 1.0),
            ("towerHeight", 0.3),
        ]);
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
            mapManager: MapManager::new(),
            window: Window::new("Game"),
            physicsManager: PhysicsManager::new(),
            nodeHandler: NodeHandler::new(),
            gameState: GameStateEnum::ongoing,
            gameParameters: gameParameters,
        }
    }


    pub fn initialize(&mut self){

        // Create nodes for tower and enemy
        self.nodeHandler.addNodes(TypeEnum::towerType, Path::new("src/resources/mushroom.obj"), Path::new("src/resources/mushroom.mtl"));
        self.nodeHandler.addNodes(TypeEnum::enemyType, Path::new("src/resources/bird.obj"), Path::new("src/resources/bird.mtl"));
        self.nodeHandler.addNodes(TypeEnum::baseType, Path::new("src/resources/castle-tower.obj"), Path::new("src/resources/castle-tower.mtl"));
        self.nodeHandler.addNodes(TypeEnum::projectileType, Path::new("src/resources/genji-shuriken.obj"), Path::new("src/resources/genji-shuriken.mtl"));
        
        // Initialize map
        self.mapManager.parseMap();
        self.mapManager.drawMap(&mut self.window);
        self.createBase();

        // Spawns the towers specified in the map matrix
        let towerCoordList = self.mapManager.getTowerLocations();
        for coords in towerCoordList{
            self.spawnTower(coords.0, *self.gameParameters.get("towerHeight").unwrap(), coords.1);
        }

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.5, 0.7, 1.0);
        
    }

    pub fn gameloop(&mut self){

        // thread for towers to attack
        let (txTowerAttack, rxTowerAttack) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let val = String::from("kill enemies now please :)");
                txTowerAttack.send(val).unwrap();
                thread::sleep(Duration::from_millis(5000));
            }
        });

        loop{

            // Check TowerAttack thread for receives
            let receivedTowerAttack = rxTowerAttack.try_recv();
            match receivedTowerAttack {
                Ok(_) => self.checkEnemies(),
                Err(_) => (),
            }

            self.physicsManager.step();

            //self.updateNodes();
            
            // Adds functionality to close window by pressing escape
            let escape = self.window.get_key(Key::Escape);
            if matches!(escape, Action::Press){
                break;
            }   

            // On some key press, spawn tower on random empty tile
            let space = self.window.get_key(Key::Space);
            if matches!(space, Action::Press) {
                let nextTowerLocation = self.mapManager.nextTowerLocation();
                match nextTowerLocation {
                    Ok(n) => self.spawnTower(n.0 ,0.5,n.1),
                    Err(n) => println!("{}",n),
                }                
            }

            //self.checkGame();

            while !self.eventManager.eventBufferIsEmpty(){
                self.doEvent();
            }

            self.updateNodes();


            if matches!(self.gameState, GameStateEnum::won){
                let font = Font::default();
                let pos = Point2::new(self.window.width()as f32/2.0 , self.window.height()as f32/2.0);
                self.window.draw_text("You win! :D", &pos, 250.0, &font, &Point3::new(0.6, 0.0, 0.6));
            }
            else if matches!(self.gameState, GameStateEnum::lost){
                let font = Font::default();
                let pos = Point2::new(self.window.width()as f32/2.0 , self.window.height()as f32/2.0);
                self.window.draw_text("You lost! :(", &pos, 250.0, &font, &Point3::new(0.0, 0.0, 0.4));
            }

            self.window.render();
        }
    }


    fn checkEnemies(&mut self){
        let mut enemyOnMap = false;

        //let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
        let mut rigidBodyCompList = self.entityManager.borrowComponentVecMut::<RigidBodyComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let zip = rigidBodyCompList.iter_mut().zip(typeCompList.iter_mut());

        let iter = zip.filter_map
        (|(rigidBodyComp, typeComp)| Some((rigidBodyComp.as_mut()?, typeComp.as_mut()?)));
        for (rigidBodyComp, typeComp) in iter {
            if matches!(typeComp.getType(), TypeEnum::enemyType){
                let position = self.physicsManager.getRigidBody(rigidBodyComp.getRigidBodyHandle()).unwrap().translation();
                self.eventManager.sendEvent(EventEnum::towerAttackEvent { xTarget: position[0], yTarget: position[1], zTarget: position[2] });
                enemyOnMap = true;
                break;
            }
        }

        if !enemyOnMap{
            // Spawn new wave or won game
            // if no more waves:
                self.gameState = GameStateEnum::won;
            
        }
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
                self.spawnProjectile(xTarget, yTarget, zTarget, 
                                        coords.0, *self.gameParameters.get("enemyHeight").unwrap(), coords.1);
                println!("Spawned projectile!")

            }
        }


        // Create the necessary components for a tower
        // if let EventEnum::spawnTowerEvent{x, y, z} = event {
        //     self.spawnTower(x as f32, y as f32, z as f32);
        // }


        // Create the necessary components for an enemy and sets translation at start point of map
        // if let EventEnum::spawnEnemyEvent = event {
        //     self.spawnEnemy();
        // }


        // All events here
    }


    fn updateNodes(&mut self){
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
            if matches!(typeComp.getType(), TypeEnum::projectileType){
                let node = renderComp.getSceneNode();
                let rigidBody = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).unwrap();
                let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
                node.write().unwrap().set_local_translation(Translation3::new(t.0, t.1, t.2));
                node.write().unwrap().prepend_to_local_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.6));
            }
        }
    }


    fn moveEnemy(rigidBody: &mut RigidBody, moveComp: &mut MoveComponent) -> (f32, f32, f32){
        
        let mut nextPoint = moveComp.getNextPoint();
        let speed = moveComp.getSpeed();
        let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
        // If the enemy is located near enough the next point, then remove it and use the new next point in the sequence
        if  (nextPoint.0 - 0.3) < t.0 && t.0 < (nextPoint.0 + 0.3) && (nextPoint.1 - 0.3) < t.2 && t.2 < (nextPoint.1 + 0.3){
            nextPoint = moveComp.popAndGetNextPoint();
        }

        let mut velocity = (0.0, 0.0, 0.0);
        if t.0 < nextPoint.0{
            velocity.0 = speed;
            
        } else if t.0 > nextPoint.0{
            velocity.0 = -speed;
        }

        if t.2 < nextPoint.1 {
            velocity.2 = speed;
        } else if t.2 > nextPoint.1 {
            velocity.2 = -speed;
        }

        rigidBody.set_linvel(vector![velocity.0, velocity.1, velocity.2], true);

        return (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);

    }


    fn createBase(&mut self){
        let endCoords = self.mapManager.getEnd();

        let base = self.entityManager.newObject();
        self.entityManager.addComponentToObject(base, TypeComponent::new(TypeEnum::baseType));
        self.entityManager.addComponentToObject(base, HealthComponent::new(20));

        self.createRenderComponent(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, 0.007);
        self.createRigidBodyAndColliderComponents(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, (0.0, 0.0, 0.0));

    }


    fn spawnProjectile(&mut self, xTarget:  f32, yTarget: f32, zTarget: f32, xOrigin: f32, yOrigin: f32, zOrigin: f32){
        //TODO: Add ability to move
        let projectile = self.entityManager.newObject();
        self.entityManager.addComponentToObject(projectile, TypeComponent::new(TypeEnum::projectileType));
        self.entityManager.addComponentToObject(projectile, AttackDamageComponent::new(*self.gameParameters.get("towerAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(projectile, MoveComponent::newWithTarget(20.0, (xTarget, yTarget, zTarget)));

        let speed = 20.0;
        
        let pythagoras = ((xTarget-xOrigin).powf(2.0)+(yTarget-yOrigin).powf(2.0)+(zTarget-zOrigin).powf(2.0)).sqrt();
        let xVelocity = (xTarget-xOrigin) * speed / pythagoras;
        let yVelocity = (yTarget-yOrigin) * speed / pythagoras;
        let zVelocity = (zTarget-zOrigin) * speed / pythagoras;

        self.createRenderComponent(projectile, TypeEnum::projectileType,  xOrigin, yOrigin, zOrigin, 0.06);
        self.createRigidBodyAndColliderComponents(projectile, TypeEnum::projectileType,  xOrigin, yOrigin, zOrigin, (xVelocity, yVelocity, zVelocity));
    }


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(*self.gameParameters.get("towerAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(1));

        self.createRenderComponent(tower, TypeEnum::towerType, x, y, z, 0.6)
    }



    fn spawnEnemy(&mut self){
        let enemy = self.entityManager.newObject();
        self.entityManager.addComponentToObject(enemy, TypeComponent::new(TypeEnum::enemyType));
        self.entityManager.addComponentToObject(enemy, AttackDamageComponent::new(*self.gameParameters.get("enemyAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(enemy, HealthComponent::new(30));
        self.entityManager.addComponentToObject(enemy, MoveComponent::newWithPath(1.0, self.mapManager.findPath().unwrap()));
        
        let startCoords = self.mapManager.getStart();
        self.createRenderComponent(enemy, TypeEnum::enemyType, startCoords.0, *self.gameParameters.get("enemyHeight").unwrap(), startCoords.1, 0.5);
        self.createRigidBodyAndColliderComponents(enemy, TypeEnum::enemyType, startCoords.0, *self.gameParameters.get("enemyHeight").unwrap(), startCoords.1, (0.0, 0.0, 0.0));
    }


    fn createRenderComponent(&mut self, id: usize, objectType: TypeEnum, x: f32, y: f32, z: f32, scale: f32){
        let temp = self.nodeHandler.getNames(objectType).unwrap();
        let names = temp.clone();
        let meshManager = self.nodeHandler.getMeshManager(objectType).unwrap();
        let mut groupNode = self.window.add_group();

        // Build a group node from individual mesh parts
        for name in names{
            let mesh = meshManager.get(name.as_str()).unwrap();
            groupNode.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
        }
        groupNode.set_local_translation(Translation3::new(x, y, z));
        groupNode.set_local_scale(scale, scale, scale);

        // Set the color
        if matches!(objectType, TypeEnum::enemyType){
            groupNode.set_color(rand::thread_rng().gen_range(0.0..0.2), rand::thread_rng().gen_range(0.0..0.2), rand::thread_rng().gen_range(0.0..0.2));
        }
        else if matches!(objectType, TypeEnum::projectileType){
            groupNode.set_color(0.2, 0.2, 0.2);
        }
        else{
            groupNode.set_color(rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0));
        }

        // Create the component
         self.entityManager.addComponentToObject(id, RenderableComponent::new(groupNode));
    }


    fn createRigidBodyAndColliderComponents(&mut self, id: usize, objectType: TypeEnum, x: f32, y: f32, z: f32, velocity: (f32, f32, f32)){
        // Add RigidBody to PhysicsManager and RigidBodyHandle to RigidBodyComponent (like an index) with a translation 
        let body = RigidBodyBuilder::new(RigidBodyType::Dynamic);
        let mut body = body.translation(vector![x, y, z]).build();

        // Sets a velocity for projectiles
        if matches!(objectType, TypeEnum::projectileType){
            let body = body.set_linvel(vector![velocity.0, velocity.1, velocity.2], true);
        }

        let rigidBodyHandle = self.physicsManager.addRigidBody(body);
        self.entityManager.addComponentToObject(id, RigidBodyComponent::new(rigidBodyHandle));

        // Add Collider to PhysicsManager and ColliderHandle to ColliderComponent (like an index) with a translation 
        let collider = ColliderBuilder::new(ColliderShape::ball(1.0));
        let collider = collider.translation(vector![x, y, z]).sensor(true).build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        self.entityManager.addComponentToObject(id, ColliderComponent::new(colliderHandle));
    }

}


pub fn test(){

    let mut gm = GameManager::new();
    gm.initialize();

    gm.spawnEnemy();
    //gm.spawnTower(2.0, 0.3, 4.0);
    //gm.spawnProjectile(5.0, 5.0, 5.0, 3.0, 1.0, 10.0);

    gm.gameloop();

    // gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z: 2});
    // gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    
    // gm.doEvent();
    // gm.doEvent();
}