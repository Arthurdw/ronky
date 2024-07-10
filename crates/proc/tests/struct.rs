#[cfg(test)]
mod tests {
    use exported::Exported;
    use proc::Export;
    use serde_json::json;

    #[test]
    fn test_basic_export() {
        #[allow(dead_code)]
        #[derive(Export)]
        struct Human {
            name: String,
            age: u8,
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
                                "name": "name",
                                "type": "string"
                            },
                            {
                                "name": "age",
                                "type": "u8"
                            },
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
    fn test_basic_nesting() {
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
    fn test_basic_recursion() {
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
