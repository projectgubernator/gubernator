use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest<Spec> {
    pub kind: String,
    pub spec: Spec,
}

impl<Spec> Manifest<Spec>
where
    Spec: DeserializeOwned,
{
    /// Attempts to load a Gubernator manifest from a path.
    ///
    /// This constructor expects the file associated with `path` to be a YAML-formatted document
    /// containing a valid manifest. If that is not the case, the constructor will return a
    /// failure result, with an error describing the problem it ran into.
    ///
    /// ## Examples
    ///
    /// ### Loading a simple service manifest
    ///
    /// This example loads a simple service manifest located at `examples/echo.yaml`. Once the
    /// manifest has been loaded successfully, we assert on the information it contains.
    ///
    /// ```
    /// use gubernator_manifest::Manifest;
    /// use gubernator_manifest::common::Service;
    ///
    /// let manifest = Manifest::<Service>::from_path_yaml("examples/echo.yaml")
    ///     .expect("failed to load service manifest");
    ///
    /// assert_eq!(manifest.kind, "service");
    /// assert_eq!(manifest.spec.name, "echo");
    /// assert_eq!(manifest.spec.containers.len(), 1);
    ///
    /// let container = manifest
    ///     .spec
    ///     .containers
    ///     .get(0)
    ///     .expect("container at index 0 should be present");
    ///
    /// assert_eq!(container.image, "hashicorp/http-echo");
    /// ```
    pub fn from_path_yaml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_raw = std::fs::read_to_string(path)?;
        let manifest = serde_yaml::from_str::<Manifest<Spec>>(manifest_raw.as_str())?;

        Ok(manifest)
    }
}
