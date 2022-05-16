extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::FirstPerson;
use kiss3d::text::Font;
use ::nalgebra::{Translation3, Vector3};
use rand::Rng;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape, RigidBody, ActiveEvents, CollisionEvent, ColliderHandle};
use std::collections::HashMap;
use std::ops::Not;
use std::path::Path;
use std::sync::mpsc::Sender;
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
use rapier3d::prelude::AngVector;
use na::{Matrix4, vector, UnitQuaternion, Point2, Point3};

use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};

/*
TODO:

fix collisionevents:
    removeObjects in entityManager
    remove sceneNode from renderableComponent

add game lost condition somewhere

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
            ("enemyHealth", 50.0),
            ("baseHealth", 10.0),
            ("enemyHeight", 1.0),
            ("towerHeight", 0.3),
            ("enemySpeed", 2.0),
            ("projectileSpeed", 10.0),
            ("towerAttackRate", 4.0),  // How many seconds between each attack
            ("finalWave", 10.0),
            ("currentWave", 1.0),
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

        // TODO: Create outer walls
        // Check len of mapMatrix and len of a mapMatrix col
        // build walls at coors (0,0-0,len), (0,0-colLen,0), (0,len-len,colLen), (colLen,0-len,colLen)
        self.createWalls();

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.5, 0.7, 1.0);
        
    }

    pub fn gameloop(&mut self){

        let eye = Point3::new(10.0f32, 10.0, 10.0);
        let at = Point3::origin();
        let mut first_person = FirstPerson::new(eye, at);

        // thread for towers to attack
        let (txTowerAttack, rxTowerAttack) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let val = String::from("kill enemies now please :)");
                txTowerAttack.send(val).unwrap();
                thread::sleep(Duration::from_millis(4000));     // Set to same as towerAttackRate
            }
        });

        // // thread for starting waves,,, TODO: REMOVE KILL EXTERMINATE, this will be done from somewhere else
        let (spawnEnemySender, spawnEnemyReciever) = mpsc::channel();
        let (nextWaveSender, nextWaveReciever) = mpsc::channel();
        // thread::spawn(move ||{
        //     loop {
        //         // wait 2000 ms and then start next wave
        //         thread::sleep(Duration::from_millis(5000));
        //         nextWaveSender.send(true).unwrap();
        //     }
        // });
        
        // thread for bird spawning
        thread::spawn(move || {
            let mut waveCounter = 0;
            let mut waveNumberEnemies = 2;
            loop {
                // only spawn a wave when told to do so by nextWaveReciever
                // wait until we recieve a call do spawn a wave
                nextWaveReciever.recv().unwrap(); 

                // spawns waveNumberEnemies enemies with 200 ms between each, 
                println!("wave {waveCounter} with {waveNumberEnemies} enemies");
                for _ in 0..waveNumberEnemies{
                    spawnEnemySender.send(true).unwrap();
                    thread::sleep(Duration::from_millis(2000));
                }

                // keep track of how many enemies to spawn and wavecounter
                waveNumberEnemies += 2;
                waveCounter += 1;
            }
        });

        loop{

            // Check TowerAttack thread for receives
            let receivedTowerAttack = rxTowerAttack.try_recv();
            match receivedTowerAttack {
                Ok(_) => self.checkEnemies(&nextWaveSender),
                Err(_) => (),
            }

            // spawn an enemy whenever we recieve a spawnEnemy call from the bird spawining thread
            let doSpawnEnemy = spawnEnemyReciever.try_recv();
            match doSpawnEnemy {
                Ok(_) => self.spawnEnemy(),
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


            let collision = self.physicsManager.getEvent();
            match collision{
                Some(collision) => self.collisionEvent(collision),
                None => (),
            }
            

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

            self.window.render_with_camera(&mut first_person);
        }
    }


    fn collisionEvent(&mut self, collision: CollisionEvent){
        let collider1 = collision.collider1();
        let collider2 = collision.collider2();
        let mut types: Vec<(TypeEnum, &mut IdComponent,&mut AttackDamageComponent, &mut ColliderComponent)> = Vec::new();

        let mut colliderCompList = self.entityManager.borrowComponentVecMut::<ColliderComponent>().unwrap();
        let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let mut damageCompList = self.entityManager.borrowComponentVecMut::<AttackDamageComponent>().unwrap();
        let zip = colliderCompList.iter_mut().zip(idCompList.iter_mut().zip(typeCompList.iter_mut().zip(damageCompList.iter_mut())));
        let iter = zip.filter_map(|(colliderComp, (idComp, (typeComp, damageComp))),
                                                                            |Some((colliderComp.as_mut()?, idComp.as_mut()?, typeComp.as_mut()?, damageComp.as_mut()?)));
        /* Loop through all objects and if it's an enemy then move it */
        for (colliderComp, idComp, typeComp, damageComp) in iter{
            if colliderComp.getColliderHandle().0 == collider1.0{ 
                types.push((typeComp.getType(), idComp, damageComp, colliderComp));
            }
            else if colliderComp.getColliderHandle().0 == collider2.0{ 
                types.push((typeComp.getType(), idComp, damageComp, colliderComp));
            }
        }

        let obj1 = types.pop();
        let obj2 = types.pop();

        if obj1.is_some() && obj2.is_some(){
            let obj1 = obj1.unwrap();
            let obj2 = obj2.unwrap();

            if matches!(obj1.0, TypeEnum::enemyType){
                if matches!(obj2.0, TypeEnum::projectileType){
                    println!("enemy & projectile");
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj1.1.getId(), damage: obj2.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                }
                else if matches!(obj2.0, TypeEnum::baseType){
                    println!("enemy & base");
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj2.1.getId(), damage: obj1.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                }
            }
    
            if matches!(obj1.0, TypeEnum::projectileType){
                if matches!(obj2.0, TypeEnum::enemyType){
                    println!("projectile & enemy");
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj2.1.getId(), damage: obj1.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                }
                else if matches!(obj2.0, TypeEnum::wallType){
                    println!("projectile & wall");
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                }
            }
    
            if matches!(obj1.0, TypeEnum::baseType){
                if matches!(obj2.0, TypeEnum::enemyType){
                    println!("base & enemy");
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj1.1.getId(), damage: obj2.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                }
            }
    
            if matches!(obj1.0, TypeEnum::wallType){
                if matches!(obj2.0, TypeEnum::projectileType){
                    println!("wall & projectile");
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                }
            }
        }
    }



    fn checkEnemies(&mut self, waveSender: &Sender<bool>){
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
            let currentWave = *self.gameParameters.get("currentWave").unwrap();
            let finalWave = *self.gameParameters.get("finalWave").unwrap();

            if currentWave == finalWave {
                self.gameState = GameStateEnum::won;
            }
            else {
                waveSender.send(true);
                self.gameParameters.insert("currentWave", currentWave + 1.0).unwrap();
            }
            
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
                    println!("hp on id={} after taking damage: {}", idComp.getId(), healthComp.getHealth());
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
            }
        }

        if let EventEnum::removeObjectEvent{id, colliderHandle} = event {
            self.entityManager.removeObject(id);
            self.physicsManager.removeRigidBodyWithCollider(colliderHandle.0);
            //self.window.remove_node(sceneNode from renderableComponent);
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
        let iter = zip.filter_map(|(renderComp, (rigidComp, 
                                                                            (typeComp, moveComp))),
                                                                            |Some((renderComp.as_mut()?, rigidComp.as_mut()?, typeComp.as_mut()?, moveComp.as_mut()?)));
        /* Loop through all objects and if it's an enemy then move it */
        for (renderComp, rigidComp, typeComp, moveComp) in iter {
            if matches!(typeComp.getType(), TypeEnum::enemyType){
                let node = renderComp.getSceneNode();
                let rigidBody = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).unwrap();

                // Retrieves the rigidBody coordinates
                let t = GameManager::moveEnemy(rigidBody, moveComp);
                let r = rigidBody.rotation().clone();

                let axis;
                match r.axis() {
                    Some(n) => axis = n[1],
                    None => axis = 0.0, // why the hell would you return a None if the axis is zero ????????
                }

                let mut axisangle = Vector3::y();
                axisangle[1] = axisangle[1]* axis * r.angle();
                let r = UnitQuaternion::new(axisangle);

                //let t = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).translation();
                // Sets the renderableComponent node coordinates to the rigidBody coordinates
                node.write().unwrap().set_local_translation(Translation3::new(t.0, t.1, t.2));
                node.write().unwrap().set_local_rotation(r);
            }
            if matches!(typeComp.getType(), TypeEnum::projectileType){
                let a = rigidComp.getRigidBodyHandle();
                println!("{}", rigidComp.getRigidBodyHandle().0.into_raw_parts().0);
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
        let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
        // If the enemy is located near enough the next point, then remove it and use the new next point in the sequence
        if  (nextPoint.0 - 0.3) < t.0 && t.0 < (nextPoint.0 + 0.3) && (nextPoint.1 - 0.3) < t.2 && t.2 < (nextPoint.1 + 0.3){
            nextPoint = moveComp.popAndGetNextPoint();
        }

        let speed = moveComp.getSpeed() as f32;
        let pythagoras = ((nextPoint.0 - t.0).powf(2.0)+(nextPoint.1 - t.2).powf(2.0)).sqrt();

        let xVelocity = (nextPoint.0 - t.0) * speed / pythagoras;
        let yVelocity = 0.0; //(yTarget-yOrigin) * speed / pythagoras;
        let zVelocity = (nextPoint.1 - t.2) * speed / pythagoras;

        rigidBody.set_linvel(vector![xVelocity,yVelocity,zVelocity], true);

        let theta = ((xVelocity).atan2(zVelocity))  * 1.0;
        let axisangle = AngVector::new(0.0,theta,0.0);

        rigidBody.set_rotation(axisangle, true);
        return (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
    }


    fn createBase(&mut self){
        let endCoords = self.mapManager.getEnd();

        let base = self.entityManager.newObject();
        self.entityManager.addComponentToObject(base, TypeComponent::new(TypeEnum::baseType));
        self.entityManager.addComponentToObject(base, HealthComponent::new(*self.gameParameters.get("baseHealth").unwrap() as usize));
        // Remove these plz
        self.entityManager.addComponentToObject(base, MoveComponent::new(0.0));
        self.entityManager.addComponentToObject(base, AttackDamageComponent::new(0.0));

        self.createRenderComponent(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, 0.007);
        self.createRigidBodyAndColliderComponents(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, (0.0, 0.0, 0.0), 1.0);

    }


    fn spawnProjectile(&mut self, xTarget:  f32, yTarget: f32, zTarget: f32, xOrigin: f32, yOrigin: f32, zOrigin: f32){
        //TODO: Add ability to move
        let projectile = self.entityManager.newObject();
        self.entityManager.addComponentToObject(projectile, TypeComponent::new(TypeEnum::projectileType));
        self.entityManager.addComponentToObject(projectile, AttackDamageComponent::new(*self.gameParameters.get("towerAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(projectile, MoveComponent::new(*self.gameParameters.get("projectileSpeed").unwrap()));

        let speed = *self.gameParameters.get("projectileSpeed").unwrap();
        
        let pythagoras = ((xTarget-xOrigin).powf(2.0)+(yTarget-yOrigin).powf(2.0)+(zTarget-zOrigin).powf(2.0)).sqrt();
        let xVelocity = (xTarget-xOrigin) * speed / pythagoras;
        let yVelocity = (yTarget-yOrigin) * speed / pythagoras;
        let zVelocity = (zTarget-zOrigin) * speed / pythagoras;

        self.createRenderComponent(projectile, TypeEnum::projectileType,  xOrigin, yOrigin, zOrigin, 0.06);
        self.createRigidBodyAndColliderComponents(projectile, TypeEnum::projectileType, 
                     xOrigin, yOrigin, zOrigin, (xVelocity, yVelocity, zVelocity), 0.1);
    }


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(*self.gameParameters.get("towerAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(*self.gameParameters.get("towerAttackRate").unwrap()));    // Do we even use this??

        self.createRenderComponent(tower, TypeEnum::towerType, x, y, z, 0.6);
    }



    fn spawnEnemy(&mut self){
        let enemy = self.entityManager.newObject();
        self.entityManager.addComponentToObject(enemy, TypeComponent::new(TypeEnum::enemyType));
        self.entityManager.addComponentToObject(enemy, AttackDamageComponent::new(*self.gameParameters.get("enemyAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(enemy, HealthComponent::new(*self.gameParameters.get("enemyHealth").unwrap() as usize));
        self.entityManager.addComponentToObject(enemy, MoveComponent::newWithPath(*self.gameParameters.get("enemySpeed").unwrap(), self.mapManager.findPath().unwrap()));
        
        let startCoords = self.mapManager.getStart();
        self.createRenderComponent(enemy, TypeEnum::enemyType, startCoords.0, *self.gameParameters.get("enemyHeight").unwrap(), startCoords.1, 0.5);
        self.createRigidBodyAndColliderComponents(enemy, TypeEnum::enemyType,
                     startCoords.0, *self.gameParameters.get("enemyHeight").unwrap(), startCoords.1, (0.0, 0.0, 0.0), 0.5);
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


    fn createRigidBodyAndColliderComponents(&mut self, id: usize, objectType: TypeEnum, x: f32, y: f32, z: f32, velocity: (f32, f32, f32), radius: f32){
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
        let collider = ColliderBuilder::new(ColliderShape::ball(0.5));
        let collider = collider.translation(vector![x, y, z]).sensor(true).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        self.entityManager.addComponentToObject(id, ColliderComponent::new(colliderHandle));
    }

    fn createWalls(&mut self){
        let map = self.mapManager.getMapMatrix();
        let width = map.ncols() as f32;
        let height = map.nrows() as f32;

        // let mut cube = self.window.add_cube(width as f32+4.0, 5.0, 0.1);
        // cube.set_local_translation(Translation3::new(width/2.0 -0.5, 2.5, -2.0));
        // let mut cube = self.window.add_cube(width as f32+4.0, 5.0, 0.1);
        // cube.set_local_translation(Translation3::new(width/2.0-0.5, 2.5, height+2.0));
        // let mut cube = self.window.add_cube(0.1, 5.0, height+3.8);
        // cube.set_local_translation(Translation3::new(-2.5, 2.5, height/2.0));
        // let mut cube = self.window.add_cube(0.1, 5.0, height+3.8);
        // cube.set_local_translation(Translation3::new(width+2.0-0.5, 2.5, height/2.0));

        let wall1 = self.entityManager.newObject();
        let wall2 = self.entityManager.newObject();
        let wall3 = self.entityManager.newObject();
        let wall4 = self.entityManager.newObject();
        self.entityManager.addComponentToObject(wall1, TypeComponent::new(TypeEnum::wallType));
        self.entityManager.addComponentToObject(wall2, TypeComponent::new(TypeEnum::wallType));
        self.entityManager.addComponentToObject(wall3, TypeComponent::new(TypeEnum::wallType));
        self.entityManager.addComponentToObject(wall4, TypeComponent::new(TypeEnum::wallType));
        // I want to remove this but then wall collision events are impossible......
        self.entityManager.addComponentToObject(wall1, AttackDamageComponent::new(0.0));
        self.entityManager.addComponentToObject(wall2, AttackDamageComponent::new(0.0));
        self.entityManager.addComponentToObject(wall3, AttackDamageComponent::new(0.0));
        self.entityManager.addComponentToObject(wall4, AttackDamageComponent::new(0.0));

        // The south wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(width as f32+4.0, 5.0, 0.1));
        let collider = collider.translation(vector![width/2.0-0.5, 2.5, -2.0]).sensor(true).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall1, ColliderComponent::new(colliderHandle));

        // The north wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(width as f32+4.0, 5.0, 0.1));
        let collider = collider.translation(vector![width/2.0-0.5, 2.5, height+2.0]).sensor(true).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall2, ColliderComponent::new(colliderHandle));

        // The east wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(0.1, 5.0, height+3.8));
        let collider = collider.translation(vector![-2.5, 2.5, height/2.0]).sensor(true).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall3, ColliderComponent::new(colliderHandle));

        // The west wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(0.1, 5.0, height+3.8));
        let collider = collider.translation(vector![width+1.5, 2.5, height/2.0]).sensor(true).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall4, ColliderComponent::new(colliderHandle));
    }

}


pub fn test(){

    let mut gm = GameManager::new();
    gm.initialize();

    //gm.spawnEnemy();
    //gm.spawnTower(2.0, 0.3, 4.0);
    //gm.spawnProjectile(5.0, 5.0, 5.0, 3.0, 1.0, 10.0);

    gm.gameloop();

    // gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z: 2});
    // gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    
    // gm.doEvent();
    // gm.doEvent();
}