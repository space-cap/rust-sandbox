use bevy::prelude::*;
use crate::components::{Collider2d, Enemy, GameState, LaserBullet, Player};
use std::collections::HashSet;

/// AABB 기반 다대다 충돌 감지 및 엔티티 안전 소멸 시스템
pub fn collision_detection_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    bullet_query: Query<(Entity, &Transform, &Collider2d), With<LaserBullet>>,
    enemy_query: Query<(Entity, &Transform, &Collider2d), With<Enemy>>,
    player_query: Query<(&Transform, &Collider2d), With<Player>>,
) {
    if game_state.is_game_over {
        return;
    }

    // 동일 프레임 내 중복 despawn 방지용 집합
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // 1. 총알(Bullet) vs 적기(Enemy) AABB 충돌 검사
    for (bullet_e, bullet_tf, bullet_col) in bullet_query.iter() {
        if despawned_entities.contains(&bullet_e) {
            continue;
        }

        let b_pos = bullet_tf.translation.truncate();

        for (enemy_e, enemy_tf, enemy_col) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_e) {
                continue;
            }

            let e_pos = enemy_tf.translation.truncate();

            // AABB 교차 여부 판정
            if bullet_col.intersects(b_pos, enemy_col, e_pos) {
                despawned_entities.insert(bullet_e);
                despawned_entities.insert(enemy_e);

                commands.entity(bullet_e).despawn();
                commands.entity(enemy_e).despawn();

                game_state.score += 100;
                game_state.enemies_killed += 1;
                break;
            }
        }
    }

    // 2. 적기(Enemy) vs 플레이어(Player) 충돌 검사
    if let Ok((player_tf, player_col)) = player_query.single() {
        let p_pos = player_tf.translation.truncate();

        for (enemy_e, enemy_tf, enemy_col) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_e) {
                continue;
            }

            let e_pos = enemy_tf.translation.truncate();

            if player_col.intersects(p_pos, enemy_col, e_pos) {
                despawned_entities.insert(enemy_e);
                commands.entity(enemy_e).despawn();

                game_state.shields -= 1;
                if game_state.shields <= 0 {
                    game_state.shields = 0;
                    game_state.is_game_over = true;
                }
                break;
            }
        }
    }
}

