Pour les uuid :
il faudra stocker avec id.as_bytes() au lieu de id.to_string(),
et reconstruire avec Uuid::from_slice(&bytes) à la lecture.