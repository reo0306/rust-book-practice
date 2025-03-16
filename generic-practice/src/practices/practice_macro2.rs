macro_rules! create_struct {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),*) => {
        struct $struct_name {
            $($field_name: $field_type),*
        }

        impl $struct_name {
            pub fn new($($field_name: $field_type), *) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
    };
}

pub struct PracticeMacro2;

impl PracticeMacro2 {
    pub fn exec() {
        create_struct!(User, id: u32, name: String);
        let user = User::new(1, "Alice".to_string());
        println!("User ID: {}, Name: {}", user.id, user.name);
    }
}