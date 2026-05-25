use std::{f32, ops::RangeInclusive};

use egui::{Color32, PopupAnchor, Pos2, Shape, Stroke, epaint::CircleShape};
use egui_plot::{
    Cursor, LabelFormatter, PlotBounds, PlotGeometry, PlotItem, PlotItemBase,
    PlotPoint, PlotTransform,
};

use crate::{
    app::{
        constants::{get_purity_color, get_resource_color},
        view_options::{ViewOptions, ViewOptionsTarget},
    },
    game::{FrackingCore, GeyserNode, ResourceDescriptor, ResourceNode},
};

pub enum ResourceDisplayContent<'a> {
    ResourceNodes(ResourceDescriptor, Vec<&'a ResourceNode>),
    FrackingNodes(ResourceDescriptor, Vec<&'a FrackingCore>),
    Geysers(Vec<&'a GeyserNode>),
}

impl<'a> ResourceDisplayContent<'a> {
    pub fn get_color(&self) -> Color32 {
        match self {
            Self::ResourceNodes(resource, _) | Self::FrackingNodes(resource, _) => {
                get_resource_color(*resource)
            }
            Self::Geysers(_) => get_resource_color(ResourceDescriptor::Water),
        }
    }

    fn convert_location(location: [f32; 3]) -> PlotPoint {
        PlotPoint::new(location[0] as f64, location[1] as f64)
    }

    pub fn get_points(&self) -> Vec<PlotPoint> {
        match self {
            Self::ResourceNodes(_, nodes) => nodes
                .iter()
                .map(|n| Self::convert_location(n.location))
                .collect(),

            Self::FrackingNodes(_, cores) => cores
                .iter()
                .flat_map(|c| {
                    let mut points = Vec::with_capacity(1 + c.satellites.len());
                    points.push(Self::convert_location(c.location));

                    for s in &c.satellites {
                        points.push(Self::convert_location(s.location));
                    }

                    points
                })
                .collect(),

            Self::Geysers(geysers) => geysers
                .iter()
                .map(|g| Self::convert_location(g.location))
                .collect(),
        }
    }
}

pub struct ResourceDisplay<'a> {
    base: PlotItemBase,
    geometry_points: Vec<PlotPoint>,

    marker_base_size: f32,
    content: ResourceDisplayContent<'a>,

    view_options: &'a ViewOptions,
    view_options_highlight: Option<ViewOptionsTarget>,
    plot_highlight: bool,
    icon: Option<egui::TextureId>,
}

impl<'a> ResourceDisplay<'a> {
    pub fn new(
        marker_base_size: f32,
        content: ResourceDisplayContent<'a>,
        view_options: &'a ViewOptions,
        highlight: Option<ViewOptionsTarget>,
        icon: Option<egui::TextureId>,
    ) -> Self {
        let name = match content {
            ResourceDisplayContent::ResourceNodes(resource, _)
            | ResourceDisplayContent::FrackingNodes(resource, _) => resource.to_string(),
            ResourceDisplayContent::Geysers(_) => "Geyser".to_owned(),
        };

        Self {
            base: PlotItemBase::new(name),
            geometry_points: content.get_points(),

            marker_base_size,
            content,

            view_options,
            view_options_highlight: highlight,
            plot_highlight: false,
            icon,
        }
    }

    fn marker(
        center: Pos2,
        radius: f32,
        fill_color: Color32,
        stroke_color: Color32,
        icon: Option<egui::TextureId>,
        shapes: &mut Vec<Shape>
    ) {
        shapes.push(Shape::Circle(CircleShape {
            center,
            radius,
            fill: fill_color,
            stroke: Stroke::new(radius / 10.0, stroke_color),
        }));

        if let Some(icon) = icon {
            let icon_size = radius * 1.5;
            shapes.push(egui::Shape::image(
                icon,
                egui::Rect::from_center_size(
                    center,
                    egui::vec2(icon_size, icon_size),
                ),
                egui::Rect::from_min_max(
                    egui::pos2(0.0, 0.0),
                    egui::pos2(1.0, 1.0),
                ),
                egui::Color32::WHITE,
            ))
        }
    }
}

impl<'a> PlotItem for ResourceDisplay<'a> {
    fn shapes(&self, _ui: &egui::Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        const HIGHLIGHT_SCALE: f32 = f32::consts::SQRT_2;

        let scale = if self.plot_highlight && self.view_options_highlight.is_none() {
            HIGHLIGHT_SCALE
        } else {
            1.0
        };
        let color = self.color();

        match &self.content {
            ResourceDisplayContent::ResourceNodes(resource, nodes) => {
                for node in nodes {
                    let target = ViewOptionsTarget::ResourceWithPurity(*resource, node.purity);
                    if !self
                        .view_options
                        .is_target_visible(target)
                        .unwrap_or_default()
                    {
                        continue;
                    }

                    let scale = if self
                        .view_options_highlight
                        .is_some_and(|h| h.contains(&target))
                    {
                        HIGHLIGHT_SCALE
                    } else {
                        scale
                    };

                    let center = transform.position_from_point(
                        &ResourceDisplayContent::convert_location(node.location),
                    );
                    Self::marker(
                        center,
                        self.marker_base_size * scale,
                        get_purity_color(node.purity),
                        get_resource_color(*resource),
                        self.icon,
                        shapes,
                    );
                }
            }

            ResourceDisplayContent::FrackingNodes(resource, cores) => {
                let target = ViewOptionsTarget::ResourceFrackingNodes(*resource);
                if !self
                    .view_options
                    .is_target_visible(target)
                    .unwrap_or_default()
                {
                    return;
                }

                let scale = if self
                    .view_options_highlight
                    .is_some_and(|h| h.contains(&target))
                {
                    HIGHLIGHT_SCALE
                } else {
                    scale
                };

                for core in cores {
                    for satellite in &core.satellites {
                        let center = transform.position_from_point(
                            &ResourceDisplayContent::convert_location(satellite.location),
                        );

                        Self::marker(
                            center,
                            0.75 * self.marker_base_size * scale,
                            get_purity_color(satellite.purity),
                            get_resource_color(*resource),
                            self.icon,
                            shapes,
                        );
                    }
                }
            }

            ResourceDisplayContent::Geysers(geysers) => {
                let target = ViewOptionsTarget::Geysers;
                if !self
                    .view_options
                    .is_target_visible(target)
                    .unwrap_or_default()
                {
                    return;
                }

                let scale = if self
                    .view_options_highlight
                    .is_some_and(|h| h.contains(&target))
                {
                    HIGHLIGHT_SCALE
                } else {
                    scale
                };

                for geyser in geysers {
                    let center = transform.position_from_point(
                        &ResourceDisplayContent::convert_location(geyser.location),
                    );

                    Self::marker(
                        center,
                        self.marker_base_size * scale,
                        get_purity_color(geyser.purity),
                        color,
                        self.icon,
                        shapes,
                    );
                }
            }
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.content.get_color()
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(&self.geometry_points)
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        for p in &self.geometry_points {
            bounds.extend_with(p);
        }

        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }

    fn on_hover(
        &self,
        plot_area_response: &egui::Response,
        elem: egui_plot::ClosestElem,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<egui_plot::Cursor>,
        plot: &egui_plot::PlotConfig<'_>,
        _label_formatter: &Option<LabelFormatter<'_>>,
    ) {
        let line_color = if plot.ui.visuals().dark_mode {
            Color32::from_gray(100).additive()
        } else {
            Color32::from_black_alpha(180)
        };

        let value = self.geometry_points[elem.index];
        let pointer = plot.transform.position_from_point(&value);
        shapes.push(Shape::circle_filled(pointer, 3.0, line_color));

        cursors.push(Cursor::Vertical { x: value.x });
        cursors.push(Cursor::Horizontal { y: value.y });

        let mut tooltip = egui::Tooltip::always_open(
            plot_area_response.ctx.clone(),
            plot_area_response.layer_id,
            plot_area_response.id,
            PopupAnchor::Pointer,
        );

        let tooltip_width = plot_area_response.ctx.global_style().spacing.tooltip_width;

        tooltip.popup = tooltip.popup.width(tooltip_width);

        tooltip.gap(12.0).show(|ui| {
            ui.set_max_width(tooltip_width);

            let location = match &self.content {
                ResourceDisplayContent::ResourceNodes(_, nodes) => {
                    let node = nodes[elem.index];

                    ui.label(format!("{} ({:?})", node.resource, node.purity));
                    node.location
                }
                ResourceDisplayContent::FrackingNodes(_, cores) => {
                    let mut index = elem.index;

                    let mut location = [0f32; 3];
                    for core in cores {
                        if index == 0 {
                            ui.label(format!("{} (Resource Well)", core.resource));

                            location = core.location;
                            break;
                        }

                        index -= 1;

                        if index < core.satellites.len() {
                            let satellite = &core.satellites[index];
                            ui.label(format!(
                                "{} ({:?} Resource Well)",
                                core.resource, satellite.purity
                            ));

                            location = satellite.location;
                            break;
                        }

                        index -= core.satellites.len();
                    }

                    location
                }
                ResourceDisplayContent::Geysers(geysers) => {
                    let geyser = geysers[elem.index];

                    ui.label(format!("Geyser ({:?})", geyser.purity));
                    geyser.location
                }
            };

            ui.label(format!(
                "x = {:.1}\ny = {:.1}\nz = {:.1}",
                location[0], location[1], location[2],
            ));
        });
    }

    fn highlight(&mut self) {
        self.plot_highlight = true
    }

    fn highlighted(&self) -> bool {
        if self.plot_highlight {
            return true;
        }

        let Some(view_options_highlight) = self.view_options_highlight else {
            return false;
        };

        match view_options_highlight {
            ViewOptionsTarget::Geysers => {
                matches!(self.content, ResourceDisplayContent::Geysers(_))
            }
            ViewOptionsTarget::Resource(resource) => {
                matches!(
                    self.content,
                    ResourceDisplayContent::ResourceNodes(r, _)
                    | ResourceDisplayContent::FrackingNodes(r, _)
                        if r == resource
                )
            }
            ViewOptionsTarget::ResourceWithPurity(resource, _) => {
                matches!(self.content, ResourceDisplayContent::ResourceNodes(r, _) if r == resource)
            }
            ViewOptionsTarget::ResourceFrackingNodes(resource) => {
                matches!(self.content, ResourceDisplayContent::FrackingNodes(r, _) if r == resource)
            }
            _ => false,
        }
    }
}
