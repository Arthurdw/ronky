// TODO: slice type
// TODO: option type
// TODO: tuple type
// TODO: enum type

#[cfg(test)]
mod tests {
    use ronky_exported::Exported;
    use ronky_proc::Export;
    use serde_json::json;

    #[test]
    fn test_basic_struct() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            name: String,
            age: u32,
        }
        let export = Human::export();
        assert_eq!(
            export,
            json!({
                "name": "Human",
                "fields": [
                    {
                        "name": "name",
                        "type": "String"
                    },
                    {
                        "name": "age",
                        "type": "u32"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_arrays() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            hobbies: Vec<String>,
        }

        let export = Human::export();

        assert_eq!(
            export,
            json!({
                "name": "Human",
                "fields": [
                    {
                        "name": "hobbies",
                        "type": "list",
                        "of": ["String"]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_nesting() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            pets: Vec<Pet>,
        }

        #[allow(dead_code)]
        struct Pet();

        let export = Human::export();

        assert_eq!(
            export,
            json!({
                "name": "Human",
                "fields": [
                    {
                        "name": "pets",
                        "type": "list",
                        "of": ["Pet"]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_recursion() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            friends: Vec<Human>,
        }

        let export = Human::export();

        assert_eq!(
            export,
            json!({
                "name": "Human",
                "fields": [
                    {
                        "name": "friends",
                        "type": "list",
                        "of": ["Human"]
                    }
                ]
            })
        )
    }

    #[test]
    fn test_generics() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            friends: Friend<Super, Bestie>,
        }
        struct Bestie();
        struct Super();
        struct Friend<T, U> {
            marker1: std::marker::PhantomData<T>,
            marker2: std::marker::PhantomData<U>,
        }

        let export = Human::export();

        assert_eq!(
            export,
            json!({
                "name": "Human",
                "fields": [
                    {
                        "name": "friends",
                        "type": "Friend",
                        "generics": ["Super", "Bestie"]
                    }
                ]
            })
        )
    }
}
