use anyhow::Result;
use serde::{Serialize, Serializer, ser::SerializeMap};
use turbo_tasks::{ResolvedVc, Vc};
use turbo_tasks_fs::{File, FileContent, FileSystemPath};
use turbopack_core::{
    asset::{Asset, AssetContent},
    output::{OutputAsset, OutputAssetsReference},
};

use crate::paths::{AssetPath, AssetPaths};

#[turbo_tasks::value]
pub struct AssetHashesManifestAsset {
    output_path: FileSystemPath,
    asset_paths: ResolvedVc<AssetPaths>,
}

#[turbo_tasks::value_impl]
impl AssetHashesManifestAsset {
    #[turbo_tasks::function]
    pub fn new(output_path: FileSystemPath, asset_paths: ResolvedVc<AssetPaths>) -> Vc<Self> {
        AssetHashesManifestAsset {
            output_path,
            asset_paths,
        }
        .cell()
    }
}

#[turbo_tasks::value_impl]
impl OutputAssetsReference for AssetHashesManifestAsset {}

#[turbo_tasks::value_impl]
impl OutputAsset for AssetHashesManifestAsset {
    #[turbo_tasks::function]
    async fn path(&self) -> Vc<FileSystemPath> {
        self.output_path.clone().cell()
    }
}

#[turbo_tasks::value_impl]
impl Asset for AssetHashesManifestAsset {
    #[turbo_tasks::function]
    async fn content(&self) -> Result<Vc<AssetContent>> {
        let files = self.asset_paths.await?;

        #[derive(Serialize)]
        struct Manifest<'a> {
            #[serde(serialize_with = "serialize_vec_as_map")]
            files: &'a Vec<AssetPath>,
        }

        let json = serde_json::to_string(&Manifest { files: &files })?;

        Ok(AssetContent::file(
            FileContent::Content(File::from(json)).cell(),
        ))
    }
}

fn serialize_vec_as_map<S>(list: &Vec<AssetPath>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(list.len()))?;
    for entry in list {
        map.serialize_entry(&entry.path, &entry.content_hash)?;
    }
    map.end()
}
