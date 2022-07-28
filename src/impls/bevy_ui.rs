use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_egui::egui;

use crate::Inspectable;

use super::{NumberAttributes, OptionAttributes};

impl_for_simple_enum!(Display: Flex, None);
impl_for_simple_enum!(bevy::ui::FocusPolicy: Block, Pass);
impl_for_simple_enum!(PositionType: Absolute, Relative);
impl_for_simple_enum!(Direction: Inherit, LeftToRight, RightToLeft);
impl_for_simple_enum!(FlexDirection: Row, Column, RowReverse, ColumnReverse);
impl_for_simple_enum!(FlexWrap: NoWrap, Wrap, WrapReverse);
impl_for_simple_enum!(AlignItems: FlexStart, FlexEnd, Center, Baseline, Stretch);
impl_for_simple_enum!(
    AlignSelf: Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch
);
impl_for_simple_enum!(
    AlignContent: FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround
);
impl_for_simple_enum!(
    JustifyContent: FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
);



impl Inspectable for Val {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        _: Self::Attributes,
        context: &mut crate::Context,
    ) -> bool {
        use std::mem::discriminant;

        let selected = match self {
            Val::Undefined => "Undefined",
            Val::Auto => "Auto",
            Val::Px(_) => "Px",
            Val::Percent(_) => "Percent",
        };

        enum WhatToShow {
            Px,
            Percent,
        }

        let mut what_to_show = match self {
            Val::Px(_) => Some(WhatToShow::Px),
            Val::Percent(_) => Some(WhatToShow::Percent),
            _ => None,
        };

        let mut changed = false;

        ui.columns(2, |ui| {
            egui::ComboBox::from_id_source(context.id())
                .selected_text(selected)
                .show_ui(&mut ui[0], |ui| {
                    if ui
                        .selectable_label(*self == Val::Undefined, "Undefined")
                        .clicked()
                    {
                        *self = Val::Undefined;
                        what_to_show = None;
                        changed = true;
                    }
                    if ui.selectable_label(*self == Val::Auto, "Auto").clicked() {
                        *self = Val::Auto;
                        what_to_show = None;
                        changed = true;
                    }
                    let is_px = discriminant(self) == discriminant(&Val::Px(0.0));
                    if ui.selectable_label(is_px, "Px").clicked() {
                        what_to_show = Some(WhatToShow::Px);
                        changed = true;
                    }
                    let is_pct = discriminant(self) == discriminant(&Val::Percent(0.0));
                    if ui.selectable_label(is_pct, "Percent").clicked() {
                        what_to_show = Some(WhatToShow::Percent);
                        changed = true;
                    }
                });

            match what_to_show {
                Some(WhatToShow::Px) => {
                    let mut value = match self {
                        Val::Px(val) => *val,
                        _ => 0.0,
                    };

                    let attrs = NumberAttributes {
                        speed: 10.0,
                        ..Default::default()
                    };
                    changed |= value.ui(&mut ui[1], attrs, context);
                    *self = Val::Px(value);
                }
                Some(WhatToShow::Percent) => {
                    let mut value = match self {
                        Val::Percent(val) => *val,
                        _ => 0.0,
                    };

                    changed |= value.ui(&mut ui[1], Default::default(), context);
                    *self = Val::Percent(value);
                }
                None => {}
            }
        });
        changed
    }
}
