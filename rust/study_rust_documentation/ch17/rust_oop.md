# Rust? OOP??

## 1. Encapsulation of Rust
> Encapsulation can be implemented with Rust

## 2. Inheritance of Rust
> Inheritance Cannot be implemented with Rust

- Rust's Recycle  
    Use Trait!!
- Rust's Polymorphism  
    Bounded parametic polymorphism

    ## But! Inheritance is not essential value!
        Sometimes, Inheritacne make code unflexible because child object always Inherit parent object's method although they don't need.

    ## How implement Polymorphism without Inheritance with Rust's Trait!


# Trait
- Trait Object cannot be added data. It's different feature with other's Object

## Trait Object require Object-safe
- If all methods declared in trait satisfy next two features, the trait has Object-safe
    - Method's return type is not 'Self'
    - Method doesn't have generic type parameter.  

    So, if trait object doesn't satisfy Object-safe occurs error.  
    #### example ( clone method returns 'Self' )
    ```
        pub struct Screen {
            pub components: Vec<Box<dyn Clone>>
        }
    ```
    ```
    error[E0038]: the trait `std::clone::Clone` cannot be made into an object
    --> src/lib.rs:2:5
    |
    2 |     pub components: Vec<Box<dyn Clone>>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` cannot be
    made into an object
    |
    = note: the trait cannot require that `Self : Sized`
    ```