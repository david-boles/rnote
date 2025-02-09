use p2d::bounding_volume::{BoundingVolume, AABB};
use piet::RenderContext;

use crate::helpers::AABBHelpers;
use crate::penhelpers::{PenEvent, PenState};
use crate::penpath::Element;
use crate::shapes::Ellipse;
use crate::style::{drawhelpers, Composer};
use crate::{Shape, Style};

use super::shapebuilderbehaviour::{BuilderProgress, ShapeBuilderCreator};
use super::ShapeBuilderBehaviour;

#[derive(Debug, Clone)]
/// The state
pub enum FociEllipseBuilderState {
    /// first
    First(na::Vector2<f64>),
    /// foci
    Foci([na::Vector2<f64>; 2]),
    /// foci and point
    FociAndPoint {
        /// The foci
        foci: [na::Vector2<f64>; 2],
        /// the point
        point: na::Vector2<f64>,
    },
}

#[derive(Debug, Clone)]
/// building ellipse with foci and point
pub struct FociEllipseBuilder {
    /// the state
    pub state: FociEllipseBuilderState,
}

impl ShapeBuilderCreator for FociEllipseBuilder {
    fn start(element: Element) -> Self {
        Self {
            state: FociEllipseBuilderState::First(element.pos),
        }
    }
}

impl ShapeBuilderBehaviour for FociEllipseBuilder {
    fn handle_event(&mut self, event: PenEvent) -> BuilderProgress {
        //log::debug!("state: {:?}, event: {:?}", &self.state, &event);

        match (&mut self.state, event) {
            (FociEllipseBuilderState::First(first), PenEvent::Down { element, .. }) => {
                *first = element.pos;
            }
            (FociEllipseBuilderState::First(first), PenEvent::Up { element, .. }) => {
                self.state = FociEllipseBuilderState::Foci([*first, element.pos])
            }
            (FociEllipseBuilderState::First(_), _) => {}
            (FociEllipseBuilderState::Foci(foci), PenEvent::Down { element, .. }) => {
                foci[1] = element.pos;
            }
            (FociEllipseBuilderState::Foci(foci), PenEvent::Up { element, .. }) => {
                self.state = FociEllipseBuilderState::FociAndPoint {
                    foci: *foci,
                    point: element.pos,
                };
            }
            (FociEllipseBuilderState::Foci(_), _) => {}
            (
                FociEllipseBuilderState::FociAndPoint { foci: _, point },
                PenEvent::Down { element, .. },
            ) => {
                *point = element.pos;
            }
            (FociEllipseBuilderState::FociAndPoint { foci, point }, PenEvent::Up { .. }) => {
                let shape = Ellipse::from_foci_and_point(*foci, *point);

                return BuilderProgress::Finished(Some(vec![Shape::Ellipse(shape)]));
            }
            (FociEllipseBuilderState::FociAndPoint { .. }, _) => {}
        }

        BuilderProgress::InProgress
    }

    fn bounds(&self, style: &Style, zoom: f64) -> AABB {
        let stroke_width = style.stroke_width();

        match &self.state {
            FociEllipseBuilderState::First(point) => AABB::from_half_extents(
                na::Point2::from(*point),
                na::Vector2::repeat(stroke_width.max(drawhelpers::POS_INDICATOR_RADIUS) / zoom),
            ),
            FociEllipseBuilderState::Foci(foci) => {
                AABB::new_positive(na::Point2::from(foci[0]), na::Point2::from(foci[1]))
                    .loosened(stroke_width.max(drawhelpers::POS_INDICATOR_RADIUS) / zoom)
            }
            FociEllipseBuilderState::FociAndPoint { foci, point } => {
                let ellipse = Ellipse::from_foci_and_point(*foci, *point);
                ellipse
                    .composed_bounds(style)
                    .loosened(drawhelpers::POS_INDICATOR_RADIUS / zoom)
            }
        }
    }

    fn draw_styled(&self, cx: &mut piet_cairo::CairoRenderContext, style: &Style, zoom: f64) {
        cx.save().unwrap();
        match &self.state {
            FociEllipseBuilderState::First(point) => {
                drawhelpers::draw_pos_indicator(cx, PenState::Down, *point, zoom);
            }
            FociEllipseBuilderState::Foci(foci) => {
                drawhelpers::draw_pos_indicator(cx, PenState::Up, foci[0], zoom);
                drawhelpers::draw_pos_indicator(cx, PenState::Up, foci[1], zoom);
            }
            FociEllipseBuilderState::FociAndPoint { foci, point } => {
                let ellipse = Ellipse::from_foci_and_point(*foci, *point);

                ellipse.draw_composed(cx, style);

                drawhelpers::draw_vec_indicator(cx, PenState::Down, foci[0], *point, zoom);
                drawhelpers::draw_vec_indicator(cx, PenState::Down, foci[1], *point, zoom);
                drawhelpers::draw_pos_indicator(cx, PenState::Up, foci[0], zoom);
                drawhelpers::draw_pos_indicator(cx, PenState::Up, foci[1], zoom);
                drawhelpers::draw_pos_indicator(cx, PenState::Down, *point, zoom);
            }
        }
        cx.restore().unwrap();
    }
}
