extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::FirstPerson;
use kiss3d::text::Font;
use ::nalgebra::{Translation3, Vector3};
use rand::Rng;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType, ColliderShape, RigidBody, ActiveEvents, CollisionEvent, InteractionGroups};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};
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
use kiss3d::{window::Window, event::Action};
use kiss3d::light::Light;
use kiss3d::event::{Key};

use rapier3d::na::{self as nalgebra};
use rapier3d::prelude::AngVector;
use na::{vector, UnitQuaternion, Point2, Point3};

use std::thread::{self};
use std::time::{Duration, SystemTime};
use std::sync::{mpsc};


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
            ("towerAttackDamage", 5.0),     // Damage towers do per attack
            ("enemyAttackDamage", 1.0),     // Damage enemies do when reaching the base
            ("enemyHealth", 50.0),          // HP of enemies
            ("baseHealth", 1.0),            // HP of the base
            ("enemyHeight", 1.0),           // Y-coord for where the enemies spawn and fly
            ("towerHeight", 0.25),          // Y-coord for where the towers spawn
            ("enemySpeed", 3.0),            // How fast the enemies are
            ("projectileSpeed", 14.0),      // How fast the projectiles are
            ("towerAttackRate", 1000.0),    // Milliseconds between each tower attack
            ("finalWave", 20.0),            // Number of enemy waves
            ("currentWave", 0.0),           // Start wave, do not change please
            ("enemySpawnRate", 500.0),      // Milliseconds between enemy spawns within a wave
            ("printMS", 0.0),               // 0 = Don't print, 1 = Do print simulation time ms
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
        self.mapManager.parseMap("src/resources/map2.csv");
        self.mapManager.drawMap(&mut self.window);
        self.createBase();

        // Spawns the towers specified in the map matrix
        let towerCoordList = self.mapManager.getTowerLocations();
        for coords in towerCoordList{
            self.spawnTower(coords.0, *self.gameParameters.get("towerHeight").unwrap(), coords.1);
        }

        // Creates walls
        self.createWalls();

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.5, 0.7, 1.0);
    }


    pub fn gameloop(&mut self){

        let eye = Point3::new(92.0, 7.0, 104.0);
        let at = Point3::new(104.0, 1.0, 106.0);
        let mut first_person = FirstPerson::new(eye, at);

        let (spawnEnemySender, spawnEnemyReciever) = mpsc::channel();
        let (nextWaveSender, nextWaveReciever) = mpsc::channel();
        let (waveProgressSender, waveProgressReceiver) = mpsc::channel();
        let (enemySpawnRateSender, enemySpawnRateReceiver) = mpsc::channel();
        let (towerAttackRateSender, towerAttackRateReceiver) = mpsc::channel();

        enemySpawnRateSender.send(*self.gameParameters.get("enemySpawnRate").unwrap() as u64);
        towerAttackRateSender.send(*self.gameParameters.get("towerAttackRate").unwrap() as u64);

        // thread for towers to attack
        let (txTowerAttack, rxTowerAttack) = mpsc::channel();
        thread::spawn(move || {
            let towerAttackRate = towerAttackRateReceiver.recv().unwrap(); 

            loop {
                let val = String::from("kill enemies now please :)");
                txTowerAttack.send(val).unwrap();
                thread::sleep(Duration::from_millis(towerAttackRate));     // Set to same as towerAttackRate
            }
        });
        
        // thread for bird spawning
        thread::spawn(move || {
            let mut waveNumberEnemies = 2;
            let enemySpawnRate = enemySpawnRateReceiver.recv().unwrap(); 

            loop {
                // only spawn a wave when told to do so by nextWaveReciever
                // wait until we recieve a call do spawn a wave
                nextWaveReciever.recv().unwrap(); 

                // spawns waveNumberEnemies enemies with 2000 ms between each, 
                for _ in 0..waveNumberEnemies{
                    spawnEnemySender.send(true).unwrap();
                    thread::sleep(Duration::from_millis(enemySpawnRate));
                }

                waveProgressSender.send(true);

                // keep track of how many enemies to spawn and wavecounter
                waveNumberEnemies += 2;
            }
        });

        loop{
            let startTime = SystemTime::now();
            
            // Check TowerAttack thread for receives
            let receivedTowerAttack = rxTowerAttack.try_recv();
            match receivedTowerAttack {
                Ok(_) => self.checkEnemies(&nextWaveSender, &waveProgressReceiver),
                Err(_) => (),
            }

            // spawn an enemy whenever we recieve a spawnEnemy call from the bird spawining thread
            let doSpawnEnemy = spawnEnemyReciever.try_recv();
            match doSpawnEnemy {
                Ok(_) => self.spawnEnemy(),
                Err(_) => (),
            }

            self.physicsManager.step();
            
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

            while !self.eventManager.eventBufferIsEmpty(){
                self.doEvent();
            }

            if matches!(self.gameState, GameStateEnum::ongoing){
                self.updateNodes();
            }
            
            loop{
                // Do this a lot more than just once per frame
                let collision = self.physicsManager.getEvent();
                match collision{
                    Some(collision) => self.collisionEvent(collision),
                    None => break,
                }
            }
            
            if matches!(self.gameState, GameStateEnum::won){
                let font = Font::default();
                let pos = Point2::new(self.window.width()as f32/2.0 , self.window.height()as f32/2.0);
                self.window.draw_text("You win! :D", &pos, 250.0, &font, &Point3::new(0.6, 0.0, 0.6));
            }
            else if matches!(self.gameState, GameStateEnum::lost){
                let font = Font::default();
                let pos = Point2::new(self.window.width()as f32/2.0 , self.window.height()as f32/2.0);
                self.window.draw_text("You lose! :(", &pos, 250.0, &font, &Point3::new(0.0, 0.0, 0.4));
            }

            // Measure the simulation time per update (excluding rendering)
            if *self.gameParameters.get("printMS").unwrap() == 1.0{
                match startTime.elapsed() {
                    Ok(elapsed) => {
                        println!("{} ms", elapsed.as_millis());
                    }
                    Err(e) => {
                        // an error occurred!
                        println!("Error: {:?}", e);
                    }
                }
            }
            
            self.window.render_with_camera(&mut first_person);
        }
    }


    fn collisionEvent(&mut self, collision: CollisionEvent){
        if collision.stopped(){
            return;
        }
        let collider1 = collision.collider1();
        let collider2 = collision.collider2();
        let mut types: Vec<(TypeEnum, &mut IdComponent,&mut AttackDamageComponent, &mut ColliderComponent, &mut RenderableComponent)> = Vec::new();

        let mut colliderCompList = self.entityManager.borrowComponentVecMut::<ColliderComponent>().unwrap();
        let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let mut damageCompList = self.entityManager.borrowComponentVecMut::<AttackDamageComponent>().unwrap();
        let mut renderCompList = self.entityManager.borrowComponentVecMut::<RenderableComponent>().unwrap();
        
        let zip = colliderCompList.iter_mut().zip(idCompList.iter_mut().zip(typeCompList.iter_mut().zip(damageCompList.iter_mut().zip(renderCompList.iter_mut()))));
        let iter = zip.filter_map(|(colliderComp, (idComp, (typeComp, (damageComp, renderComp)))),
                                                                    |Some((colliderComp.as_mut()?, idComp.as_mut()?, typeComp.as_mut()?, damageComp.as_mut()?, renderComp.as_mut()?)));
            
        for (colliderComp, idComp, typeComp, damageComp, renderComp) in iter{
            if colliderComp.getColliderHandle().0 == collider1.0 || colliderComp.getColliderHandle().0 == collider2.0{ 
                types.push((typeComp.getType(), idComp, damageComp, colliderComp, renderComp));
                if types.len() == 2{
                    break;
                }
            }
        }

        let obj1 = types.pop();
        let obj2 = types.pop();

        
        if obj1.is_some() && obj2.is_some(){
            let obj1 = obj1.unwrap();
            let obj2 = obj2.unwrap();
           
            if matches!(obj1.0, TypeEnum::enemyType){
                if matches!(obj2.0, TypeEnum::projectileType){
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj1.1.getId(), damage: obj2.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                    self.window.remove_node(obj2.4.getSceneNode());
                }
                else if matches!(obj2.0, TypeEnum::baseType){
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj2.1.getId(), damage: obj1.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                    self.window.remove_node(obj1.4.getSceneNode());
                }
            }
    
            else if matches!(obj1.0, TypeEnum::projectileType){
                if matches!(obj2.0, TypeEnum::enemyType){
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj2.1.getId(), damage: obj1.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                    self.window.remove_node(obj1.4.getSceneNode());
                }
                else if matches!(obj2.0, TypeEnum::wallType){
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj1.1.getId(), colliderHandle: obj1.3.getColliderHandle()});
                    self.window.remove_node(obj1.4.getSceneNode());
                }
            }
    
            else if matches!(obj1.0, TypeEnum::baseType){
                if matches!(obj2.0, TypeEnum::enemyType){
                    self.eventManager.sendEvent(EventEnum::takeDamageEvent { id: obj1.1.getId(), damage: obj2.2.getAttackDamage() as usize});
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                    self.window.remove_node(obj2.4.getSceneNode());
                }
            }
    
            else if matches!(obj1.0, TypeEnum::wallType){
                if matches!(obj2.0, TypeEnum::projectileType){
                    self.eventManager.sendEvent(EventEnum::removeObjectEvent { id: obj2.1.getId(), colliderHandle: obj2.3.getColliderHandle()});
                    self.window.remove_node(obj2.4.getSceneNode());
                }
            }
        }
    }



    fn checkEnemies(&mut self, waveSender: &Sender<bool>, waveProgressReceiver: &Receiver<bool>){
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

        if !enemyOnMap && matches! (self.gameState, GameStateEnum::ongoing) {
            // Spawn new wave or won game
            // if no more waves:
            let currentWave = *self.gameParameters.get("currentWave").unwrap();
            let finalWave = *self.gameParameters.get("finalWave").unwrap();

            let waveInProgressChannel = waveProgressReceiver.try_recv();
            let startNextWave;
            match waveInProgressChannel {
                Ok(_) => startNextWave = false,
                Err(_) => startNextWave = true,
            }

            if currentWave >= finalWave && !startNextWave{
                self.gameState = GameStateEnum::won;
            } else if (startNextWave){
                waveSender.send(true);
                self.gameParameters.insert("currentWave", currentWave + 1.0).unwrap();
                println!("Starting wave {} out of {}", currentWave, finalWave);
            }            
        }
    }


    fn doEvent(&mut self){
        let event = self.eventManager.readEvent();

        if let EventEnum::takeDamageEvent{id, damage} = event {
            let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let mut colliderCompList = self.entityManager.borrowComponentVecMut::<ColliderComponent>().unwrap();
            let mut renderCompList = self.entityManager.borrowComponentVecMut::<RenderableComponent>().unwrap();
            let zip = healthCompList.iter_mut().zip(idCompList.iter_mut().zip(colliderCompList.iter_mut().zip(renderCompList.iter_mut())));
            let iter = zip.filter_map(|(healthComp, (idComp, 
                                                                            (colliderComp, renderComp))),
                                                                            |Some((healthComp.as_mut()?, idComp.as_mut()?, colliderComp.as_mut()?, renderComp.as_mut()?)));
            
            for (healthComp, idComp, colliderComp, renderComp) in iter {
                if idComp.getId() == id {
                    healthComp.decreaseHealth(damage);
                    if healthComp.getHealth() <= 0{
                        if id == 0{
                            self.gameState = GameStateEnum::lost;
                        }
                        else{
                            self.eventManager.sendEvent(EventEnum::removeObjectEvent { id:idComp.getId(), colliderHandle: colliderComp.getColliderHandle()});
                            self.window.remove_node(renderComp.getSceneNode());
                        }
                    }
                } 
            }
        }

        // Do attack with all object of type = towers
        // Do attack = create projectile object with heading towards x, y, z
        if let EventEnum::towerAttackEvent{xTarget, yTarget, zTarget} = event {
            let coordList = self.mapManager.getTowerLocations();
            for coords in coordList{
                self.spawnProjectile(xTarget, yTarget, zTarget, 
                                        coords.0, *self.gameParameters.get("towerHeight").unwrap(), coords.1);
            }
        }

        if let EventEnum::removeObjectEvent{id, colliderHandle} = event {
            let mut idExists = false;
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let iter = idCompList.iter_mut().filter_map(|idComp| Some(idComp.as_mut()?));
            for idComp in iter{
                if idComp.getId() == id{
                    idExists = true;
                }
            }
            drop(idCompList);
            if idExists == true{
                self.entityManager.removeObject(id);
                self.physicsManager.removeRigidBodyWithCollider(colliderHandle);
            }
        }
    }


    fn updateNodes(&mut self){
        let mut renderCompList = self.entityManager.borrowComponentVecMut::<RenderableComponent>().unwrap();
        let mut rigidCompList = self.entityManager.borrowComponentVecMut::<RigidBodyComponent>().unwrap();
        let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
        let mut moveCompList = self.entityManager.borrowComponentVecMut::<MoveComponent>().unwrap();
        let mut colliderCompList = self.entityManager.borrowComponentVecMut::<ColliderComponent>().unwrap();
        let zip = renderCompList.iter_mut().zip(rigidCompList.iter_mut().zip(typeCompList.iter_mut().zip(moveCompList.iter_mut().zip(colliderCompList.iter_mut()))));
        let iter = zip.filter_map(|(renderComp, (rigidComp, 
                                                                            (typeComp, (moveComp, colliderComp)))),
                                                                            |Some((renderComp.as_mut()?, rigidComp.as_mut()?, typeComp.as_mut()?, moveComp.as_mut()?, colliderComp.as_mut()?)));
        /* Loop through all objects and if it's an enemy then move it */
        for (renderComp, rigidComp, typeComp, moveComp, colliderComp) in iter {
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

                // Sets the renderableComponent node coordinates to the rigidBody coordinates
                node.set_local_translation(Translation3::new(t.0, t.1, t.2));
                node.set_local_rotation(r);
            }
            if matches!(typeComp.getType(), TypeEnum::projectileType){
                let node = renderComp.getSceneNode();

                let rigidBody = self.physicsManager.getRigidBody(rigidComp.getRigidBodyHandle()).unwrap();
                let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
                node.set_local_translation(Translation3::new(t.0, t.1, t.2));
                node.prepend_to_local_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.6));
                self.physicsManager.getCollider(colliderComp.getColliderHandle()).set_translation(vector![t.0, t.1, t.2])
            }
            

        }
    }


    fn moveEnemy(rigidBody: &mut RigidBody, moveComp: &mut MoveComponent) -> (f32, f32, f32){ 
        let mut nextPoint = moveComp.getNextPoint();
        let t = (rigidBody.translation()[0], rigidBody.translation()[1], rigidBody.translation()[2]);
        // If the enemy is located near enough the next point, then remove it and use the new next point in the sequence
        if  (nextPoint.0 - 0.3) < t.0 && t.0 < (nextPoint.0 + 0.3) && (nextPoint.1 - 0.3) < t.2 && t.2 < (nextPoint.1 + 0.3){
            match moveComp.popAndGetNextPoint() {
                Some(n) => nextPoint = n,
                None => (),
            }
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
        self.createRigidBodyAndColliderComponents(base, TypeEnum::baseType, endCoords.0, 0.2, endCoords.1, (0.0, 0.0, 0.0), ColliderShape::cuboid(0.2, 10.0, 0.2));
    }


    fn spawnProjectile(&mut self, xTarget:  f32, yTarget: f32, zTarget: f32, xOrigin: f32, yOrigin: f32, zOrigin: f32){
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
                     xOrigin, yOrigin, zOrigin, (xVelocity, yVelocity, zVelocity), ColliderShape::ball(0.2));
    }


    fn spawnTower(&mut self, x: f32, y: f32, z: f32){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType));
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(*self.gameParameters.get("towerAttackDamage").unwrap()));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(*self.gameParameters.get("towerAttackRate").unwrap()));    

        let scale = rand::thread_rng().gen_range(0.35..0.7);
        self.createRenderComponent(tower, TypeEnum::towerType, x, y, z, scale);
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
                     startCoords.0, *self.gameParameters.get("enemyHeight").unwrap(), startCoords.1, (0.0, 0.0, 0.0), ColliderShape::ball(0.3));
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


    fn createRigidBodyAndColliderComponents(&mut self, id: usize, objectType: TypeEnum, x: f32, y: f32, z: f32, velocity: (f32, f32, f32), colliderShape: ColliderShape){
        // Add RigidBody to PhysicsManager and RigidBodyHandle to RigidBodyComponent (like an index) with a translation 
        let bodyBuilder = RigidBodyBuilder::new(RigidBodyType::Dynamic);
        let mut body = bodyBuilder.translation(vector![x, y, z]).build();

        // Sets a velocity for projectiles
        if matches!(objectType, TypeEnum::projectileType){
            body.set_linvel(vector![velocity.0, velocity.1, velocity.2], true);
        }

        let rigidBodyHandle = self.physicsManager.addRigidBody(body);
        self.entityManager.addComponentToObject(id, RigidBodyComponent::new(rigidBodyHandle));

        // Add Collider to PhysicsManager and ColliderHandle to ColliderComponent (like an index) with a translation 
        let colliderBuilder = ColliderBuilder::new(colliderShape);
        let mut collider = colliderBuilder.sensor(true).active_events(ActiveEvents::COLLISION_EVENTS);

        if matches!(objectType, TypeEnum::projectileType){
            collider = collider.collision_groups(InteractionGroups::new(0b0010, 0b0001));
        }

        else if matches!(objectType, TypeEnum::enemyType){
            collider = collider.collision_groups(InteractionGroups::new(0b0001, 0b0010));
        }

        let collider = collider.build();
        let colliderHandle = self.physicsManager.addColliderWithParent(collider, rigidBodyHandle);
        
        self.entityManager.addComponentToObject(id, ColliderComponent::new(colliderHandle));  
    }

    fn createWalls(&mut self){
        let map = self.mapManager.getMapMatrix();
        let width = map.ncols() as f32;
        let height = map.nrows() as f32;
        let offset = 100.0;

        let wall1 = self.entityManager.newObject();
        let wall2 = self.entityManager.newObject();
        let wall3 = self.entityManager.newObject();
        let wall4 = self.entityManager.newObject();

        let mut cube = self.window.add_cube(width as f32+4.0, 5.0, 0.1);
        cube.set_local_translation(Translation3::new(width/2.0-0.5+offset, 2.5, offset-2.0));
        cube.set_visible(false);
        self.entityManager.addComponentToObject(wall1, RenderableComponent::new(cube));

        let mut cube = self.window.add_cube(width as f32+4.0, 5.0, 0.1);
        cube.set_local_translation(Translation3::new(width/2.0-0.5+offset, 2.5, height+2.0+offset));
        cube.set_visible(false);
        self.entityManager.addComponentToObject(wall2, RenderableComponent::new(cube));

        let mut cube = self.window.add_cube(0.1, 5.0, height+3.8);
        cube.set_local_translation(Translation3::new(offset-2.5, 2.5, height/2.0+offset));
        cube.set_visible(false);
        self.entityManager.addComponentToObject(wall3, RenderableComponent::new(cube));

        let mut cube = self.window.add_cube(0.1, 5.0, height+3.8);
        cube.set_local_translation(Translation3::new(width+2.0-0.5+offset, 2.5, height/2.0+offset));
        cube.set_visible(false);
        self.entityManager.addComponentToObject(wall4, RenderableComponent::new(cube));

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
        let collider = collider.translation(vector![width/2.0-0.5+offset, 2.5, offset-2.0]).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall1, ColliderComponent::new(colliderHandle));

        // The north wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(width as f32+4.0, 5.0, 0.1));
        let collider = collider.translation(vector![width/2.0-0.5+offset, 2.5, height+2.0+offset]).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall2, ColliderComponent::new(colliderHandle));

        // The east wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(0.1, 5.0, height+3.8));
        let collider = collider.translation(vector![offset-2.5, 2.5, height/2.0+offset]).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall3, ColliderComponent::new(colliderHandle));

        // The west wall
        let collider = ColliderBuilder::new(ColliderShape::cuboid(0.1, 5.0, height+3.8));
        let collider = collider.translation(vector![width+2.0-0.5+offset, 2.5, height/2.0+offset]).active_events(ActiveEvents::COLLISION_EVENTS).build();
        let colliderHandle = self.physicsManager.addCollider(collider);
        self.entityManager.addComponentToObject(wall4, ColliderComponent::new(colliderHandle));
    }

}


pub fn main(){
    let mut gm = GameManager::new();
    gm.initialize();
    gm.gameloop();
}