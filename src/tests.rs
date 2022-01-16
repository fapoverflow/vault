#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_actors() -> Result<(), Box<dyn Error>> {
        // cargo test parse_actors -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Actor>(
                &Path::new(&env::args().nth(3).unwrap()).join("actors.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_actorrefs() -> Result<(), Box<dyn Error>> {
        // cargo test parse_actorrefs -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::ActorReference>(
                &Path::new(&env::args().nth(3).unwrap()).join("actor_references.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_images() -> Result<(), Box<dyn Error>> {
        // cargo test parse_images -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Image>(
                &Path::new(&env::args().nth(3).unwrap()).join("images.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_labels() -> Result<(), Box<dyn Error>> {
        // cargo test parse_labels -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Label>(
                &Path::new(&env::args().nth(3).unwrap()).join("labels.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_markers() -> Result<(), Box<dyn Error>> {
        // cargo test parse_markers -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Marker>(
                &Path::new(&env::args().nth(3).unwrap()).join("markers.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_labelleditems() -> Result<(), Box<dyn Error>> {
        // cargo test parse_labelleditems -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::LabelledItem>(
                &Path::new(&env::args().nth(3).unwrap()).join("labelled_items.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_scenes() -> Result<(), Box<dyn Error>> {
        // cargo test parse_scenes -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Scene>(
                &Path::new(&env::args().nth(3).unwrap()).join("scenes.db")
            )?
        );
        Ok(())
    }

    #[test]
    fn parse_studios() -> Result<(), Box<dyn Error>> {
        // cargo test parse_studios -- --nocapture /tmp/library
        println!(
            "{:?}",
            utils::vec_type_or_deleted_from_file::<models::Studio>(
                &Path::new(&env::args().nth(3).unwrap()).join("studios.db")
            )?
        );
        Ok(())
    }
}
