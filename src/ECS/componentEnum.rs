use super::{healthComponent::HealthComponent, moveComponent::MoveComponent};


pub enum ComponentEnum{
    healthComponent(HealthComponent),
    moveComponent(MoveComponent),
}