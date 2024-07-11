#[cfg(test)]
mod tests {
    use exported::Exported;
    use proc::Export;
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
                "types": [
                    {
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
                "types": [
                    {
                        "name": "Test",
                        "fields": [
                            {
                                "name": "hobbies",
                                "type": ["string"]
                            }
                        ]
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
        #[derive(Export)]
        struct Pet {
            name: String,
            species: String,
        }

        let export = Human::export();

        assert_eq!(
            export,
            json!({
                "types": [
                    {
                        "name": "Human",
                        "fields": [
                            {
                                "name": "pets",
                                "type": ["Pet"]
                            }
                        ]
                    },
                    {
                        "name": "Pet",
                        "fields": [
                            {
                                "name": "name",
                                "type": "string"
                            },
                            {
                                "name": "species",
                                "type": "string"
                            }
                        ]
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
                "types": [
                    {
                        "name": "Test",
                        "fields": [
                            {
                                "name": "friends",
                                "type": ["Test"]
                            }
                        ]
                    }
                ]
            })
        )
    }
}
