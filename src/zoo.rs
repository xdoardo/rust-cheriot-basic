//! Examples of use of Rust's dynamic dispatch (in the context of FFI)

use crate::{println, rtos_utils::cheriot_random_byte};
use alloc::boxed::Box;
use core::ffi::c_void;

/// What is an animal?
trait Animal {
    /// An animal can speak.
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow!");
    }
}

/// Create a new animal, randomly.
#[unsafe(no_mangle)]
pub extern "C" fn animal_make() -> *mut c_void {
    //* Experiment 3.5:  use other crates */
    let mut seed: [u8; 32] = [0; 32];
    for i in 0..32 {
        unsafe {
            seed[i] = cheriot_random_byte();
        }
    }

    let mut rng = <rand::rngs::SmallRng as rand::SeedableRng>::from_seed(seed);

    if rand::Rng::random_bool(&mut rng, 0.5) {
        Box::into_raw(Box::new(Box::new(Dog) as Box<dyn Animal>)) as *mut Box<dyn Animal>
            as *mut c_void
    } else {
        Box::into_raw(Box::new(Box::new(Cat) as Box<dyn Animal>)) as *mut Box<dyn Animal>
            as *mut c_void
    }
}

/// Makes the animal speak.
#[unsafe(no_mangle)]
pub extern "C" fn animal_speak(animal: *mut c_void) {
    if animal.is_null() {
        return;
    }

    let animal = animal as *mut Box<dyn Animal>;
    let animal = unsafe { Box::from_raw(animal) };
    animal.speak();

    core::mem::forget(animal);
}

/// Frees the animal.
#[unsafe(no_mangle)]
pub extern "C" fn animal_destroy(animal: *mut c_void) {
    if animal.is_null() {
        return;
    }

    let animal = animal as *mut Box<dyn Animal>;
    let animal = unsafe { Box::from_raw(animal) };
    drop(animal);
}

#[unsafe(no_mangle)]
pub extern "C" fn zoo_tour() {
    fn speak(animal: &dyn Animal) {
        animal.speak();
    }

    println!("Making some animals, and testing basic dynamic dispatch:");

    let dog = Dog;
    dog.speak();
    speak(&dog);

    let cat = Cat;
    cat.speak();
    speak(&cat);
}
