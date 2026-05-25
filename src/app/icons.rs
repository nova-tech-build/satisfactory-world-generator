use std::collections::HashMap;

use crate::game::ResourceDescriptor;

pub struct IconSet {
    resources: HashMap<ResourceDescriptor, egui::TextureHandle>,
}

impl IconSet {
    pub fn load(ctx: &egui::Context) -> Self {
        let resources = [
            (ResourceDescriptor::OreIron, include_bytes!("../../assets/icons/Desc_OreIron_C.png").as_slice()),
            (ResourceDescriptor::Coal, include_bytes!("../../assets/icons/Desc_Coal_C.png").as_slice()),
            (ResourceDescriptor::OreCopper, include_bytes!("../../assets/icons/Desc_OreCopper_C.png").as_slice()),
            (ResourceDescriptor::Stone, include_bytes!("../../assets/icons/Desc_Stone_C.png").as_slice()),
            (ResourceDescriptor::RawQuartz, include_bytes!("../../assets/icons/Desc_RawQuartz_C.png").as_slice()),
            (ResourceDescriptor::LiquidOil, include_bytes!("../../assets/icons/Desc_LiquidOil_C.png").as_slice()),
            (ResourceDescriptor::Water, include_bytes!("../../assets/icons/Desc_Water_C.png").as_slice()),
            (ResourceDescriptor::SAM, include_bytes!("../../assets/icons/Desc_SAM_C.png").as_slice()),
            (ResourceDescriptor::NitrogenGas, include_bytes!("../../assets/icons/Desc_NitrogenGas_C.png").as_slice()),
            (ResourceDescriptor::OreBauxite, include_bytes!("../../assets/icons/Desc_OreBauxite_C.png").as_slice()),
            (ResourceDescriptor::OreGold, include_bytes!("../../assets/icons/Desc_OreGold_C.png").as_slice()),
            (ResourceDescriptor::Sulfur, include_bytes!("../../assets/icons/Desc_Sulfur_C.png").as_slice()),
            (ResourceDescriptor::OreUranium, include_bytes!("../../assets/icons/Desc_OreUranium_C.png").as_slice()),
        ]
        .into_iter()
        .filter_map(|(resource, bytes)| {
            load_texture(ctx, resource.get_internal_name(), bytes).map(|texture| (resource, texture))
        })
        .collect();

        Self { resources }
    }

    pub fn resource(&self, resource: ResourceDescriptor) -> Option<&egui::TextureHandle> {
        self.resources.get(&resource)
    }
}

impl Default for IconSet {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}

fn load_texture(
    ctx: &egui::Context,
    name: &str,
    bytes: &[u8],
) -> Option<egui::TextureHandle> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    Some(ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        Default::default(),
    ))
}