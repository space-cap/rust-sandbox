use bevy::prelude::*;
use crate::components::{AnimationIndices, AnimationTimer};

/// 델타 타임 기반으로 스프라이트 아틀라스 프레임을 순환 전환하는 시스템
pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                if atlas.index < indices.first || atlas.index >= indices.last {
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

